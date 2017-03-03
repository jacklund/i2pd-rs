#![feature(box_syntax)]

#[macro_use]
extern crate clap;
extern crate ini;
extern crate vec_map;
extern crate yaml_rust;

mod i2p;

fn main() {
    let daemon: i2p::daemon::Daemon = match i2p::daemon::new() {
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
