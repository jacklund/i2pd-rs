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

pub struct Daemon {
    config: Config,
    data_dir: PathBuf,
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

        let use_ntcp = self.config.bool_value("ntcp.enable", Some(true)).unwrap();
        let use_ssu = self.config.bool_value("ssu.enable", Some(true)).unwrap();

        info!("Daemon: starting Transports");
        self.transports.start(use_ntcp, use_ssu)?;
        if self.transports.is_running() {
            info!("Daemon: Transports started");
        } else {
            error!("Daemon: Failed to start transports");
            self.stop();
            return Err(Error::Transport(format!("Transports failed to start")));
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
            data_dir.push("Library/Application Support/i2pd");
        } else if cfg!(target_os = "unix") {
            match env::home_dir() {
                Some(home_dir) => data_dir.push(home_dir),
                None => data_dir.push("/tmp"),
            }
            data_dir.push("i2pd");
        }

        Ok(data_dir)
    }

    fn get_data_dir(config: &Config) -> Result<PathBuf, Error> {
        match config.path_value("datadir", None) {
            Some(dir) => Ok(dir),
            None => Daemon::find_data_dir(),
        }
    }

    pub fn new(config: Config) -> Result<Daemon, Error> {
        let data_dir = Self::get_data_dir(&config)?;
        info!("Creating Router Context");
        let router_context = RouterContext::new(&config)?;
        info!("Creating NetDB");
        let netdb = NetDB::new(&config, &data_dir)?;
        info!("Creating Transports");
        let transports = Transports::new();

        info!("Creating HTTP");
        let http = config.bool_value("http.enabled", Some(true)).unwrap();
        let mut http_server = None;
        if http {
            let http_address = config.string_value("http.address", Some("127.0.0.1")).unwrap();
            let http_port = config.i64_value("http.port", Some(7070)).unwrap();
            info!("Daemon starting HTTP server at {}:{}",
                  http_address,
                  http_port);
            http_server = Some(HTTPServer::new(&http_address, http_port as u32)?);
        }

        Ok(Daemon {
            config: config,
            data_dir: data_dir,
            router_context: router_context,
            netdb: netdb,
            transports: transports,
            http_server: http_server,
        })
    }
}