use ini::ini;
use log4rs;
use std::error;
use std::fmt;
use std::io;
use std::str;

#[derive(Debug)]
pub enum Error {
    ConfigFile(ini::Error),
    IO(io::Error),
    ParseConfig(str::ParseBoolError),
    Configuration(String),
    LogConfig(log4rs::Error),
}

impl From<ini::Error> for Error  {
    fn from(error: ini::Error) -> Error  {
        Error::ConfigFile(error)
    }
}

impl From<io::Error> for Error  {
    fn from(error: io::Error) -> Error  {
        Error::IO(error)
    }
}

impl From<str::ParseBoolError> for Error  {
    fn from(error: str::ParseBoolError) -> Error  {
        Error::ParseConfig(error)
    }
}

impl From<log4rs::Error> for Error  {
    fn from(error: log4rs::Error) -> Error  {
        Error::LogConfig(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ConfigFile(ref err) => write!(f, "Configuration error: {}", err),
            Error::ParseConfig(ref err) => write!(f, "Configuration error: {}", err),
            Error::Configuration(ref err) => write!(f, "Configuration error: {}", err),
            Error::LogConfig(ref err) => write!(f, "Configuration error: {}", err),
            Error::IO(ref err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConfigFile(ref err) => err.description(),
            Error::ParseConfig(ref err) => err.description(),
            Error::LogConfig(ref err) => err.description(),
            Error::Configuration(ref err) => err.as_str(),
            Error::IO(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ConfigFile(ref e) => e.cause(),
            Error::ParseConfig(ref e) => e.cause(),
            Error::LogConfig(ref e) => e.cause(),
            Error::Configuration(_) => None,
            Error::IO(ref e) => e.cause(),
        }
    }
}