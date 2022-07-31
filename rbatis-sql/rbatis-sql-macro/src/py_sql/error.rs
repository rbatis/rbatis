//! Errorand Result types.
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};
use std::io;



pub type Result<T> = std::result::Result<T, Error>;

/// A generic error that represents all the ways a method can fail inside of rexpr::core.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Default Error
    E(String),
}

impl Display for Error {
    // IntellijRust does not understand that [non_exhaustive] applies only for downstream crates
    // noinspection RsMatchCheck
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::E(error) => write!(f, "{}", error),
        }
    }
}

impl StdError for Error {}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Self {
        Error::from(err.to_string())
    }
}

impl From<&str> for Error {
    fn from(arg: &str) -> Self {
        return Error::E(arg.to_string());
    }
}

impl From<std::string::String> for Error {
    fn from(arg: String) -> Self {
        return Error::E(arg);
    }
}

impl From<&dyn std::error::Error> for Error {
    fn from(arg: &dyn std::error::Error) -> Self {
        return Error::E(arg.to_string());
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        Error::from(self.to_string())
    }

    fn clone_from(&mut self, source: &Self) {
        *self = Self::from(source.to_string());
    }
}


pub trait OptionToResult<T> {
    fn to_result(self, error_str: &str) -> Result<T>;
}

impl<T> OptionToResult<T> for Option<T> {
    fn to_result(self, error_str: &str) -> Result<T> {
        if self.is_some() {
            Ok(self.unwrap())
        } else {
            Err(Error::from(error_str))
        }
    }
}

#[test]
fn test_json_error() {
    let e = Error::from("fuck");
    let s = serde_json::to_string(&e).unwrap();
    println!("{}", s.as_str());
}
