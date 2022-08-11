use crate::io::MySqlBufMutExt;
use crate::types::enums::Enum;
use crate::types::set::Set;
use crate::types::year::Year;
use crate::types::Encode;
use rbdc::date::Date;
use rbdc::datetime::FastDateTime;
use rbdc::decimal::Decimal;
use rbdc::json::Json;
use rbdc::timestamp::Timestamp;
use rbdc::types::time::Time;
use rbdc::uuid::Uuid;
use rbdc::Error;
use rbs::Value;
use std::str::FromStr;

impl Encode for Value {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        match self {
            Value::Null => Ok(0),
            Value::Bool(v) => {
                buf.extend(&(v as i8).to_le_bytes());
                Ok(1)
            }
            Value::I32(v) => {
                buf.extend(v.to_le_bytes());
                Ok(4)
            }
            Value::I64(v) => {
                buf.extend(v.to_le_bytes());
                Ok(8)
            }
            Value::U32(v) => {
                buf.extend(v.to_le_bytes());
                Ok(4)
            }
            Value::U64(v) => {
                buf.extend(v.to_le_bytes());
                Ok(8)
            }
            Value::F32(v) => {
                let len = &v.to_le_bytes();
                buf.extend(len);
                Ok(len.len())
            }
            Value::F64(v) => {
                let len = &v.to_le_bytes();
                buf.extend(len);
                Ok(len.len())
            }
            Value::String(v) => {
                buf.put_str_lenenc(&v);
                Ok(0)
            }
            Value::Binary(v) => {
                buf.put_bytes_lenenc(v);
                Ok(0)
            }
            Value::Array(_) => Ok(0),
            Value::Map(_) => Ok(0),
            Value::Ext(ext_type, v) => {
                match ext_type {
                    "Uuid" => {
                        //uuid -> string
                        Uuid(v.into_string().unwrap_or_default()).encode(buf)
                    }
                    //decimal = 12345678
                    "Decimal" => Decimal(v.into_string().unwrap_or_default()).encode(buf),
                    //year = "1993"
                    "Year" => Year(v.as_u64().unwrap_or_default() as u16).encode(buf),
                    //Date = "1993-02-06"
                    "Date" => Date(
                        fastdate::Date::from_str(&v.into_string().unwrap_or_default()).unwrap(),
                    )
                    .encode(buf),
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => Time(
                        fastdate::Time::from_str(&v.into_string().unwrap_or_default()).unwrap(),
                    )
                    .encode(buf),
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => Timestamp(v.as_u64().unwrap_or_default()).encode(buf),
                    "DateTime" => FastDateTime(
                        fastdate::DateTime::from_str(&v.into_string().unwrap_or_default()).unwrap(),
                    )
                    .encode(buf),
                    "Json" => Json(v.into_string().unwrap_or_default()).encode(buf),
                    "Enum" => Enum(v.into_string().unwrap_or_default()).encode(buf),
                    "Set" => Set(v.into_string().unwrap_or_default()).encode(buf),
                    _ => {
                        buf.put_bytes_lenenc(v.into_bytes().unwrap_or_default());
                        Ok(0)
                    }
                }
            }
        }
    }
}
