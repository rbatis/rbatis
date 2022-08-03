use std::borrow::Cow;
use crate::arguments::{PgArgumentBuffer, PgArguments};
use crate::type_info::PgTypeInfo;
use crate::types::Oid;
use rbs::Value;
use std::mem;
use std::str::FromStr;
use rbdc::date::Date;
use rbdc::datetime::FastDateTime;
use rbdc::decimal::Decimal;
use rbdc::Error;
use rbdc::json::Json;
use rbdc::timestamp::Timestamp;
use rbdc::types::time::Time;
use rbdc::uuid::Uuid;
use crate::types::byte::Bytea;
use crate::types::money::Money;
use crate::types::timestamptz::Timestamptz;
use crate::types::timetz::Timetz;

pub enum IsNull {
    No,
    Yes,
}

pub trait Encode {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error>;
}

impl From<Vec<Value>> for PgArguments {
    fn from(args: Vec<Value>) -> Self {
        let mut arg = PgArguments {
            types: Vec::with_capacity(args.len()),
            buffer: PgArgumentBuffer::default(),
        };
        for x in args {
            arg.add(x).unwrap();
        }
        arg
    }
}


impl Encode for Value {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        Ok(match self {
            Value::Null => IsNull::Yes,
            Value::Bool(v) => {
                v.encode(buf)?
            }
            Value::I32(v) => {
                v.encode(buf)?
            }
            Value::I64(v) => {
                v.encode(buf)?
            }
            Value::U32(v) => {
                v.encode(buf)?
            }
            Value::U64(v) => {
                v.encode(buf)?
            }
            Value::F32(v) => {
                v.encode(buf)?
            }
            Value::F64(v) => {
                v.encode(buf)?
            }
            Value::String(v) => {
                //default -> string
                v.encode(buf)?
            }
            Value::Binary(v) => {
                v.encode(buf)?
            }
            Value::Array(v) => {
                v.encode(buf)?
            }
            Value::Map(v) => {
                IsNull::Yes
            }
            Value::Ext(type_name, v) => {
                match type_name {
                    "Uuid" => {
                        Uuid(v.into_string().unwrap_or_default()).encode(buf)?
                    }
                    //decimal = 12345678
                    "Decimal" => {
                       Decimal(v.into_string().unwrap_or_default()).encode(buf)?
                    }
                    //Date = "1993-02-06"
                    "Date" => {
                        Date(fastdate::Date::from_str(&v.into_string().unwrap_or_default()).unwrap()).encode(buf)?
                    }
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => {
                        Time(fastdate::Time::from_str(&v.into_string().unwrap_or_default()).unwrap()).encode(buf)?
                    }
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => {
                        Timestamp(v.as_u64().unwrap_or_default()).encode(buf)?
                    }
                    "DateTime" => {
                        FastDateTime(fastdate::DateTime::from_str(&v.into_string().unwrap_or_default()).unwrap()).encode(buf)?
                    }
                    "Bytea" => {
                        Bytea(v.as_u64().unwrap_or_default() as u8).encode(buf)?
                    }
                    "Char" => {
                        v.into_string().unwrap_or_default().encode(buf)?
                    }
                    "Name" => {
                        v.into_string().unwrap_or_default().encode(buf)?
                    }
                    "Int8" => {
                        (v.as_i64().unwrap_or_default() as i32).encode(buf)?
                    }
                    "Int2" => {
                        (v.as_i64().unwrap_or_default() as i8).encode(buf)?
                    }
                    "Int4" => {
                        (v.as_i64().unwrap_or_default() as i16).encode(buf)?
                    }
                    "Text" => {
                        v.into_string().unwrap_or_default().encode(buf)?
                    }
                    "Oid" => Oid::from(v.as_u64().unwrap_or_default() as u32).encode(buf)?,
                    "Json" => Json(v.into_string().unwrap_or_default()).encode(buf)?,
                    "Point" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Lseg" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Path" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Box" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Polygon" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Line" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Cidr" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Float4" => {
                        (v.as_f64().unwrap_or_default() as f32).encode(buf)?
                    }
                    "Float8" => {
                        v.as_f64().unwrap_or_default().encode(buf)?
                    }
                    "Unknown" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Circle" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Macaddr8" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Macaddr" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Inet" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Bpchar" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Varchar" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Timestamptz" => {
                       Timestamptz(v.as_u64().unwrap_or_default()).encode(buf)?
                    }
                    "Interval" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Timetz" => {
                        Timetz(rbs::from_value(*v).map_err(|e|Error::from(e.to_string()))?).encode(buf)?
                    }
                    "Bit" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Varbit" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Numeric" => {
                        Decimal(v.into_string().unwrap_or_default()).encode(buf)?
                    }
                    "Record" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Jsonb" => {
                        Json(v.into_string().unwrap_or_default()).encode(buf)?
                    }
                    "Int4Range" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "NumRange" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "TsRange" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "TstzRange" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "DateRange" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Int8Range" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Jsonpath" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Money" => {
                        Money(v.as_i64().unwrap_or_default()).encode(buf)?
                    }
                    "Void" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Custom" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "DeclareWithName" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "DeclareWithOid" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    _ => IsNull::Yes,
                }
            }
        })
    }
}






