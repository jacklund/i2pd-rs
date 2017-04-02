use clap::{Arg, ArgMatches, App};
use i2p::error::Error;
use linked_hash_map::LinkedHashMap;
use serde::Deserialize;
use serde_yaml::{self, Mapping};
use std::collections::HashMap;
use std::convert::From;
use std::env;
use std::fs::{create_dir_all, File};
use std::path::{PathBuf, Path};
use std::str::FromStr;

struct ConfigFile {
    path: PathBuf,
    file: File,
}

fn get_config_dir(command_line: &ArgMatches) -> Result<PathBuf, Error> {
    match command_line.value_of("config-dir") {
        Some(name) => {
            let dir = PathBuf::from(name);
            if dir.is_dir() {
                Ok(dir)
            } else {
                Err(Error::Configuration(format!("Configuration directory '{}' is not a \
                                                  directory",
                                                 name)))
            }
        }
        None => get_default_config_dir(&command_line),
    }
}

fn get_default_config_dir(command_line: &ArgMatches) -> Result<PathBuf, Error> {
    let config_dir_opt: Option<PathBuf> = if cfg!(target_os = "unix") {
        if command_line.is_present("daemon") {
            Some(PathBuf::from("/etc/i2pd"))
        } else {
            if let Some(mut dir) = env::home_dir() {
                dir.push(".i2pd");
                Some(dir)
            } else {
                None
            }
        }
    } else if cfg!(target_os = "macos") {
        if let Some(mut dir) = env::home_dir() {
            dir.push("Library/Application Support/i2pd");
            Some(dir)
        } else {
            None
        }
    } else {
        None
    };

    if config_dir_opt.is_some() {
        let config_dir = config_dir_opt.unwrap();
        if config_dir.is_dir() {
            return Ok(config_dir);
        }
    }

    Err(Error::Configuration(format!("Couldn't find configuration dir")))
}

fn get_config_file(command_line: &ArgMatches, config_dir: &PathBuf) -> Result<ConfigFile, Error> {
    let config_file = match command_line.value_of("config") {
        Some(filename) => PathBuf::from(filename),
        None => {
            let mut pathbuf = PathBuf::from(config_dir);
            pathbuf.push("config.yml");
            pathbuf
        }
    };

    match File::open(&config_file) {
        Ok(file) => {
            Ok(ConfigFile {
                path: config_file,
                file: file,
            })
        }
        Err(error) => {
            Err(Error::IO {
                message: Some(format!("Error opening config file {}",
                                      config_file.to_str().unwrap())),
                error: error,
            })
        }
    }
}

fn get_working_dir(command_line: &ArgMatches) -> Result<PathBuf, Error> {
    if let Some(dirname) = command_line.value_of("working-dir") {
        let pathbuf = PathBuf::from(dirname);
        if pathbuf.is_dir() {
            return Ok(pathbuf);
        } else {
            return Err(Error::Configuration(format!("Working dir '{}' doesn't exist", dirname)));
        }
    }

    match env::home_dir() {
        Some(home) => {
            let mut pathbuf = PathBuf::from(home);
            if cfg!(target_os = "macos") {
                pathbuf.push("Library");
                pathbuf.push("Application Support");
                pathbuf.push("i2p");
            } else if cfg!(target_os = "unix") {
                pathbuf.push(".i2p");
            }
            create_dir_all(&pathbuf)?;
            return Ok(pathbuf);
        }
        None => return Err(Error::Configuration(format!("Couldn't find home directory"))),
    }
}

fn parse_config_file(config_file: ConfigFile) -> Result<Config, Error> {
    match serde_yaml::from_reader(&config_file.file) {
        Ok(config) => {
            match config {
                serde_yaml::Value::Mapping(mapping) => Ok(Config { values: mapping }),
                _ => {
                    Err(Error::Configuration(format!("Unable to read configuration file {:?} as \
                                                      map",
                                                     config_file.path)))
                }
            }
        }
        Err(error) => {
            Err(Error::Configuration(format!("Error reading configuration file {:?}: {}",
                                             config_file.path,
                                             error)))
        }
    }
}

fn merge_configs(args: ArgMatches, mut config: Config) -> Config {
    if args.is_present("config-dir") {
        config.insert_string("i2p.dir.config", args.value_of("config-dir").unwrap());
    }

    config
}

#[derive(Debug)]
pub struct Config {
    values: Mapping,
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        let cmd_line = App::new("i2pd")
            .about("I2P Daemon")
            .author(crate_authors!())
            .version(crate_version!())
            .arg(Arg::with_name("config")
                .short("C")
                .long("config")
                .value_name("FILE")
                .help("config file location")
                .takes_value(true))
            .arg(Arg::with_name("config-dir")
                .long("config-dir")
                .value_name("DIR")
                .help("config directory location")
                .takes_value(true))
            .arg(Arg::with_name("working-dir")
                .long("working-dir")
                .value_name("DIR")
                .help("working directory location")
                .takes_value(true))
            .get_matches();

        let config_dir = get_config_dir(&cmd_line)?;
        let config_file = get_config_file(&cmd_line, &config_dir)?;
        let working_dir = get_working_dir(&cmd_line)?;
        Ok(merge_configs(cmd_line, parse_config_file(config_file)?))
    }

    fn find(&self, path: &str) -> Option<&serde_yaml::Value> {
        let mut current = &self.values;
        let mut path_elements: Vec<&str> = path.split('.').collect::<Vec<&str>>();
        path_elements.reverse();
        while !path_elements.is_empty() {
            let element = path_elements.pop().unwrap();
            let temp = match current.get(&serde_yaml::Value::String(element.to_string())) {
                None => return None,
                Some(node) => {
                    if path_elements.is_empty() {
                        return Some(node);
                    } else {
                        match *node {
                            serde_yaml::Value::Mapping(ref mapping) => mapping,
                            _ => return None,
                        }
                    }
                }
            };
            current = temp;
        }

        None
    }

    pub fn bool_value(&self, path: &str, default: Option<bool>) -> Option<bool> {
        match self.find(path) {
            Some(value) => {
                match *value {
                    serde_yaml::Value::Bool(b) => Some(b),
                    _ => None,
                }
            }
            None => default,
        }
    }

    pub fn string_value(&self, path: &str, default: Option<&str>) -> Option<String> {
        match self.find(path) {
            Some(value) => {
                match *value {
                    serde_yaml::Value::String(ref string) => Some(string.clone()),
                    _ => None,
                }
            }
            None => default.map(|d| d.to_string()),
        }
    }

    pub fn path_value(&self, path: &str, default: Option<PathBuf>) -> Option<PathBuf> {
        match self.string_value(path, None) {
            Some(path) => Some(PathBuf::from(path)),
            None => default,
        }
    }

    pub fn i64_value(&self, path: &str, default: Option<i64>) -> Option<i64> {
        match self.find(path) {
            Some(value) => {
                match *value {
                    serde_yaml::Value::I64(i) => Some(i),
                    _ => None,
                }
            }
            None => default,
        }
    }

    fn insert_string(&mut self, path: &str, value: &str) {
        unimplemented!()
    }
}