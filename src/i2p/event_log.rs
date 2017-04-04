use i2p::config::Config;

#[derive(Debug)]
pub struct EventLog {
}

impl EventLog {
    pub fn new(config: &Config) -> EventLog {
        EventLog {}
    }
}