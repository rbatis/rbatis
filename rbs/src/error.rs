use serde::{ser, Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError, TryFromIntError};
use std::str::Utf8Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    E(String),
}

impl Error {
    pub fn append(self, arg: &str) -> Error {
        match self {
            Error::E(mut e) => {
                e.push_str(arg);
                Error::E(e)
            }
        }
    }
}

impl Error {
    #[allow(dead_code)]
    #[inline]
    pub fn protocol(err: impl Display) -> Self {
        Error::from(format!("ProtocolError {}", err))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::E(e) => std::fmt::Display::fmt(&e, f),
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::E(format!("{}", msg))
    }
}

impl serde::de::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Self {
        Error::E(format!("{}", msg))
    }
}

impl From<std::io::Error> for Error {
    fn from(arg: std::io::Error) -> Self {
        Error::from(arg.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::from(e.to_string())
    }
}

impl From<&str> for Error {
    fn from(arg: &str) -> Self {
        Error::from(arg.to_string())
    }
}

impl From<String> for Error {
    fn from(arg: String) -> Self {
        Error::E(arg)
    }
}

impl From<ParseIntError> for Error {
    fn from(arg: ParseIntError) -> Self {
        Error::from(arg.to_string())
    }
}

impl From<ParseFloatError> for Error {
    fn from(arg: ParseFloatError) -> Self {
        Error::from(arg.to_string())
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::from(e.to_string())
    }
}


// Format an error message as a `Protocol` error
#[macro_export]
macro_rules! err_protocol {
    ($expr:expr) => {
        $crate::Error::E($expr.into())
    };

    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::E(format!($fmt, $($arg)*))
    };
}
