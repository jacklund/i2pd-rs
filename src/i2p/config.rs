use clap::{Arg, ArgMatches, App};
use i2p::error::Error;
use ini::Ini;
use std::collections::HashMap;
use std::convert::From;
use std::env;
use std::path::{PathBuf, Path};
use std::str::FromStr;

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
    if let Some(config_path) = cmd_line.get_value("config") {
        let path = Path::new(&config_path);
        if path.is_file() {
            return Some(path.to_str().unwrap().to_string());
        }
    }
    if let Some(datadir) = cmd_line.get_value("datadir") {
        let mut path_buf = PathBuf::from(datadir);
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

fn merge_configs(config_file: Config, args: Config) -> Config {
    args.map.iter().fold(config_file, |mut config, (key, value)| {
        config.insert(key, value);
        config
    })
}

#[derive(Debug, Default)]
pub struct Config {
    map: HashMap<String, String>,
}

impl Config {
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

impl Convert<Ini> for Config {
    fn convert(ini: Ini) -> Config {
        ini.iter().fold(Default::default(), |mut hashmap, (section, props)| {
            match *section {
                Some(ref name) => {
                    for (key, value) in props {
                        hashmap.insert(format!("{0}.{1}", name, key).as_str(),
                                       value.clone().as_str());
                    }
                }
                None => {
                    for (key, value) in props {
                        hashmap.insert(key.clone().as_str(), value.clone().as_str());
                    }
                }
            };
            hashmap
        })
    }
}

impl<'a> Convert<ArgMatches<'a>> for Config {
    fn convert(matches: ArgMatches) -> Config {
        Config {
            map: matches.args
                .iter()
                .map(|(k, v)| {
                    (k.to_string(),
                     match v.vals.get(1) {
                         Some(val) => val.to_str().unwrap().to_string(),
                         None => "true".to_string(),
                     })
                })
                .collect(),
        }
    }
}