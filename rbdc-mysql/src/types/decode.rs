use crate::protocol::text::ColumnType;
use crate::value::{MySqlValue, MySqlValueFormat};
use byteorder::{ByteOrder, LittleEndian};
use bytes::Buf;
use fastdate::{Date, DateTime};
use rbdc::Error;
use rbs::Value;
use std::str::FromStr;

impl From<MySqlValue> for Value {
    fn from(v: MySqlValue) -> Self {
        match v.type_info().r#type {
            ColumnType::Tiny => Value::U64(uint_decode(v).unwrap_or_default()),
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
                    let mut s = decode_timestamp(v).unwrap_or_default();
                    let date = DateTime::from_str(&s).unwrap();
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
            let date = decode_date_buf(&buf[1..])?.to_string();
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
            let date = decode_date_buf(&buf[1..])?.to_string();
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

fn decode_date_buf(buf: &[u8]) -> Result<Date, Error> {
    if buf.is_empty() {
        // zero buffer means a zero date (null)
        return  Ok(Date{
            year: 0000,
            mon: 00,
            day: 00,
        });
    }
    Ok(Date{
        year: LittleEndian::read_u16(buf),
        mon: buf[2] as u8,
        day: buf[3] as u8,
    })
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
