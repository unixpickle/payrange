use std::fmt;
use std::fmt::{Display, Formatter};

use hyper;
use serde_yaml;

#[derive(Debug)]
pub enum Error {
    HyperError(hyper::Error),
    YamlError(serde_yaml::Error),
    RemoteError{status: i32, reason: Option<String>}
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Error {
        Error::HyperError(e)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Error {
        Error::YamlError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            &Error::HyperError(ref e) => e.fmt(f),
            &Error::YamlError(ref e) => e.fmt(f),
            &Error::RemoteError{status: ref s, reason: Some(ref r)} => {
                write!(f, "remote error with status {}: {}", s, r)
            },
            &Error::RemoteError{status: ref s, reason: None} => {
                write!(f, "remote error with status {}", s)
            }
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::HyperError(ref e) => e.description(),
            &Error::YamlError(ref e) => e.description(),
            &Error::RemoteError{status: _, reason: Some(ref r)} => {
                r
            },
            _ => "undescribed error from remote API"
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match self {
            &Error::HyperError(ref e) => Some(e),
            &Error::YamlError(ref e) => Some(e),
            &Error::RemoteError{status: _, reason: _} => None
        }
    }
}
