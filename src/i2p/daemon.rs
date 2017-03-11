use i2p::config::Config;
use i2p::error::Error;
use i2p::logging;
use i2p::router_context::RouterContext;
use std::default::Default;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Daemon {
    config: Config,
    data_dir: PathBuf,
    is_daemon: bool,
    router_context: RouterContext,
}

impl Daemon {
    pub fn start(&self) -> Option<Error> {
        None
    }

    pub fn stop(&self) -> Option<Error> {
        None
    }

    pub fn run(&self) {}

    fn find_data_dir() -> Result<PathBuf, Error> {
        let mut data_dir = PathBuf::new();
        if cfg!(target_os = "macos") {
            if let Some(home_dir) = env::home_dir() {
                data_dir.push(home_dir);
            }
            data_dir.push("/Library/Application Support/i2pd");
        } else if cfg!(target_os = "unix") {
            match env::home_dir() {
                Some(home_dir) => data_dir.push(home_dir),
                None => data_dir.push("/tmp"),
            }
            data_dir.push("i2pd");
        }
        Ok(data_dir)
    }

    fn get_data_dir(&self, config: &Config) -> Result<PathBuf, Error> {
        match config.get_value("datadir") {
            Some(dir) => {
                let mut datadir_path = PathBuf::new();
                datadir_path.push(dir);
                if !datadir_path.is_dir() {
                    fs::create_dir_all(datadir_path.as_path())?;
                };
                Ok(datadir_path)
            }
            None => Daemon::find_data_dir(),
        }
    }

    fn initialize(&mut self) -> Result<(), Error> {
        self.config = Config::get_config()?;
        self.data_dir = self.get_data_dir(&self.config)?;
        self.is_daemon = self.config.get_bool_value("daemon", false)?;
        logging::configure(&self.config, &self.data_dir)?;
        self.router_context = RouterContext::new(&self.config)?;

        Ok(())
    }

    pub fn new() -> Result<Daemon, Error> {
        let mut daemon: Daemon = Default::default();
        daemon.initialize()?;

        Ok(daemon)
    }
}