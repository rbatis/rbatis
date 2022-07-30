use rbdc::Error;
use crate::SqliteValueRef;

pub trait Decode{
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, Error> where Self: Sized;
}