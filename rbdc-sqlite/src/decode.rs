use rbdc::Error;
use rbs::Value;
use crate::{SqliteTypeInfo, SqliteValue, SqliteValueRef};
use crate::type_info::DataType;

pub trait Decode {
    fn decode(value: SqliteValue) -> Result<Self, Error> where Self: Sized;
}

impl Decode for Value {
    fn decode(value: SqliteValue) -> Result<Self, Error> where Self: Sized {
        match value.type_info.0 {
            DataType::Null => {
                Ok(Value::Null)
            }
            DataType::Int => Ok(Value::I32(i32::decode(value)?)),
            DataType::Float => { Ok(Value::F64(f64::decode(value)?)) }
            DataType::Text => { Ok(Value::String(String::decode(value)?)) }
            DataType::Blob => { Ok(Value::Binary(Vec::<u8>::decode(value)?)) }
            DataType::Numeric => { Ok(Value::String(String::decode(value)?)) }
            DataType::Bool => { Ok(Value::Bool(bool::decode(value)?)) }
            DataType::Int64 => { Ok(Value::I64(i64::decode(value)?)) }
            DataType::Date => { Ok(Value::Ext("Date", Box::new(Value::String(String::decode(value)?)))) }
            DataType::Time => { Ok(Value::Ext("Time", Box::new(Value::String(String::decode(value)?)))) }
            DataType::Datetime => { Ok(Value::Ext("Datetime", Box::new(Value::String(String::decode(value)?)))) }
        }
    }
}