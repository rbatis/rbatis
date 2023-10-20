use crate::result_set::MySqlTypeInfo;
use crate::value::MySqlValue;
use rbdc::Error;

pub mod date;
pub mod datetime;
pub mod decimal;
pub mod decode;
pub mod enums;
pub mod geometry;
pub mod json;
pub mod set;
pub mod time;
pub mod timestamp;
pub mod uuid;
pub mod value;
pub mod year;

pub trait TypeInfo {
    fn type_info(&self) -> MySqlTypeInfo;
}

pub trait Decode {
    fn decode(value: MySqlValue) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait Encode {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error>;
}


#[cfg(test)]
mod test{
    #[test]
    fn test_datetime(){

    }
}