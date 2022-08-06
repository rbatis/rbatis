use std::error;
use std::fmt::{self, Display, Formatter};

use serde::de::Unexpected;

use crate::{Value, ValueRef};

pub use self::de::{deserialize_from, from_value, EnumRefDeserializer};
pub use self::se::{to_value, to_value_def};

mod de;
mod se;
mod se_ref;
pub use se_ref::to_value_ref;

/// ser ref Error
#[derive(Debug)]
pub enum Error {
    /// Syntax
    Syntax(String),
}

impl Display for Error {
    #[cold]
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            Error::Syntax(ref err) => write!(fmt, "error while decoding value: {}", err),
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

impl<'a> ValueExt for ValueRef<'a> {
    #[cold]
    fn unexpected(&self) -> Unexpected<'_> {
        match *self {
            ValueRef::Null => Unexpected::Unit,
            ValueRef::Bool(v) => Unexpected::Bool(v),
            ValueRef::I32(v) => Unexpected::Signed(v as i64),
            ValueRef::I64(v) => Unexpected::Signed(v),
            ValueRef::U32(v) => Unexpected::Unsigned(v as u64),
            ValueRef::U64(v) => Unexpected::Unsigned(v),
            ValueRef::F32(v) => Unexpected::Float(v as f64),
            ValueRef::F64(v) => Unexpected::Float(v),
            ValueRef::String(ref v) => Unexpected::Bytes(&v.as_bytes()[..]),
            ValueRef::Binary(ref v) => Unexpected::Bytes(v),
            ValueRef::Array(..) => Unexpected::Seq,
            ValueRef::Map(..) => Unexpected::Map,
            ValueRef::Ext(..) => Unexpected::Seq,
        }
    }
}
