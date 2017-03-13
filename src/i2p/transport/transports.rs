use i2p::error::Error;

#[derive(Debug, Default)]
pub struct Transports {
    is_online: bool,
    is_running: bool,
}

impl Transports {
    pub fn new() -> Transports {
        Transports {
            is_online: true,
            is_running: false,
        }
    }

    pub fn start(&self, use_ntcp: bool, use_ssu: bool) -> Result<(), Error> {
        if !use_ssu {
            info!("Transports: ssu disabled");
        }

        if !use_ntcp {
            info!("Transports: ntcp disabled");
        }

        unimplemented!()
    }

    pub fn is_running(&self) -> bool {
        true
    }

    pub fn stop(&self) {
    }
}