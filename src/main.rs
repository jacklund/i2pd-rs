#![feature(box_syntax)]
#![feature(plugin, custom_derive)]
#![plugin(mockers_macros)]

#[cfg(test)]
extern crate base64;
extern crate byteorder;
#[macro_use]
extern crate clap;
extern crate ini;
#[macro_use]
extern crate log;
extern crate log4rs;
#[cfg(test)]
extern crate mockers;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate tempdir;
extern crate time;
extern crate vec_map;
extern crate walkdir;
extern crate yaml_rust;

mod i2p;

use i2p::fs::config_dir;
use i2p::daemon::Daemon;
use i2p::logging;
use std::error::Error;

fn main() {
    let config_dir = match config_dir() {
        Ok(config_dir) => config_dir,
        Err(error) => panic!(error),
    };

    if let Err(error) = logging::initialize(config_dir) {
        panic!("Error initializing logging: {}", error);
    }

    let daemon: Daemon = match Daemon::new() {
        Err(error) => panic!("Initialization error: {}: {} - {:?}", error, error.description(), error.cause()),
        Ok(daemon) => daemon,
    };

    match daemon.start() {
        Ok(_) => daemon.run(),
        Err(error) => {
            daemon.stop();
            error!("Error in daemon: {}", error);
        }
    }
}
