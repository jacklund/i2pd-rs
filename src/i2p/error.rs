use ini::ini;
use log4rs;
use std::error;
use std::fmt;
use std::io;
use std::num;
use std::str;

#[derive(Debug)]
pub enum Error {
    ConfigFile(ini::Error),
    IO(io::Error),
    ParseConfig(str::ParseBoolError),
    ParseIntConfig(num::ParseIntError),
    Configuration(String),
    LogConfig(log4rs::Error),
    Serialization(String),
    Transport(String),
    ConvertString(str::Utf8Error),
    Crypto(String),
}

impl From<ini::Error> for Error {
    fn from(error: ini::Error) -> Error {
        Error::ConfigFile(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IO(error)
    }
}

impl From<str::ParseBoolError> for Error {
    fn from(error: str::ParseBoolError) -> Error {
        Error::ParseConfig(error)
    }
}

impl From<log4rs::Error> for Error {
    fn from(error: log4rs::Error) -> Error {
        Error::LogConfig(error)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Error {
        Error::ParseIntConfig(error)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(error: str::Utf8Error) -> Error {
        Error::ConvertString(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ConfigFile(ref err) => write!(f, "Configuration error: {}", err),
            Error::ParseConfig(ref err) => write!(f, "Configuration error: {}", err),
            Error::ParseIntConfig(ref err) => write!(f, "Configuration error: {}", err),
            Error::Configuration(ref err) => write!(f, "Configuration error: {}", err),
            Error::LogConfig(ref err) => write!(f, "Configuration error: {}", err),
            Error::Serialization(ref err) => write!(f, "Serialization error: {}", err),
            Error::Transport(ref err) => write!(f, "Transport error: {}", err),
            Error::ConvertString(ref err) => write!(f, "String conversion error: {}", err),
            Error::Crypto(ref err) => write!(f, "Crypto error: {}", err),
            Error::IO(ref err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConfigFile(ref err) => err.description(),
            Error::ParseConfig(ref err) => err.description(),
            Error::ParseIntConfig(ref err) => err.description(),
            Error::LogConfig(ref err) => err.description(),
            Error::Configuration(ref err) => err.as_str(),
            Error::Serialization(ref err) => err,
            Error::Transport(ref err) => err,
            Error::Crypto(ref err) => err,
            Error::ConvertString(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ConfigFile(ref e) => e.cause(),
            Error::ParseConfig(ref e) => e.cause(),
            Error::ParseIntConfig(ref e) => e.cause(),
            Error::LogConfig(ref e) => e.cause(),
            Error::Configuration(_) |
            Error::Serialization(_) |
            Error::Crypto(_) |
            Error::Transport(_) => None,
            Error::ConvertString(ref err) => err.cause(),
            Error::IO(ref e) => e.cause(),
        }
    }
}