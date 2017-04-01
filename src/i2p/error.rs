use log;
use log4rs;
use serde_yaml;
use std::error;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::num;
use std::str;

#[derive(Debug)]
pub enum ParseError {
    Bool(str::ParseBoolError),
    Int(num::ParseIntError),
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::Bool(ref error) => error.description(),
            ParseError::Int(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParseError::Bool(ref error) => Some(error),
            ParseError::Int(ref error) => Some(error),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Bool(ref error) => write!(f, "Error parsing boolean value: {}", error),
            ParseError::Int(ref error) => write!(f, "Error parsing integer value: {}", error),
        }
    }
}

#[derive(Debug)]
pub enum LogError {
    LogConfig(log4rs::config::Error),
    LogConfigErrors(log4rs::config::Errors),
    LogError {
        message: String,
        error: log4rs::Error,
    },
    SetLogger(log::SetLoggerError),
}

impl error::Error for LogError {
    fn description(&self) -> &str {
        match *self {
            LogError::LogConfig(ref error) => error.description(),
            LogError::LogError { message: ref message, error: ref error } => error.description(),
            LogError::LogConfigErrors(ref errors) => errors.description(),
            LogError::SetLogger(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LogError::LogConfig(ref error) => Some(error),
            LogError::LogError { message: ref message, error: ref error } => Some(error),
            LogError::LogConfigErrors(ref errors) => None,
            LogError::SetLogger(ref error) => Some(error),
        }
    }
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogError::LogConfig(ref error) => {
                write!(f, "Error in logging configuration: {}", error)
            }
            LogError::LogError { message: ref message, error: ref error } => {
                write!(f, "{}: {}", message, error)
            }
            LogError::LogConfigErrors(ref errors) => write!(f, "Logging errors: {}", errors),
            LogError::SetLogger(ref error) => write!(f, "Error setting logger: {}", error),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ConfigFile(serde_yaml::Error),
    IO {
        message: Option<String>,
        error: io::Error,
    },
    ParseConfig(ParseError),
    Configuration(String),
    Logging(LogError),
    Serialization(String),
    Transport(String),
    ConvertString(str::Utf8Error),
    Crypto(String),
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Error {
        Error::ConfigFile(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IO {
            message: None,
            error: error,
        }
    }
}

impl From<str::ParseBoolError> for Error {
    fn from(error: str::ParseBoolError) -> Error {
        Error::ParseConfig(ParseError::Bool(error))
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(error: log::SetLoggerError) -> Error {
        Error::Logging(LogError::SetLogger(error))
    }
}

// impl From<log4rs::Error> for Error {
//     fn from(error: log4rs::Error) -> Error {
//         Error::Logging(LogError::LogError(error))
//     }
// }

impl From<log4rs::config::Error> for Error {
    fn from(error: log4rs::config::Error) -> Error {
        Error::Logging(LogError::LogConfig(error))
    }
}

impl From<log4rs::config::Errors> for Error {
    fn from(error: log4rs::config::Errors) -> Error {
        Error::Logging(LogError::LogConfigErrors(error))
    }
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Error {
        Error::ParseConfig(ParseError::Int(error))
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
            Error::ConfigFile(ref err) => write!(f, "Error reading configuration file: {}", err),
            Error::ParseConfig(ref err) => write!(f, "Error parsing configuration: {}", err),
            Error::Configuration(ref err) => write!(f, "Configuration error: {}", err),
            Error::Logging(ref err) => write!(f, "Logging error: {}", err),
            Error::Serialization(ref err) => write!(f, "Serialization error: {}", err),
            Error::Transport(ref err) => write!(f, "Transport error: {}", err),
            Error::ConvertString(ref err) => write!(f, "String conversion error: {}", err),
            Error::Crypto(ref err) => write!(f, "Crypto error: {}", err),
            Error::IO { message: ref message, error: ref error } => {
                write!(f,
                       "{}: {}",
                       message.clone().unwrap_or(error.description().to_string()),
                       error)
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConfigFile(ref err) => err.description(),
            Error::ParseConfig(ref err) => err.description(),
            Error::Logging(ref err) => err.description(),
            Error::Configuration(ref err) => err.as_str(),
            Error::Serialization(ref err) => err,
            Error::Transport(ref err) => err,
            Error::Crypto(ref err) => err,
            Error::ConvertString(ref err) => err.description(),
            Error::IO { message: ref message, error: ref error } => {
                match *message {
                    Some(ref msg) => msg.as_str(),
                    None => error.description(),
                }
            }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ConfigFile(ref e) => Some(e),
            Error::ParseConfig(ref e) => Some(e),
            Error::Logging(ref e) => Some(e),
            Error::Configuration(_) |
            Error::Serialization(_) |
            Error::Crypto(_) |
            Error::Transport(_) => None,
            Error::ConvertString(ref err) => Some(err),
            Error::IO { message: ref message, error: ref error } => Some(error),
        }
    }
}