use gcrypt;
use i2p::config::Config;
use i2p::data::router_info::RouterInfo;
use i2p::error::{Error, ParseError};
use i2p::event_log::EventLog;
use i2p::router_context::RouterContext;
use libc;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;

const DEFAULT_NETWORK_ID: u32 = 2;
const NETWORK_ID_CONFIG: &str = "router.networkID";

#[derive(Debug)]
pub struct Router {
    router_context: RouterContext,
    event_log: EventLog,
    network_id: u32,
    state: RouterState,
    config: Config,
    token: gcrypt::Token,
    router_info: Option<RouterInfo>,
}

#[derive(Debug)]
enum RouterState {
    /** constructor complete */
    INITIALIZED,
    /** runRouter() called */
    STARTING_1,
    /** startupStuff() complete, most of the time here is NTP */
    STARTING_2,
    /** NTP done, Job queue started, StartupJob queued, runRouter() returned */
    STARTING_3,
    /** RIs loaded. From STARTING_3 */
    NETDB_READY,
    /** Non-zero-hop expl. tunnels built. From STARTING_3 */
    EXPL_TUNNELS_READY,
    /** from NETDB_READY or EXPL_TUNNELS_READY */
    RUNNING,
    /**
        *  A "soft" restart, primarily of the comm system, after
        *  a port change or large step-change in system time.
        *  Does not stop the whole JVM, so it is safe even in the absence
        *  of the wrapper.
        *  This is not a graceful restart - all peer connections are dropped immediately.
        */
    RESTARTING,
    /** cancellable shutdown has begun */
    GRACEFUL_SHUTDOWN,
    /** In shutdown(). Non-cancellable shutdown has begun */
    FINAL_SHUTDOWN_1,
    /** In shutdown2(). Killing everything */
    FINAL_SHUTDOWN_2,
    /** In finalShutdown(). Final cleanup */
    FINAL_SHUTDOWN_3,
    /** all done */
    STOPPED
}

fn is_another_router_running(pid_dir: &PathBuf) -> Result<bool, Error> {
    let pid_from_file = read_pid_file(pid_dir)?;
    info!("PID of router process from pid file is {:?}", pid_from_file);
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
    info!("Reading from pid file {:?}", pid_filename);
    if !pid_filename.exists() {
        info!("PID file {:?} not found", pid_filename);
        return Ok(None);
    }

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

fn write_pid_file(pid_dir: &PathBuf) -> Result<(), Error> {
    let pid_filename = pid_filename(pid_dir);
    info!("Writing pid file {:?}", pid_filename);
    let mut pid_file = match OpenOptions::new().write(true).create(true).open(&pid_filename) {
        Ok(file) => file,
        Err(error) => {
            return Err(Error::IO {
                message: Some(format!("Error opening PID file {:?}", pid_filename)),
                error: error,
            })
        }
    };
    match pid_file.write(get_process_id().to_string().as_bytes()) {
        Ok(_) => Ok(()),
        Err(error) => Err(Error::IO { message: Some(format!("Error writing pid file")), error: error })
    }
}

fn get_process_id() -> u32 {
    unsafe {
        libc::getpid() as u32
    }
}

fn init_gcrypt() -> gcrypt::Token {
    gcrypt::init(|mut x| {
        x.disable_secmem().enable_quick_random();
    })
}

impl Router {
    pub fn new(config: Config) -> Result<Router, Error> {
        let context = RouterContext::new(&config)?;

        info!("Checking to see if another router is running");
        if is_another_router_running(&context.pid_dir)? {
            error!("Another router is running! Exiting.");
            exit(1);
        }

        write_pid_file(&context.pid_dir);

        let network_id = config.i64_value(NETWORK_ID_CONFIG, Some(DEFAULT_NETWORK_ID as i64)).unwrap() as u32;

        Ok(Router {
            router_context: context,
            event_log: EventLog::new(&config),
            network_id: network_id,
            state: RouterState::INITIALIZED,
            config: config,
            token: init_gcrypt(),
            router_info: None,
        })
    }

    pub fn run(&mut self) {
        self.state = RouterState::STARTING_1;
        self.event_log.add_event("started", None);
    }
}