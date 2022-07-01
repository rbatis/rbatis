use std::{error, fmt, fmt::Display, io, sync::Arc};
use std::fmt::Formatter;
use serde::ser;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    E(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::E(e) => {
                f.write_str(e)
            }
        }
    }
}

impl error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::E(msg.to_string())
    }
}
