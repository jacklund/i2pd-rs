use i2p::config::Config;
use i2p::data::router_info::RouterInfo;
use i2p::error::Error;
use i2p::fs::hashed_storage::HashedStorage;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct NetDB {
    router_info_store: HashedStorage,
}

impl NetDB {
    pub fn new(config: &Config, data_dir: &PathBuf) -> Result<NetDB, Error> {
        let netdb: NetDB = NetDB {
            router_info_store: HashedStorage::new(data_dir, "i2pd-rs", "routerinfo", false)?,
        };

        Ok(netdb)
    }

    pub fn start(&self) -> Result<(), Error> {
        Ok(())
    }

    pub fn stop(&self) {
    }
}