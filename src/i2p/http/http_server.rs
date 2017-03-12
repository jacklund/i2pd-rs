use i2p::error::Error;

#[derive(Debug, Default)]
pub struct HTTPServer {
}

impl HTTPServer {
    pub fn new(address: &str, port: u32) -> Result<HTTPServer, Error> {
        Ok(HTTPServer{})
    }

    pub fn start(&self) {
    }

    pub fn stop(&self) {
    }
}