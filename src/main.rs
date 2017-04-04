#![feature(box_syntax)]
#![feature(plugin, custom_derive)]

#[cfg(test)]
extern crate base64;
extern crate byteorder;
#[macro_use]
extern crate clap;
extern crate libc;
extern crate linked_hash_map;
#[macro_use]
extern crate log;
extern crate log4rs;
#[cfg(test)]
extern crate mockers;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
#[cfg(test)]
extern crate tempdir;
extern crate time;
extern crate vec_map;
extern crate walkdir;
extern crate yaml_rust;

mod i2p;

use i2p::config::Config;
use i2p::logging;
use i2p::router::Router;
use std::error::Error;
use std::process::exit;

fn main() {
    let config: Config = match Config::new() {
        Ok(config) => config,
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    };

    let config_dir = match config.path_value("i2p.dir.config", None) {
        Some(dir) => dir,
        None => panic!("No config dir configured"),
    };

    if let Err(error) = logging::initialize(&config_dir) {
        panic!("Error initializing logging: {}", error);
    }
    
    info!("{:?}", Router::new(&config));
}
