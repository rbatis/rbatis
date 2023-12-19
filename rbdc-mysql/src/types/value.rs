use crate::io::MySqlBufMutExt;
use crate::protocol::text::ColumnType;
use crate::result_set::MySqlTypeInfo;
use crate::types::decode::{
    decode_date, decode_time, decode_timestamp, decode_year, f32_decode, f64_decode, int_decode,
    uint_decode, decode_bool
};
use crate::types::enums::Enum;
use crate::types::set::Set;
use crate::types::year::Year;
use crate::types::{Decode, Encode, TypeInfo};
use crate::value::MySqlValue;
use rbdc::date::Date;
use rbdc::datetime::DateTime;
use rbdc::decimal::Decimal;
use rbdc::json::Json;
use rbdc::timestamp::Timestamp;
use rbdc::types::time::Time;
use rbdc::uuid::Uuid;
use rbdc::Error;
use rbs::Value;
use std::str::FromStr;

impl TypeInfo for Value {
    fn type_info(&self) -> MySqlTypeInfo {
        match self {
            Value::Null => MySqlTypeInfo::null(),
            Value::Bool(_) => MySqlTypeInfo::from_type(ColumnType::Tiny),
            Value::I32(_) => MySqlTypeInfo::from_type(ColumnType::Long),
            Value::I64(_) => MySqlTypeInfo::from_type(ColumnType::LongLong),
            Value::U32(_) => MySqlTypeInfo::from_type(ColumnType::Long),
            Value::U64(_) => MySqlTypeInfo::from_type(ColumnType::LongLong),
            Value::F32(_) => MySqlTypeInfo::from_type(ColumnType::Float),
            Value::F64(_) => MySqlTypeInfo::from_type(ColumnType::Double),
            Value::String(_) => MySqlTypeInfo::from_type(ColumnType::VarChar),
            Value::Binary(_) => MySqlTypeInfo::from_type(ColumnType::Blob),
            Value::Array(_) => MySqlTypeInfo::from_type(ColumnType::Json),
            Value::Map(_) => MySqlTypeInfo::from_type(ColumnType::Json),
            Value::Ext(ext_type, _) => {
                match *ext_type {
                    "Uuid" => MySqlTypeInfo::from_type(ColumnType::VarChar),
                    //decimal = 12345678
                    "Decimal" => MySqlTypeInfo::from_type(ColumnType::NewDecimal),
                    //year = "1993"
                    "Year" => MySqlTypeInfo::from_type(ColumnType::Year),
                    //Date = "1993-02-06"
                    "Date" => MySqlTypeInfo::from_type(ColumnType::Date),
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => MySqlTypeInfo::from_type(ColumnType::Time),
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => {
                        //datetime=5byte
                        MySqlTypeInfo::from_type(ColumnType::Timestamp)
                    }
                    "DateTime" => MySqlTypeInfo::from_type(ColumnType::Datetime),
                    "Json" => MySqlTypeInfo::from_type(ColumnType::Json),
                    "Enum" => MySqlTypeInfo::from_type(ColumnType::Enum),
                    "Set" => MySqlTypeInfo::from_type(ColumnType::Set),
                    _ => MySqlTypeInfo::null(),
                }
            }
        }
    }
}

impl Encode for Value {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        match self {
            Value::Null => Ok(0),
            Value::Bool(v) => {
                buf.extend(&(v as u8).to_le_bytes());
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
            Value::Array(v) => Json(Value::Array(v).to_string()).encode(buf),
            Value::Map(v) => Json(Value::Map(v).to_string()).encode(buf),
            Value::Ext(ext_type, v) => {
                match ext_type {
                    "Uuid" => {
                        //uuid -> string
                        Uuid(v.into_string().unwrap_or_default()).encode(buf)
                    }
                    //decimal = 12345678
                    "Decimal" => Decimal::from_str(v.as_str().unwrap_or_default())
                        .unwrap_or_default()
                        .encode(buf),
                    //year = "1993"
                    "Year" => Year(v.as_u64().unwrap_or_default() as u16).encode(buf),
                    //Date = "1993-02-06"
                    "Date" => Date(fastdate::Date::from_str(
                        &v.into_string().unwrap_or_default(),
                    )?)
                    .encode(buf),
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => Time(fastdate::Time::from_str(
                        &v.into_string().unwrap_or_default(),
                    )?)
                    .encode(buf),
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => Timestamp(v.as_i64().unwrap_or_default()).encode(buf),
                    "DateTime" => DateTime(fastdate::DateTime::from_str(
                        &v.into_string().unwrap_or_default(),
                    )?)
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

impl Decode for Value {
    fn decode(v: MySqlValue) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(match v.type_info().r#type {
            ColumnType::Tiny => Value::Bool(decode_bool(v).unwrap_or_default()),
            ColumnType::Short => Value::I32(int_decode(v).unwrap_or_default() as i32),
            ColumnType::Long => Value::I64(int_decode(v).unwrap_or_default()),
            ColumnType::Float => Value::F32(f32_decode(v).unwrap_or_default()),
            ColumnType::Double => Value::F64(f64_decode(v).unwrap_or_default()),
            ColumnType::Null => Value::Null,
            ColumnType::LongLong => Value::I64(int_decode(v).unwrap_or_default()),
            ColumnType::Int24 => Value::I32(int_decode(v).unwrap_or_default() as i32),
            ColumnType::VarChar => Value::String(v.as_str().unwrap_or_default().to_string()),
            ColumnType::Bit => Value::U64(uint_decode(v).unwrap_or_default()),
            ColumnType::TinyBlob => Value::Binary(v.as_bytes().unwrap_or_default().to_vec()),
            ColumnType::MediumBlob => Value::Binary(v.as_bytes().unwrap_or_default().to_vec()),
            ColumnType::LongBlob => Value::Binary(v.as_bytes().unwrap_or_default().to_vec()),
            ColumnType::Blob => Value::Binary(v.as_bytes().unwrap_or_default().to_vec()),
            ColumnType::VarString => Value::String(v.as_str().unwrap_or_default().to_string()),
            ColumnType::String => Value::String(v.as_str().unwrap_or_default().to_string()),
            ColumnType::Timestamp => Value::Ext(
                "Timestamp",
                Box::new(Value::U64({
                    let s = decode_timestamp(v).unwrap_or_default();
                    let date = fastdate::DateTime::from_str(&s)?;
                    date.unix_timestamp_millis() as u64
                })),
            ),
            ColumnType::Decimal => Value::Ext(
                "Decimal",
                Box::new(Value::String(v.as_str().unwrap_or("0").to_string())),
            ),
            ColumnType::Date => Value::Ext(
                "Date",
                Box::new(Value::String(decode_date(v).unwrap_or_default())),
            ),
            ColumnType::Time => Value::Ext(
                "Time",
                Box::new(Value::String(decode_time(v).unwrap_or_default())),
            ),
            ColumnType::Datetime => Value::Ext(
                "DateTime",
                Box::new(Value::String(decode_timestamp(v).unwrap_or_default())),
            ),
            ColumnType::Year => Value::Ext(
                "Year",
                Box::new(Value::String(decode_year(v).unwrap_or_default())),
            ),
            ColumnType::Json => Value::Ext(
                "Json",
                Box::new(Value::String(v.as_str().unwrap_or_default().to_string())),
            ),
            ColumnType::NewDecimal => Value::Ext(
                "Decimal",
                Box::new(Value::String(v.as_str().unwrap_or("0").to_string())),
            ),
            ColumnType::Enum => Value::Ext(
                "Enum",
                Box::new(Value::String(v.as_str().unwrap_or("").to_string())),
            ),
            ColumnType::Set => Value::Ext(
                "Set",
                Box::new(Value::String(v.as_str().unwrap_or("").to_string())),
            ),
            //bytes ,see https://dev.mysql.com/doc/internals/en/x-protocol-messages-messages.html
            ColumnType::Geometry => Value::Ext(
                "Geometry",
                Box::new(Value::Binary(v.as_bytes().unwrap_or_default().to_vec())),
            ),
        })
    }
}
