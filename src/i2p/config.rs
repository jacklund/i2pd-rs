use clap::{Arg, ArgMatches, App};
use i2p::error::Error;
use ini::Ini;
use std::collections::HashMap;
use std::convert::From;
use std::env;
use std::str::FromStr;
use std::path::{PathBuf, Path};
use yaml_rust::yaml::Yaml;

fn get_home_dir_config_path() -> Option<String> {
    match env::home_dir() {
        Some(mut pathbuf) => {
            pathbuf.push(".i2pd");
            pathbuf.set_file_name("i2pd.conf");
            let path = pathbuf.as_path();
            if path.is_file() {
                Some(path.to_str().unwrap().to_string())
            } else {
                None
            }
        }
        None => None,
    }
}

fn get_main_config_path() -> Option<String> {
    let path = Path::new("/var/lib/i2pd/i2pd.conf");
    if path.is_file() {
        Some(path.to_str().unwrap().to_string())
    } else {
        None
    }
}

fn get_config_path(cmd_line: &Config) -> Option<String> {
    if cmd_line.contains_key("config") {
        let config_path = cmd_line.get("config").unwrap().to_string();
        let path = Path::new(config_path.as_str());
        if path.is_file() {
            return Some(path.to_str().unwrap().to_string());
        }
    }
    if cmd_line.contains_key("datadir") {
        let mut path_buf = PathBuf::from(cmd_line.get("datadir").unwrap().to_string().as_str());
        path_buf.set_file_name("i2pd.conf");
        let path = path_buf.as_path();
        if path.is_file() {
            return Some(path.to_str().unwrap().to_string());
        }
    }
    if cfg!(unix) {
        if let Some(path) = get_home_dir_config_path() {
            return Some(path);
        }
        if let Some(path) = get_main_config_path() {
            return Some(path);
        }
    }
    None
}

#[derive(Clone, Debug)]
pub enum ConfigEntry {
    Bool(bool),
    String(String),
}

impl ToString for ConfigEntry {
    fn to_string(&self) -> String {
        match *self {
            ConfigEntry::Bool(ref b) => b.to_string(),
            ConfigEntry::String(ref s) => s.clone(),
        }
    }
}

pub type Config = HashMap<String, ConfigEntry>;

trait Convert<T> {
    fn convert(T) -> Self;
}

impl Convert<Ini> for Config {
    fn convert(ini: Ini) -> Config {
        ini.iter().fold(HashMap::new(), |mut hashmap, (section, props)| {
            match *section {
                Some(ref name) => {
                    for (key, value) in props {
                        hashmap.insert(format!("{0}.{1}", name, key),
                                       match bool::from_str(value) {
                                           Ok(b) => ConfigEntry::Bool(b),
                                           Err(_) => ConfigEntry::String(value.clone()),
                                       });
                    }
                }
                None => {
                    for (key, value) in props {
                        hashmap.insert(key.clone(),
                                       match bool::from_str(value) {
                                           Ok(b) => ConfigEntry::Bool(b),
                                           Err(_) => ConfigEntry::String(value.clone()),
                                       });
                    }
                }
            };
            hashmap
        })
    }
}

impl<'a> Convert<ArgMatches<'a>> for Config {
    fn convert(matches: ArgMatches) -> Config {
        matches.args
            .iter()
            .map(|(k, v)| {
                (k.to_string(),
                 match v.vals.get(1) {
                     Some(val) => ConfigEntry::String(val.clone().into_string().unwrap()),
                     None => ConfigEntry::Bool(true),
                 })
            })
            .collect()
    }
}

fn merge_configs(config_file: Config, args: Config) -> Config {
    args.iter().fold(config_file.clone(), |mut config, (key, value)| {
        config.insert(key.clone(), value.clone());
        config
    })
}

pub fn get_config() -> Result<Config, Error> {
    let cmd_line = Config::convert(App::new("i2pd")
        .about("I2P Daemon")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("datadir")
            .long("datadir")
            .value_name("DIR")
            .help("data directory location")
            .takes_value(true))
        .arg(Arg::with_name("config")
            .long("config")
            .value_name("FILE")
            .help("config file location")
            .takes_value(true))
        .arg(Arg::with_name("daemon")
            .long("daemon")
            .help("run in background"))
        .get_matches());

    let config = match get_config_path(&cmd_line) {
        Some(path) => {
            let ini = Ini::load_from_file(&path)?;
            merge_configs(Config::convert(ini), cmd_line)
        }
        None => cmd_line,
    };

    Ok(config)
}