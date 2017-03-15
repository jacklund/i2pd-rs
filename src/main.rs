#![feature(box_syntax)]
#![feature(plugin, custom_derive)]
#![plugin(mockers_macros)]

extern crate bincode;
#[macro_use]
extern crate clap;
extern crate ini;
#[macro_use]
extern crate log;
extern crate log4rs;
#[cfg(test)]
extern crate mockers;
extern crate rustc_serialize;
extern crate tempdir;
extern crate time;
extern crate vec_map;
extern crate walkdir;
extern crate yaml_rust;

mod i2p;

use i2p::daemon::Daemon;

fn main() {
    let daemon: Daemon = match Daemon::new() {
        Err(error) => panic!("Initialization error: {}", error),
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
