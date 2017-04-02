use i2p::config::Config;
use i2p::error::Error;
use i2p::event_log::EventLog;
use i2p::router_context::RouterContext;

pub struct Router {
    router_context: RouterContext,
    event_log: EventLog,
}

impl Router {
    pub fn new(config: &Config) -> Result<Router, Error> {
        Ok(Router {
            router_context: RouterContext::new(config)?,
            event_log: EventLog::new(config),
        })
    }
}