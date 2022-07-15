use crate::protocol;
use crate::protocol::text::ColumnType;
use crate::result_set::MySqlColumn;
use crate::value::{MySqlValue, MySqlValueFormat, MySqlValueRef};
use byteorder::{ByteOrder, LittleEndian};
use bytes::Buf;
use rbdc::db::{MetaData, Row};
use rbdc::error::Error;
use rbdc::ext::ustr::UStr;
use rbs::Value;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

/// Implementation of [`Row`] for MySQL.
#[derive(Debug)]
pub struct MySqlRow {
    pub row: protocol::Row,
    pub format: MySqlValueFormat,
    pub columns: Arc<Vec<MySqlColumn>>,
    pub column_names: Arc<HashMap<UStr, usize>>,
}

pub trait Index {
    fn columns(&self) -> &[MySqlColumn];

    fn try_get_raw(&self, index: usize) -> Result<MySqlValueRef<'_>, Error>;
}

impl Index for MySqlRow {
    fn columns(&self) -> &[MySqlColumn] {
        &self.columns
    }

    fn try_get_raw(&self, index: usize) -> Result<MySqlValueRef<'_>, Error> {
        let column: &MySqlColumn = &self.columns[index];
        let value = self.row.get(index as usize);

        Ok(MySqlValueRef {
            format: self.format,
            row: Some(&self.row.storage),
            type_info: column.type_info.clone(),
            value,
        })
    }
}

impl MetaData for MySqlRow {
    fn column_len(&self) -> usize {
        self.column_names.len()
    }

    fn column_name(&self, i: usize) -> String {
        for (s, idx) in self.column_names.deref() {
            if idx.eq(&i) {
                return s.to_string();
            }
        }
        return String::new();
    }

    fn column_type(&self, i: usize) -> String {
        match self.columns.get(i) {
            None => String::new(),
            Some(v) => format!("{:?}", v.type_info.r#type),
        }
    }
}

impl Row for MySqlRow {
    fn meta_data(&self) -> &dyn MetaData {
        self
    }

    fn get(&self, i: usize) -> Option<Value> {
        match self.try_get_raw(i) {
            Err(_) => None,
            Ok(v) => Some(Value::from(v)),
        }
    }
}

impl From<MySqlValueRef<'_>> for Value {
    fn from(v: MySqlValueRef<'_>) -> Self {
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
            ColumnType::Bit => {
                todo!()
            }
            ColumnType::Json => {
                todo!()
            }
            ColumnType::NewDecimal => {
                todo!()
            }
            ColumnType::Enum => {
                todo!()
            }
            ColumnType::Set => {
                todo!()
            }
            ColumnType::TinyBlob => {
                todo!()
            }
            ColumnType::MediumBlob => {
                todo!()
            }
            ColumnType::LongBlob => {
                todo!()
            }
            ColumnType::Blob => {
                todo!()
            }
            ColumnType::VarString => Value::String(v.as_str().unwrap_or_default().to_string()),
            ColumnType::String => Value::String(v.as_str().unwrap_or_default().to_string()),
            ColumnType::Geometry => {
                todo!()
            }
        }
    }
}

fn uint_decode(value: MySqlValueRef<'_>) -> Result<u64, Error> {
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

fn int_decode(value: MySqlValueRef<'_>) -> Result<i64, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.parse().unwrap_or_default(),
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            LittleEndian::read_int(buf, buf.len())
        }
    })
}

fn f32_decode(value: MySqlValueRef<'_>) -> Result<f32, Error> {
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

fn f64_decode(value: MySqlValueRef<'_>) -> Result<f64, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Binary => LittleEndian::read_f64(value.as_bytes()?),
        MySqlValueFormat::Text => value.as_str()?.parse().unwrap_or_default(),
    })
}

fn decode_timestamp(value: MySqlValueRef<'_>) -> Result<String, Error> {
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

fn decode_year(value: MySqlValueRef<'_>) -> Result<String, Error> {
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

fn decode_date(value: MySqlValueRef<'_>) -> Result<String, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.to_string(),
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            let len = buf[0];
            let date = decode_date_buf(&buf[1..])?;
            // let dt = if len > 4 {
            //     decode_time_buf(len - 4, &buf[5..])?
            // } else {
            //     "00:00:00".to_string()
            // };
            date
        }
    })
}

fn decode_time(value: MySqlValueRef<'_>) -> Result<String, Error> {
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

fn decode_bool(value: MySqlValueRef<'_>) -> Result<bool, Error> {
    Ok(int_decode(value)? != 0)
}
