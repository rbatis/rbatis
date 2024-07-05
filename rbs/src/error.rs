use std::{error, fmt};
use std::fmt::{Display, Formatter};

/// ser ref Error
#[derive(Debug)]
pub enum Error {
    /// Syntax
    Syntax(String),
}

impl Error {
    pub fn append(self, arg: &str) -> Self {
        match self {
            Error::Syntax(mut v) => {
                v.push_str(arg);
                Self::Syntax(v)
            }
        }
    }
}

impl Display for Error {
    #[cold]
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            Error::Syntax(ref err) => write!(fmt, "{}", err),
        }
    }
}
impl error::Error for Error {}