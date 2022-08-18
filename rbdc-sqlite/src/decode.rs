use crate::SqliteValue;
use rbdc::Error;

pub trait Decode {
    fn decode(value: SqliteValue) -> Result<Self, Error>
    where
        Self: Sized;
}
