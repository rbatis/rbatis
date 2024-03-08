use std::error;
use std::fmt::{self, Display, Formatter};

use serde::de::Unexpected;

use crate::Value;

pub use self::de::{deserialize_from, from_value};
pub use self::se::{to_value, to_value_def};

mod de;
mod se;

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

trait ValueExt {
    fn unexpected(&self) -> Unexpected<'_>;
}

impl ValueExt for Value {
    #[cold]
    fn unexpected(&self) -> Unexpected<'_> {
        match *self {
            Value::Null => Unexpected::Unit,
            Value::Bool(v) => Unexpected::Bool(v),
            Value::I32(v) => Unexpected::Signed(v as i64),
            Value::I64(v) => Unexpected::Signed(v),
            Value::U32(v) => Unexpected::Unsigned(v as u64),
            Value::U64(v) => Unexpected::Unsigned(v),
            Value::F32(v) => Unexpected::Float(v as f64),
            Value::F64(v) => Unexpected::Float(v),
            Value::String(ref v) => Unexpected::Bytes(v.as_bytes()),
            Value::Binary(ref v) => Unexpected::Bytes(v),
            Value::Array(..) => Unexpected::Seq,
            Value::Map(..) => Unexpected::Map,
            Value::Ext(..) => Unexpected::Seq,
        }
    }
}

impl ValueExt for &Value {
    #[cold]
    fn unexpected(&self) -> Unexpected<'_> {
        match *self {
            Value::Null => Unexpected::Unit,
            Value::Bool(v) => Unexpected::Bool(*v),
            Value::I32(v) => Unexpected::Signed(*v as i64),
            Value::I64(v) => Unexpected::Signed(*v),
            Value::U32(v) => Unexpected::Unsigned(*v as u64),
            Value::U64(v) => Unexpected::Unsigned(*v),
            Value::F32(v) => Unexpected::Float(*v as f64),
            Value::F64(v) => Unexpected::Float(*v),
            Value::String(ref v) => Unexpected::Bytes(v.as_bytes()),
            Value::Binary(ref v) => Unexpected::Bytes(v),
            Value::Array(..) => Unexpected::Seq,
            Value::Map(..) => Unexpected::Map,
            Value::Ext(..) => Unexpected::Seq,
        }
    }
}
