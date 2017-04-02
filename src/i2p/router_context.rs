use i2p::config::Config;
use i2p::error::Error;
use std::env;
use std::path::PathBuf;

pub struct RouterContext {
    config_dir: PathBuf,
    router_dir: PathBuf,
}

fn make_dir(dir: &PathBuf) {
    // TODO: Implement
}

impl RouterContext {
    pub fn new(config: &Config) -> Result<RouterContext, Error> {
        let cwd = match env::current_dir() {
            Ok(dir) => dir,
            Err(error) => return Err(Error::Configuration(format!("Error finding current directory: {}", error))),
        };
        let config_dir = config.path_value("i2p.config.dir", Some(&cwd)).unwrap();
        make_dir(&config_dir);

        let router_dir = config.path_value("i2p.router.dir", Some(&config_dir)).unwrap();
        make_dir(&router_dir);

        Ok(RouterContext {
            config_dir: config_dir,
            router_dir: router_dir,
        })
    }
}