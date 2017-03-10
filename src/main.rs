#![feature(box_syntax)]

#[macro_use]
extern crate clap;
extern crate ini;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate vec_map;
extern crate yaml_rust;

mod i2p;

use i2p::daemon::Daemon;

fn main() {
    let daemon: Daemon = match Daemon::new() {
        Err(error) => panic!("Initialization error: {}", error),
        Ok(daemon) => daemon,
    };

    if let Some(error) = daemon.start() {
        daemon.stop();
        println!("{:?}", error);
    } else {
        daemon.run();
    }
}
