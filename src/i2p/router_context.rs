use i2p::config::Config;
use i2p::error::Error;
use std::env;
use std::path::PathBuf;

pub struct RouterContext {
    pub config_dir: PathBuf,
    pub router_dir: PathBuf,
    pub pid_dir: PathBuf,
    pub log_dir: PathBuf,
    pub app_dir: PathBuf,
}

fn make_dir(dir: &PathBuf) {
    // TODO: Implement
}

impl RouterContext {
    pub fn new(config: &Config) -> Result<RouterContext, Error> {
        let cwd = match env::current_dir() {
            Ok(dir) => dir,
            Err(error) => {
                return Err(Error::Configuration(format!("Error finding current directory: {}",
                                                        error)))
            }
        };
        let config_dir = config.path_value("i2p.dir.config", Some(&cwd)).unwrap();
        make_dir(&config_dir);

        let router_dir = config.path_value("i2p.dir.router", Some(&config_dir)).unwrap();
        make_dir(&router_dir);

        let pid_dir = config.path_value("i2p.dir.pid", Some(&router_dir)).unwrap();
        make_dir(&pid_dir);

        let log_dir = config.path_value("i2p.dir.log", Some(&router_dir)).unwrap();
        make_dir(&log_dir);

        let app_dir = config.path_value("i2p.dir.app", Some(&router_dir)).unwrap();
        make_dir(&app_dir);

        Ok(RouterContext {
            config_dir: config_dir,
            router_dir: router_dir,
            pid_dir: pid_dir,
            log_dir: log_dir,
            app_dir: app_dir,
        })
    }
}