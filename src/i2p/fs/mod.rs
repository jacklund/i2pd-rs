pub mod hashed_storage;

use i2p::error::Error;
use std::env;
use std::path::PathBuf;

pub fn config_dir() -> Result<PathBuf, Error> {
    if cfg!(target_os = "unix") {
        if let Some(mut pathbuf) = env::home_dir() {
            pathbuf.push(".i2pd-rs");
            if pathbuf.is_dir() {
                return Ok(pathbuf);
            }
        };
        let pathbuf = PathBuf::from("/etc/i2pd-rs");
        if pathbuf.is_dir() {
            return Ok(pathbuf);
        }
    } else if cfg!(target_os = "macos") {
        if let Some(mut pathbuf) = env::home_dir() {
            pathbuf.push("Library/Application Support/i2pd-rs");
            if pathbuf.is_dir() {
                return Ok(pathbuf);
            }
        };
        let pathbuf = PathBuf::from("/Library/Application Support/i2pd-rs");
        if pathbuf.is_dir() {
            return Ok(pathbuf);
        }

        return Ok(PathBuf::from("."));
    }

    Err(Error::Configuration(format!("Configuration directory not found")))
}