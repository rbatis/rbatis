use std::str::EncodeUtf16;
use byteorder::{ByteOrder, LittleEndian};
use chrono::Utc;
use rust_decimal::Decimal;
use tiberius::{ColumnData, ColumnType, FromSql, FromSqlOwned, IntoRow};
use tiberius::numeric::BigDecimal;
use tiberius::xml::XmlData;
use rbs::Value;

pub trait Decode {
    fn decode(row: &tiberius::Row, i: usize, t: ColumnType) -> Value;
}

impl Decode for Value {
    fn decode(row: &tiberius::Row, i: usize, t: ColumnType) -> Value {
        match t {
            ColumnType::Null => { Value::Null }
            ColumnType::Bit => {
                let data: bool = row.get(i).unwrap();
                Value::I32(data as i32)
            }
            ColumnType::Int1 => {
                {
                    let data: u8 = row.get(i).unwrap();
                    Value::I32(data as i32)
                }
            }
            ColumnType::Int2 => {
                let data: i16 = row.get(i).unwrap();
                Value::I32(data as i32)
            }
            ColumnType::Int4 => {
                let data: i32 = row.get(i).unwrap();
                Value::I32(data)
            }
            ColumnType::Int8 => {
                let data: i64 = row.get(i).unwrap();
                Value::I64(data)
            }
            ColumnType::Datetime4 => {
                let v: chrono::NaiveDateTime = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::Float4 => {
                let data: f32 = row.get(i).unwrap();
                Value::F32(data)
            }
            ColumnType::Float8 => {
                let data: f64 = row.get(i).unwrap();
                Value::F64(data)
            }
            ColumnType::Money => {
                let v: f64 = row.get(i).unwrap();
                Value::F64(v)
            }
            ColumnType::Datetime => {
                let v: chrono::NaiveDateTime = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::Money4 => {
                let v: f32 = row.get(i).unwrap();
                Value::F32(v)
            }
            ColumnType::Guid => {
                let data: uuid::Uuid = row.get(i).unwrap();
                Value::String(data.to_string()).into_ext("Guid")
            }
            ColumnType::Intn => {
                let data: i32 = row.get(i).unwrap();
                Value::I32(data.to_owned())
            }
            ColumnType::Bitn => {
                let data: bool = row.get(i).unwrap();
                Value::Bool(data)
            }
            ColumnType::Decimaln => {
                let data: BigDecimal = row.get(i).unwrap();
                Value::String(data.to_string()).into_ext("Decimal")
            }
            ColumnType::Numericn => {
                let v: BigDecimal = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Decimal")
            }
            ColumnType::Floatn => {
                let data: f64 = row.get(i).unwrap();
                Value::F64(data)
            }
            ColumnType::Datetimen => {
                let v: chrono::NaiveDateTime = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::Daten => {
                let v: chrono::NaiveDate = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Date")
            }
            ColumnType::Timen => {
                let v: chrono::NaiveTime = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::Datetime2 => {
                let v: chrono::NaiveDateTime = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::DatetimeOffsetn => {
                let data: chrono::DateTime<Utc> = row.get(i).unwrap();
                Value::String(data.to_string()).into_ext("Datetime")
            }
            ColumnType::BigVarBin => {
                let data: &[u8] = row.get(i).unwrap();
                Value::Binary(data.to_owned()).into_ext("BigVarBin")
            }
            ColumnType::BigVarChar => {
                let v: &str = row.get(i).unwrap();
                Value::String(v.to_string())
            }
            ColumnType::BigBinary => {
                let data: &[u8] = row.get(i).unwrap();
                Value::Binary(data.to_owned()).into_ext("BigBinary")
            }
            ColumnType::BigChar => {
                let v: &str = row.get(i).unwrap();
                Value::String(v.to_string())
            }
            ColumnType::NVarchar => {
                let v: &str = row.get(i).unwrap();
                Value::String(v.to_string())
            }
            ColumnType::NChar => {
                let v: &str = row.get(i).unwrap();
                Value::String(v.to_string())
            }
            ColumnType::Xml => {
                let data: XmlData = row.get(i).unwrap();
                Value::String(data.to_string()).into_ext("Xml")
            }
            ColumnType::Udt => {
                let data: &[u8] = row.get(i).unwrap();
                Value::Binary(data.to_owned()).into_ext("Udt")
            }
            ColumnType::Text => {
                let v: &str = row.get(i).unwrap();
                Value::String(v.to_string())
            }
            ColumnType::Image => {
                let data: &[u8] = row.get(i).unwrap();
                Value::Binary(data.to_owned()).into_ext("Image")
            }
            ColumnType::NText => {
                let v: &str = row.get(i).unwrap();
                Value::String(v.to_string())
            }
            ColumnType::SSVariant => {
                let data: &[u8] = row.get(i).unwrap();
                Value::Binary(data.to_owned()).into_ext("SSVariant")
            }
        }
    }
}