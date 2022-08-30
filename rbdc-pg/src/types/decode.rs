use crate::value::PgValue;
use rbdc::Error;

pub trait Decode: Sized {
    /// Decode a new value of this type using a raw value from the database.
    fn decode(value: PgValue) -> Result<Self, Error>;
}
