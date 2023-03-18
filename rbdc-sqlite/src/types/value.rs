use std::ops::Index;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::type_info::DataType;
use crate::{SqliteArgumentValue, SqliteValue};
use rbdc::{Error, RBDCString};
use rbdc::date::Date;
use rbdc::datetime::DateTime;
use rbdc::decimal::Decimal;
use rbdc::json::Json;
use rbdc::timestamp::Timestamp;
use rbdc::types::time::Time;
use rbdc::uuid::Uuid;
use rbs::Value;

impl Decode for Value {
    fn decode(value: SqliteValue) -> Result<Self, Error>
        where
            Self: Sized,
    {
        if value.type_info_opt().is_none() {
            return Ok(Value::Null);
        }
        match value.type_info().0 {
            DataType::Null => Ok(Value::Null),
            DataType::Int => Ok(Value::I64(i64::decode(value)?)),
            DataType::Float => Ok(Value::F64(f64::decode(value)?)),
            DataType::Text => Ok(Value::String(String::decode(value)?)),
            DataType::Blob => Ok(Value::Binary(Vec::<u8>::decode(value)?)),
            DataType::Numeric => Ok(Value::String(String::decode(value)?)),
            DataType::Bool => Ok(Value::Bool(bool::decode(value)?)),
            DataType::Int64 => Ok(Value::I64(i64::decode(value)?)),
            DataType::Date => Ok(Value::String(String::decode(value)?)),
            DataType::Time => Ok(Value::String(String::decode(value)?)),
            DataType::Datetime => Ok(Value::String(String::decode(value)?)),
        }
    }
}

impl Encode for Value {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        match self {
            Value::Null => Ok(IsNull::Yes),
            Value::Bool(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::I32(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::I64(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::U32(v) => {
                (v as i32).encode(args)?;
                Ok(IsNull::No)
            }
            Value::U64(v) => {
                (v as i64).encode(args)?;
                Ok(IsNull::No)
            }
            Value::F32(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::F64(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::String(mut v) => {
                if Date::is(&v) != "" {
                    Date::trim_ends_match(&mut v);
                    v.encode(args)?;
                    Ok(IsNull::No)
                } else if DateTime::is(&v) != "" {
                    DateTime::trim_ends_match(&mut v);
                    v.encode(args)?;
                    Ok(IsNull::No)
                } else if Time::is(&v) != "" {
                    Time::trim_ends_match(&mut v);
                    v.encode(args)?;
                    Ok(IsNull::No)
                } else if Timestamp::is(&v) != "" {
                    Timestamp::trim_ends_match(&mut v);
                    let ts = Timestamp::decode(v.as_str())?;
                    (ts.0 as i64).encode(args)?;
                    Ok(IsNull::No)
                } else if Decimal::is(&v) != "" {
                    Decimal::trim_ends_match(&mut v);
                    v.encode(args)?;
                    Ok(IsNull::No)
                } else if Uuid::is(&v) != "" {
                    Uuid::trim_ends_match(&mut v);
                    v.encode(args)?;
                    Ok(IsNull::No)
                } else {
                    v.encode(args)?;
                    Ok(IsNull::No)
                }
            }
            Value::Binary(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::Array(arr) => {
                Value::Array(arr).to_string().into_bytes().encode(args)?;
                Ok(IsNull::No)
            }
            Value::Map(m) => {
                Value::Map(m).to_string().into_bytes().encode(args)?;
                Ok(IsNull::No)
            }
        }
    }
}
