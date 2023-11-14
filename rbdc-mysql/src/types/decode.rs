use crate::protocol::text::ColumnType;
use crate::value::{MySqlValue, MySqlValueFormat};
use byteorder::{ByteOrder, LittleEndian};
use bytes::Buf;
use fastdate::Date;
use rbdc::Error;

pub(crate) fn uint_decode(value: MySqlValue) -> Result<u64, Error> {
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

pub(crate) fn int_decode(value: MySqlValue) -> Result<i64, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.parse().unwrap_or_default(),
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            LittleEndian::read_int(buf, buf.len())
        }
    })
}

pub(crate) fn f32_decode(value: MySqlValue) -> Result<f32, Error> {
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

pub(crate) fn f64_decode(value: MySqlValue) -> Result<f64, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Binary => LittleEndian::read_f64(value.as_bytes()?),
        MySqlValueFormat::Text => value.as_str()?.parse().unwrap_or_default(),
    })
}

pub(crate) fn decode_timestamp(value: MySqlValue) -> Result<String, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => {
            let mut v = value.as_str()?.to_string();
            if !v.ends_with("Z") {
                v.push_str("Z");
            }
            v
        }
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            let len = buf[0];
            let date = decode_date_buf(&buf[1..])?.to_string();
            let dt = if len > 4 {
                decode_time_buf(len - 4, &buf[5..])?
            } else {
                "00:00:00".to_string()
            };
            date + " " + &dt + "Z"
        }
    })
}

pub(crate) fn decode_year(value: MySqlValue) -> Result<String, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.to_string(),
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            let date = decode_year_buf(&buf[1..])?;
            date
        }
    })
}

pub(crate) fn decode_date(value: MySqlValue) -> Result<String, Error> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.to_string(),
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;
            let date = decode_date_buf(&buf[1..])?.to_string();
            date
        }
    })
}

pub(crate) fn decode_time(value: MySqlValue) -> Result<String, Error> {
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

pub(crate) fn decode_date_buf(buf: &[u8]) -> Result<Date, Error> {
    if buf.is_empty() {
        // zero buffer means a zero date (null)
        return Ok(Date {
            year: 0000,
            mon: 00,
            day: 00,
        });
    }
    Ok(Date {
        year: LittleEndian::read_u16(buf) as i32,
        mon: buf[2] as u8,
        day: buf[3] as u8,
    })
}

pub(crate) fn decode_year_buf(buf: &[u8]) -> Result<String, Error> {
    if buf.is_empty() {
        // zero buffer means a zero date (null)
        return Ok("".to_string());
    }
    Ok(format!("{:0>4}", LittleEndian::read_u16(buf) as i32,))
}

pub(crate) fn decode_time_buf(_: u8, mut buf: &[u8]) -> Result<String, Error> {
    let hour = buf.get_u8();
    let minute = buf.get_u8();
    let seconds = buf.get_u8();
    let milliseconds = if buf.len() >= 4 {
        buf.get_u32_le()
    } else {
        0 // 如果没有足够的字节表示毫秒值，将其设置为默认值
    };
    Ok(format!(
        "{:0>2}:{:0>2}:{:0>2}.{:0>6}",
        hour, minute, seconds, milliseconds
    ))
}

pub(crate) fn decode_bool(value: MySqlValue) -> Result<bool, Error> {
    Ok(int_decode(value)? != 0)
}
