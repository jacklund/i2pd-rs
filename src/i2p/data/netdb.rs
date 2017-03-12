use i2p::error::Error;

#[derive(Debug, Default)]
pub struct NetDB {
}

impl NetDB {
    pub fn new() -> Result<NetDB, Error> {
        let netdb: NetDB = Default::default();

        Ok(netdb)
    }

    pub fn start(&self) -> Result<(), Error> {
        Ok(())
    }

    pub fn stop(&self) {
    }
}