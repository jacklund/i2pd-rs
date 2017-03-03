use ini::ini;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Config(String),
}

impl From<ini::Error> for Error {
    fn from(error: ini::Error) -> Error {
        Error::Config(format!("{}", error))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Config(ref err) => write!(f, "Configuration error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Config(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Config(_) => None,
        }
    }
}