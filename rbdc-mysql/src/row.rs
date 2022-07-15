use crate::meta_data::MysqlMetaData;
use crate::protocol;
use crate::protocol::text::ColumnType;
use crate::result_set::{MySqlColumn, MySqlTypeInfo};
use crate::value::{MySqlValue, MySqlValueFormat, MySqlValueRef};
use byteorder::{ByteOrder, LittleEndian};
use bytes::Buf;
use rbdc::db::{MetaData, Row};
use rbdc::error::Error;
use rbdc::ext::ustr::UStr;
use rbs::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Implementation of [`Row`] for MySQL.
#[derive(Debug)]
pub struct MySqlRow {
    pub row: protocol::Row,
    pub format: MySqlValueFormat,
    pub columns: Arc<Vec<MySqlColumn>>,
    pub column_names: Arc<HashMap<UStr, (usize, MySqlTypeInfo)>>,
}

impl MySqlRow {
    pub fn columns(&self) -> &[MySqlColumn] {
        &self.columns
    }

    pub fn try_get(&self, index: usize) -> Result<MySqlValueRef<'_>, Error> {
        let column: &MySqlColumn = &self.columns[index];
        let value = self.row.get(index as usize);
        Ok(MySqlValueRef {
            format: self.format,
            type_info: column.type_info.clone(),
            value,
        })
    }

    pub fn try_take(&mut self, index: usize) -> Option<MySqlValue> {
        let column: &MySqlColumn = &self.columns[index];
        let value = self.row.take(index)?;
        Some(MySqlValue {
            value: Some(value),
            type_info: column.type_info.clone(),
            format: self.format,
        })
    }
}

impl Row for MySqlRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        Box::new(MysqlMetaData {
            inner: self.column_names.clone(),
        })
    }

    fn get(&mut self, i: usize) -> Option<Value> {
        match self.try_take(i) {
            None => None,
            Some(v) => Some(Value::from(v)),
        }
    }
}

impl From<MySqlValue> for Value {
    fn from(v: MySqlValue) -> Self {
        match v.type_info.r#type {
            ColumnType::Decimal => Value::Map(vec![(
                Value::String("t_decimal".to_string()),
                Value::String(v.as_str().unwrap_or("0").to_string()),
            )]),
            ColumnType::Tiny => Value::U64(uint_decode(v).unwrap_or_default()),
            ColumnType::Short => Value::I32(int_decode(v).unwrap_or_default() as i32),
            ColumnType::Long => Value::I64(int_decode(v).unwrap_or_default()),
            ColumnType::Float => Value::F32(f32_decode(v).unwrap_or_default()),
            ColumnType::Double => Value::F64(f64_decode(v).unwrap_or_default()),
            ColumnType::Null => Value::Nil,
            ColumnType::Timestamp => Value::Map(vec![(
                Value::String("t_timestamp".to_string()),
                Value::String(decode_timestamp(v).unwrap_or_default()),
            )]),
            ColumnType::LongLong => Value::Bool(decode_bool(v).unwrap_or_default()),
            ColumnType::Int24 => Value::I32(int_decode(v).unwrap_or_default() as i32),
            ColumnType::Date => Value::Map(vec![(
                Value::String("t_date".to_string()),
                Value::String(decode_date(v).unwrap_or_default()),
            )]),
            ColumnType::Time => Value::Map(vec![(
                Value::String("t_time".to_string()),
                Value::String(decode_time(v).unwrap_or_default()),
            )]),
            ColumnType::Datetime => Value::Map(vec![(
                Value::String("t_datetime".to_string()),
                Value::String(decode_timestamp(v).unwrap_or_default()),
            )]),
            ColumnType::Year => Value::Map(vec![(
                Value::String("t_year".to_string()),
                Value::String(decode_year(v).unwrap_or_default()),
            )]),
            ColumnType::VarChar => Value::String(v.as_str().unwrap_or_default().to_string()),
            ColumnType::Bit => Value::U64(uint_decode(v).unwrap_or_default()),
            ColumnType::Json => Value::Map(vec![(
                Value::String("t_json".to_string()),
                Value::String(v.as_str().unwrap_or_default().to_string()),
            )]),
            ColumnType::NewDecimal => Value::Map(vec![(
                Value::String("t_decimal".to_string()),
                Value::String(v.as_str().unwrap_or("0").to_string()),
            )]),
            ColumnType::Enum => Value::Map(vec![(
                Value::String("t_enum".to_string()),
                Value::String(v.as_str().unwrap_or("").to_string()),
            )]),
            ColumnType::Set => Value::Map(vec![(
                Value::String("t_set".to_string()),
                Value::String(v.as_str().unwrap_or("").to_string()),
            )]),
            ColumnType::TinyBlob => Value::Binary(v.as_bytes().unwrap_or_default().to_vec()),
            ColumnType::MediumBlob => Value::Binary(v.as_bytes().unwrap_or_default().to_vec()),
            ColumnType::LongBlob => Value::Binary(v.as_bytes().unwrap_or_default().to_vec()),
            ColumnType::Blob => Value::Binary(v.as_bytes().unwrap_or_default().to_vec()),
            ColumnType::VarString => Value::String(v.as_str().unwrap_or_default().to_string()),
            ColumnType::String => Value::String(v.as_str().unwrap_or_default().to_string()),
            ColumnType::Geometry => {
                todo!()
            }
        }
    }
}

fn uint_decode(value: MySqlValue) -> Result<u64, Error> {
    if value.type_info.r#type == ColumnType::Bit {
        // NOTE: Regardless of the value format, there is raw binary data here
        let buf = value.as_bytes()?;
        let mut value: u64 = 0;
        for b in buf {
            value = (*b as u64) | (value << 8);
        }
        return Ok(value);
    }

    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.parse().unwrap_or_default(),

        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            LittleEndian::read_uint(buf, buf.len())
        }
    })
}

fn int_decode(value: MySqlValue) -> Result<i64, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.parse().unwrap_or_default(),
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            LittleEndian::read_int(buf, buf.len())
        }
    })
}

fn f32_decode(value: MySqlValue) -> Result<f32, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;

            if buf.len() == 8 {
                // MySQL can return 8-byte DOUBLE values for a FLOAT
                // We take and truncate to f32 as that's the same behavior as *in* MySQL
                LittleEndian::read_f64(buf) as f32
            } else {
                LittleEndian::read_f32(buf)
            }
        }

        MySqlValueFormat::Text => value.as_str()?.parse().unwrap_or_default(),
    })
}

fn f64_decode(value: MySqlValue) -> Result<f64, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Binary => LittleEndian::read_f64(value.as_bytes()?),
        MySqlValueFormat::Text => value.as_str()?.parse().unwrap_or_default(),
    })
}

fn decode_timestamp(value: MySqlValue) -> Result<String, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.to_string(),
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            let len = buf[0];
            let date = decode_date_buf(&buf[1..])?;
            let dt = if len > 4 {
                decode_time_buf(len - 4, &buf[5..])?
            } else {
                "00:00:00".to_string()
            };
            date + " " + &dt
        }
    })
}

fn decode_year(value: MySqlValue) -> Result<String, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.to_string(),
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            let len = buf[0];
            let date = decode_year_buf(&buf[1..])?;
            date
        }
    })
}

fn decode_date(value: MySqlValue) -> Result<String, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.to_string(),
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            let len = buf[0];
            let date = decode_date_buf(&buf[1..])?;
            date
        }
    })
}

fn decode_time(value: MySqlValue) -> Result<String, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.to_string(),
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            let len = buf[0];
            let dt = if len > 4 {
                decode_time_buf(len - 4, &buf[5..])?
            } else {
                "00:00:00".to_string()
            };
            dt
        }
    })
}

fn decode_date_buf(buf: &[u8]) -> Result<String, Error> {
    if buf.is_empty() {
        // zero buffer means a zero date (null)
        return Ok("".to_string());
    }
    Ok(format!(
        "{:4}-{:2}-{:2}",
        LittleEndian::read_u16(buf) as i32,
        buf[2] as u8,
        buf[3] as u8,
    ))
}

fn decode_year_buf(buf: &[u8]) -> Result<String, Error> {
    if buf.is_empty() {
        // zero buffer means a zero date (null)
        return Ok("".to_string());
    }
    Ok(format!("{:4}", LittleEndian::read_u16(buf) as i32,))
}

fn decode_time_buf(len: u8, mut buf: &[u8]) -> Result<String, Error> {
    let hour = buf.get_u8();
    let minute = buf.get_u8();
    let seconds = buf.get_u8();
    Ok(format!("{:2}:{:2}:{:2}", hour, minute, seconds))
}

fn decode_bool(value: MySqlValue) -> Result<bool, Error> {
    Ok(int_decode(value)? != 0)
}
