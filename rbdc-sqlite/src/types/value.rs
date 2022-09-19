use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::type_info::DataType;
use crate::{SqliteArgumentValue, SqliteValue};
use rbdc::Error;
use rbs::Value;

impl Decode for Value {
    fn decode(value: SqliteValue) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match value.type_info().0 {
            DataType::Null => Ok(Value::Null),
            DataType::Int => Ok(Value::I64(i64::decode(value)?)),
            DataType::Float => Ok(Value::F64(f64::decode(value)?)),
            DataType::Text => Ok(Value::String(String::decode(value)?)),
            DataType::Blob => Ok(Value::Binary(Vec::<u8>::decode(value)?)),
            DataType::Numeric => Ok(Value::String(String::decode(value)?)),
            DataType::Bool => Ok(Value::Bool(bool::decode(value)?)),
            DataType::Int64 => Ok(Value::I64(i64::decode(value)?)),
            DataType::Date => Ok(Value::Ext(
                "Date",
                Box::new(Value::String(String::decode(value)?)),
            )),
            DataType::Time => Ok(Value::Ext(
                "Time",
                Box::new(Value::String(String::decode(value)?)),
            )),
            DataType::Datetime => Ok(Value::Ext(
                "Datetime",
                Box::new(Value::String(String::decode(value)?)),
            )),
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
            Value::Map(_) => Ok(IsNull::Yes),
            Value::Ext(t, v) => match t {
                "Date" => {
                    v.into_string().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                "DateTime" => {
                    v.into_string().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                "Time" => {
                    v.into_string().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                "Timestamp" => {
                    (v.as_u64().unwrap_or_default() as i64).encode(args)?;
                    Ok(IsNull::No)
                }
                "Decimal" => {
                    v.into_string().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                "Json" => {
                    v.into_bytes().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                "Uuid" => {
                    v.into_string().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                _ => Ok(IsNull::Yes),
            },
        }
    }
}
