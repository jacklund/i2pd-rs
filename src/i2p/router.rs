use i2p::config::Config;
use i2p::error::{Error, ParseError};
use i2p::event_log::EventLog;
use i2p::router_context::RouterContext;
use libc;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

pub struct Router {
    router_context: RouterContext,
    event_log: EventLog,
}

fn is_another_router_running(pid_dir: &PathBuf) -> Result<bool, Error> {
    let pid_from_file = read_pid_file(pid_dir)?;
    if let Some(old_pid) = pid_from_file {
        return Ok(is_process_running(old_pid));
    }

    Ok(false)
}

fn is_process_running(pid: u32) -> bool {
    unsafe {
        let ret = libc::kill(pid as i32, 0);
        return ret == libc::ESRCH;
    }
}

fn pid_filename(pid_dir: &PathBuf) -> PathBuf {
    let mut dir = PathBuf::from(pid_dir);
    dir.push("i2p.pid");
    dir
}

fn read_pid_file(pid_dir: &PathBuf) -> Result<Option<u32>, Error> {
    let pid_filename = pid_filename(pid_dir);
    let mut pid_file = match OpenOptions::new().read(true).open(&pid_filename) {
        Ok(file) => file,
        Err(error) => {
            return Err(Error::IO {
                message: Some(format!("Error opening PID file {:?}", pid_filename)),
                error: error,
            })
        }
    };

    let mut pid_string = String::new();
    if let Err(error) = pid_file.read_to_string(&mut pid_string) {
        return Err(Error::IO {
            message: Some(format!("Error reading from PID file {:?}", pid_filename)),
            error: error,
        });
    }

    match pid_string.parse::<u32>() {
        Ok(pid) => Ok(Some(pid)),
        Err(error) => {
            Err(Error::ParseError {
                value: pid_string,
                error: ParseError::Int(error),
            })
        }
    }
}

fn write_pid_file() {
    unimplemented!()
}

fn get_process_id() -> u32 {
    unimplemented!()
}

impl Router {
    pub fn new(config: &Config) -> Result<Router, Error> {
        let context = RouterContext::new(config)?;

        if is_another_router_running(&context.pid_dir)? {
            error!("Another router is running! Exiting.");
            exit(1);
        }

        write_pid_file();

        Ok(Router {
            router_context: context,
            event_log: EventLog::new(config),
        })
    }
}