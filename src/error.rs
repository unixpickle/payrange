use std::fmt;
use std::fmt::{Display, Formatter};

use hyper;
use serde_yaml;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Yaml(serde_yaml::Error),
    Remote{status: i32, reason: Option<String>},
    Other(String)
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Error {
        Error::Hyper(e)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Error {
        Error::Yaml(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            &Error::Hyper(ref e) => e.fmt(f),
            &Error::Yaml(ref e) => e.fmt(f),
            &Error::Remote{status: ref s, reason: Some(ref r)} => {
                write!(f, "remote error with status {}: {}", s, r)
            },
            &Error::Remote{status: ref s, reason: None} => {
                write!(f, "remote error with status {}", s)
            },
            &Error::Other(ref msg) => write!(f, "{}", msg)
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Hyper(ref e) => e.description(),
            &Error::Yaml(ref e) => e.description(),
            &Error::Remote{status: _, reason: Some(ref r)} => {
                r
            },
            &Error::Other(ref msg) => {
                msg
            },
            _ => "undescribed error from remote API"
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match self {
            &Error::Hyper(ref e) => Some(e),
            &Error::Yaml(ref e) => Some(e),
            _ => None
        }
    }
}
