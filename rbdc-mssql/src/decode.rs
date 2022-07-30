use std::str::EncodeUtf16;
use byteorder::{ByteOrder, LittleEndian};
use rust_decimal::Decimal;
use tiberius::{ColumnData, ColumnType, FromSql, FromSqlOwned, IntoRow};
use tiberius::numeric::BigDecimal;
use rbs::Value;

pub trait Decode {
    fn decode(row: &tiberius::Row, i: usize, t: ColumnType) -> Value;
}

impl Decode for Value {
    fn decode(row: &tiberius::Row, i: usize, t: ColumnType) -> Value {
        let data: &[u8] = row.get(i).unwrap();
        match t {
            ColumnType::Null => { Value::Null }
            ColumnType::Bit => {
                Value::Binary(data.to_owned())
            }
            ColumnType::Int1 => {
                {
                    Value::I32({
                        if data.len() > 0 {
                            data[0] as i32
                        } else {
                            0
                        }
                    })
                }
            }
            ColumnType::Int2 => {
                Value::I32(LittleEndian::read_i16(data) as i32)
            }
            ColumnType::Int4 => { Value::I32(LittleEndian::read_i32(data)) }
            ColumnType::Int8 => { Value::I64(LittleEndian::read_i64(data)) }
            ColumnType::Datetime4 => {
                let v: chrono::NaiveDateTime = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::Float4 => {
                Value::F32(LittleEndian::read_f32(data))
            }
            ColumnType::Float8 => {
                Value::F64(LittleEndian::read_f64(data))
            }
            ColumnType::Money => {
                let v: BigDecimal = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Decimal")
            }
            ColumnType::Datetime => {
                let v: chrono::NaiveDateTime = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::Money4 => {
                let v: Decimal = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Decimal")
            }
            ColumnType::Guid => { Value::Binary(data.to_owned()).into_ext("Guid") }
            ColumnType::Intn => { Value::Binary(data.to_owned()).into_ext("Intn") }
            ColumnType::Bitn => { Value::Binary(data.to_owned()).into_ext("Bitn") }
            ColumnType::Decimaln => { Value::Binary(data.to_owned()) }
            ColumnType::Numericn => {
                let v: BigDecimal = row.get(i).unwrap();
                Value::String(v.to_string()).into_ext("Decimal")
            }
            ColumnType::Floatn => { Value::Binary(data.to_owned()) }
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
                Value::Binary(data.to_owned()).into_ext("DatetimeOffsetn")
            }
            ColumnType::BigVarBin => { Value::Binary(data.to_owned()).into_ext("BigVarBin") }
            ColumnType::BigVarChar => {
                let v: &str = row.get(i).unwrap();
                Value::String(v.to_string())
            }
            ColumnType::BigBinary => { Value::Binary(data.to_owned()).into_ext("BigBinary") }
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
            ColumnType::Xml => { Value::Binary(data.to_owned()).into_ext("Xml") }
            ColumnType::Udt => { Value::Binary(data.to_owned()).into_ext("Udt") }
            ColumnType::Text => {
                let v: &str = row.get(i).unwrap();
                Value::String(v.to_string())
            }
            ColumnType::Image => { Value::Binary(data.to_owned()).into_ext("Image") }
            ColumnType::NText => {
                let v: &str = row.get(i).unwrap();
                Value::String(v.to_string())
            }
            ColumnType::SSVariant => { Value::Binary(data.to_owned()).into_ext("SSVariant") }
        }
    }
}