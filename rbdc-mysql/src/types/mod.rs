use rbdc::Error;
use crate::result_set::MySqlTypeInfo;
use crate::value::MySqlValue;

pub mod decode;
pub mod encode;
pub mod type_info;
pub mod decimal;
pub mod date;
pub mod datetime;
pub mod enums;
pub mod geometry;
pub mod json;
pub mod set;
pub mod time;
pub mod year;
pub mod uuid;
pub mod timestamp;

pub trait TypeInfo {
    fn type_info(&self) -> MySqlTypeInfo;
}

pub trait Decode {
    fn decode(value: MySqlValue) -> Result<Self, Error> where Self:Sized;
}

pub trait Encode {
    fn encode(self, buf:&mut Vec<u8>) -> Result<usize, Error>;
}