use i2p::config::Config;
use i2p::error::Error;
use std::env;
use std::path::PathBuf;

pub struct RouterContext {
    config_dir: PathBuf,
}

impl RouterContext {
    pub fn new(config: &Config) -> Result<RouterContext, Error> {
        let cwd = match env::current_dir() {
            Ok(dir) => dir,
            Err(error) => return Err(Error::Configuration(format!("Error finding current directory: {}", error))),
        };
        let config_dir = config.path_value("i2p.config.dir", Some(cwd)).unwrap();
        Ok(RouterContext {
            config_dir: config_dir,
        })
    }
}