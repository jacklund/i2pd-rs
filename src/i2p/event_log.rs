use i2p::config::Config;

#[derive(Debug)]
pub struct EventLog {
}

impl EventLog {
    pub fn new(config: &Config) -> EventLog {
        EventLog {}
    }

    pub fn add_event(&mut self, event: &str, info: Option<&str>) {
    }
}