use i2p::config::Config;
use i2p::data::netdb::NetDB;
use i2p::error::Error;
use i2p::http::http_server::HTTPServer;
use i2p::logging;
use i2p::router_context::RouterContext;
use i2p::transport::transports::Transports;
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
    netdb: NetDB,
    transports: Transports,
    http_server: Option<HTTPServer>,
}

impl Daemon {
    pub fn start(&self) -> Result<(), Error> {
        info!("Daemon: Starting NetDB");
        if let Err(error) = self.netdb.start() {
            error!("Error starting NetDB: {}", error);
            self.netdb.stop();
        }

        let use_ntcp = self.config.get_bool_value("ntcp", true)?;
        let use_ssu = self.config.get_bool_value("ssu", true)?;

        info!("Daemon: starting Transports");
        self.transports.start(use_ntcp, use_ssu)?;
        if self.transports.is_running() {
            info!("Daemon: Transports started");
        } else {
            error!("Daemon: Failed to start transports");
            self.stop();
            return Err(Error::Transport("Transports failed to start".to_string()));
        }

        if let Some(ref server) = self.http_server {
            server.start();
        }

        Ok(())
    }

    pub fn stop(&self) {
        if let Some(ref server) = self.http_server {
            server.stop();
        }
        self.transports.stop();
        self.netdb.stop();
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
        self.netdb = NetDB::new(&self.config, &self.data_dir)?;
        self.transports = Transports::new();

        let http = self.config.get_bool_value("http.enabled", true)?;
        if http {
            let http_address = self.config.get_value_with_default("http.address", "127.0.0.1");
            let http_port = self.config.get_int_value("http.port", 7070)?;
            info!("Daemon starting HTTP server at {}:{}",
                  http_address,
                  http_port);
            self.http_server = Some(HTTPServer::new(&http_address, http_port)?);
        }

        Ok(())
    }

    pub fn new() -> Result<Daemon, Error> {
        let mut daemon: Daemon = Default::default();
        daemon.initialize()?;

        Ok(daemon)
    }
}