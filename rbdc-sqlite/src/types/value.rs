use std::ops::Index;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::type_info::DataType;
use crate::{SqliteArgumentValue, SqliteValue};
use rbdc::Error;
use rbdc::json::Json;
use rbs::Value;

impl Decode for Value {
    fn decode(value: SqliteValue) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if value.type_info_opt().is_none(){
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
            DataType::Date => Ok(Value::from(("Date",
                                              Value::String(String::decode(value)?),))),
            DataType::Time => Ok(Value::from(( "Time", Value::String(String::decode(value)?)))),
            DataType::Datetime => Ok(Value::from(( "Datetime", Value::String(String::decode(value)?)))),
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
            Value::String(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::Binary(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::Array(_) => Ok(IsNull::Yes),
            Value::Map(mut m) => {
                let v = m.rm("value");
                let t = m.index("type").as_str().unwrap_or_default();
                match t {
                    "Date" => {
                        v.as_str().unwrap_or_default().to_string().encode(args)?;
                        Ok(IsNull::No)
                    }
                    "DateTime" => {
                        v.as_str().unwrap_or_default().to_string().encode(args)?;
                        Ok(IsNull::No)
                    }
                    "Time" => {
                        v.as_str().unwrap_or_default().to_string().encode(args)?;
                        Ok(IsNull::No)
                    }
                    "Timestamp" => {
                        (v.as_i64().unwrap_or_default()).encode(args)?;
                        Ok(IsNull::No)
                    }
                    "Decimal" => {
                        v.as_str().unwrap_or_default().to_string().encode(args)?;
                        Ok(IsNull::No)
                    }
                    "Json" => {
                        Json::from(v).value.to_string().into_bytes().encode(args)?;
                        Ok(IsNull::No)
                    }
                    "Uuid" => {
                        v.as_str().unwrap_or_default().to_string().encode(args)?;
                        Ok(IsNull::No)
                    }
                    _ =>  {
                        //json
                        Json::from(Value::Map(m)).value.to_string().into_bytes().encode(args)?;
                        Ok(IsNull::No)
                    }
                }
            },
        }
    }
}
