//! Errorand Result types.
use std::any::type_name;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};
use std::io;

use serde::{Deserialize, Deserializer};
use serde::de::Visitor;
use serde::ser::{Serialize, Serializer};
use sqlx_core::error::BoxDynError;

/// A specialized `Result` type for rbatis::core.
pub type Result<T> = std::result::Result<T, Error>;

/// A generic error that represents all the ways a method can fail inside of rbatis::core.
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

impl From<rexpr::error::Error> for crate::Error{
    fn from(e: rexpr::error::Error) -> Self {
        crate::Error::E(format!("[rbatis]{}",e.to_string()))
    }
}


impl From<sqlx_core::error::BoxDynError> for crate::Error {
    fn from(arg: BoxDynError) -> Self {
        return crate::Error::from(arg.to_string());
    }
}

impl From<sqlx_core::error::Error> for crate::Error {
    fn from(arg: sqlx_core::error::Error) -> Self {
        return crate::Error::from(arg.to_string());
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


// This is what #[derive(Serialize)] would generate.
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}


struct ErrorVisitor;

impl<'de> Visitor<'de> for ErrorVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
        where
            E: std::error::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: std::error::Error,
    {
        Ok(v.to_string())
    }
}

impl<'de> Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let r = deserializer.deserialize_string(ErrorVisitor)?;
        return Ok(Error::from(r));
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