use i2p::config::Config;
use i2p::error::Error;
use log4rs;
use std::path::{Path, PathBuf};

pub fn configure(config: &Config, data_dir: &PathBuf) -> Result<(), Error> {
    let config_file = config.get_as_path("log4rs_config");
    match config_file {
        Some(file) => {
            if file.as_path().is_file() {
                Ok(log4rs::init_file(file, Default::default())?)
            } else {
                Err(Error::Configuration(format!("Bad logging config file {}",
                                                 file.to_str().unwrap())))
            }
        }
        None => {
            let mut path_buf: PathBuf = PathBuf::from(data_dir);
            path_buf.push("log4rs.yml");
            if path_buf.as_path().is_file() {
                Ok(log4rs::init_file(path_buf, Default::default())?)
            } else if Path::new("log4rs.yml").is_file() {
                Ok(log4rs::init_file("log4rs.yml", Default::default())?)
            } else {
                Err(Error::Configuration(format!("No logging configuration file provided")))
            }
        }
    }
}