use clap::{Arg, ArgMatches, App};
use i2p::error::Error;
use serde::Deserialize;
use serde_yaml;
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
        Ok(config) => Ok(config),
        Err(error) => {
            Err(Error::Configuration(format!("Error reading configuration file {:?}: {}",
                                             config_file.path,
                                             error)))
        }
    }
}

fn merge_configs(args: ArgMatches,
                 config_file: Config,
                 config_dir: PathBuf,
                 working_dir: PathBuf)
                 -> Config {
    Config::convert(args).map.iter().fold(config_file, |mut config, (key, value)| {
        config.insert(key, value);
        config
    })
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    map: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        let cmd_line = App::new("i2pd")
            .about("I2P Daemon")
            .author(crate_authors!())
            .version(crate_version!())
            .arg(Arg::with_name("datadir")
                .long("datadir")
                .value_name("DIR")
                .help("data directory location")
                .takes_value(true))
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
            .arg(Arg::with_name("daemon")
                .long("daemon")
                .help("run in background"))
            .get_matches();

        let config_dir = get_config_dir(&cmd_line)?;
        let config_file = get_config_file(&cmd_line, &config_dir)?;
        let working_dir = get_working_dir(&cmd_line)?;
        Ok(merge_configs(cmd_line,
                         parse_config_file(config_file)?,
                         config_dir,
                         working_dir))
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
    }

    pub fn get_bool_value(&self, name: &str, default: bool) -> Result<bool, Error> {
        match self.get_value(name) {
            Some(value) => Ok(bool::from_str(value)?),
            None => Ok(default),
        }
    }

    pub fn get_int_value(&self, name: &str, default: u32) -> Result<u32, Error> {
        match self.get_value(name) {
            Some(value) => Ok(value.parse::<u32>()?),
            None => Ok(default),
        }
    }

    pub fn get_value(&self, name: &str) -> Option<&String> {
        self.map.get(name)
    }

    pub fn get_value_with_default(&self, name: &str, default: &str) -> String {
        match self.get_value(name) {
            Some(value) => value.to_string(),
            None => default.to_string(),
        }
    }

    pub fn get_as_path(&self, name: &str) -> Option<PathBuf> {
        let value = self.get_value(name);
        println!("value = {:?}", value);
        match value {
            Some(val) => {
                let mut buf = PathBuf::new();
                buf.push(val.to_string());
                Some(buf)
            }
            None => None,
        }
    }
}

trait Convert<T> {
    fn convert(T) -> Self;
}

impl<'a> Convert<ArgMatches<'a>> for Config {
    fn convert(matches: ArgMatches) -> Config {
        Config {
            map: matches.args
                .iter()
                .map(|(k, v)| {
                    (k.to_string(),
                     match v.vals.get(0) {
                         Some(val) => val.to_str().unwrap().to_string(),
                         None => "true".to_string(),
                     })
                })
                .collect(),
        }
    }
}