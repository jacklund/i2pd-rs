use i2p::config::get_config;
use i2p::error::Error;

#[derive(Debug)]
pub struct Daemon {
}

impl Daemon {
    fn start(&self) -> Option<Error> {
        None
    }

    fn stop(&self) -> Option<Error> {
        None
    }

    fn run() {
    }
}

pub fn new() -> Result<Daemon, Error> {
    let i2p_config = get_config();
    println!("{:?}", i2p_config);

    Ok(Daemon {})
}