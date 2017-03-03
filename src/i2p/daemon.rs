use i2p::config::get_config;
use i2p::error::Error;

#[derive(Debug)]
pub struct Daemon {
}

impl Daemon {
    pub fn start(&self) -> Option<Error> {
        None
    }

    pub fn stop(&self) -> Option<Error> {
        None
    }

    pub fn run(&self) {}
}

pub fn new() -> Result<Daemon, Error> {
    let i2p_config = get_config();
    println!("{:?}", i2p_config);

    Ok(Daemon {})
}