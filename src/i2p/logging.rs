use i2p::error::{Error, LogError};
use log4rs;
use std::path::PathBuf;

pub fn initialize(config_dir: &PathBuf) -> Result<(), Error> {
    let mut config_path = PathBuf::from(config_dir);
    config_path.push("log4rs.yml");
    println!("logging config is {:?}", config_path);
    match log4rs::init_file(config_path.as_path(), Default::default()) {
        Ok(()) => Ok(()),
        Err(error) => {
            Err(Error::Logging(LogError::LogError {
                message: format!("Error opening logging config file {:?}", config_path),
                error: error,
            }))
        }
    }
}