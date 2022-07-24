use crate::io::MySqlBufMutExt;
use crate::protocol::text::ColumnType;
use crate::result_set::MySqlTypeInfo;
use bytes::BufMut;
use fastdate::DateTime;
use rbs::Value;

impl From<(Value, &mut Vec<u8>)> for MySqlTypeInfo {
    fn from((value, buf): (Value, &mut Vec<u8>)) -> Self {
        return match value {
            Value::Null => MySqlTypeInfo::null(),
            Value::Bool(v) => {
                buf.extend(&(v as i8).to_le_bytes());
                MySqlTypeInfo::from_type(ColumnType::Tiny)
            }
            Value::I32(v) => {
                buf.extend(v.to_le_bytes());
                MySqlTypeInfo::from_type(ColumnType::Long)
            }
            Value::I64(v) => {
                buf.extend(v.to_le_bytes());
                MySqlTypeInfo::from_type(ColumnType::LongLong)
            }
            Value::U32(v) => {
                buf.extend(v.to_le_bytes());
                MySqlTypeInfo::from_type(ColumnType::Long)
            }
            Value::U64(v) => {
                buf.extend(v.to_le_bytes());
                MySqlTypeInfo::from_type(ColumnType::LongLong)
            }
            Value::F32(v) => {
                buf.extend(v.to_le_bytes());
                MySqlTypeInfo::from_type(ColumnType::Float)
            }
            Value::F64(v) => {
                buf.extend(v.to_le_bytes());
                MySqlTypeInfo::from_type(ColumnType::Double)
            }
            Value::String(v) => {
                buf.put_str_lenenc(&v);
                MySqlTypeInfo::from_type(ColumnType::VarChar)
            }
            Value::Binary(v) => {
                // "geometry" is bytes
                buf.put_bytes_lenenc(v);
                MySqlTypeInfo::from_type(ColumnType::Blob)
            }
            Value::Array(v) => MySqlTypeInfo::null(),
            Value::Map(m) => MySqlTypeInfo::null(),
            Value::Ext(ext_type, v) => {
                match ext_type {
                    "uuid" => {
                        //uuid -> string
                        buf.put_bytes_lenenc(v.into_string().unwrap_or_default().into_bytes());
                        MySqlTypeInfo::from_type(ColumnType::VarChar)
                    }
                    //decimal = 12345678
                    "decimal" => {
                        let mut bytes = v.into_bytes().unwrap_or_default();
                        buf.put_bytes_lenenc(bytes);
                        MySqlTypeInfo::from_type(ColumnType::NewDecimal)
                    }
                    //year = "1993"
                    "year" => {
                        let year = v.into_string().unwrap_or_default();
                        buf.push(2);
                        encode_year(buf, rbdc::time::parse_year(&year));
                        MySqlTypeInfo::from_type(ColumnType::Year)
                    }
                    //Date = "1993-02-06"
                    "date" => {
                        let s = v.into_string().unwrap_or_default();
                        if s.len() == 10 {
                            let date = rbdc::time::parse_date(&s);
                            buf.push(4);
                            encode_date(buf, date.year, date.mon, date.day);
                            MySqlTypeInfo::from_type(ColumnType::Date)
                        } else {
                            return MySqlTypeInfo::null();
                        }
                    }
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "time" => {
                        let c = v.into_string().unwrap_or_default();
                        if c.len() >= 8 {
                            let time = rbdc::time::parse_time(&c);
                            let size = time_size_hint(time.ms);
                            buf.push(size as u8);
                            encode_time(buf, time.hour, time.min, time.sec, time.ms);
                            MySqlTypeInfo::from_type(ColumnType::Time)
                        } else {
                            return MySqlTypeInfo::null();
                        }
                    }
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "timestamp" => {
                        //datetime=5byte
                        let c = v.as_str().unwrap_or_default().to_string();
                        let datetime =
                            DateTime::from_timestamp_millis(c.parse().unwrap_or_default());
                        let size = date_time_size_hint(
                            datetime.hour,
                            datetime.min,
                            datetime.sec,
                            datetime.micro,
                        );
                        buf.push(size as u8);
                        encode_date(buf, datetime.year, datetime.mon, datetime.day);
                        if size > 4 {
                            encode_time(
                                buf,
                                datetime.hour,
                                datetime.min,
                                datetime.sec,
                                datetime.micro,
                            );
                        }
                        MySqlTypeInfo::from_type(ColumnType::Timestamp)
                    }
                    "datetime" => {
                        let c = v.into_string().unwrap_or_default();
                        let datetime =
                            DateTime::from_timestamp_millis(c.parse().unwrap_or_default());
                        let size = date_time_size_hint(
                            datetime.hour,
                            datetime.min,
                            datetime.sec,
                            datetime.micro,
                        );
                        buf.push(size as u8);
                        encode_date(buf, datetime.year, datetime.mon, datetime.day);
                        if size > 4 {
                            encode_time(
                                buf,
                                datetime.hour,
                                datetime.min,
                                datetime.sec,
                                datetime.micro,
                            );
                        }
                        MySqlTypeInfo::from_type(ColumnType::Datetime)
                    }
                    "json" => {
                        buf.put_bytes_lenenc(v.into_bytes().unwrap_or_default());
                        MySqlTypeInfo::from_type(ColumnType::Json)
                    }
                    "enum" => {
                        buf.put_bytes_lenenc(v.into_bytes().unwrap_or_default());
                        MySqlTypeInfo::from_type(ColumnType::Enum)
                    }
                    "set" => {
                        buf.put_bytes_lenenc(v.into_bytes().unwrap_or_default());
                        MySqlTypeInfo::from_type(ColumnType::Set)
                    }
                    _ => {
                        MySqlTypeInfo::null()
                    }
                }
            }
        };
    }
}

fn time_size_hint(nano: u32) -> usize {
    if nano == 0 {
        // if micro_seconds is 0, length is 8 and micro_seconds is not sent
        9
    } else {
        // otherwise length is 12
        13
    }
}

fn date_time_size_hint(hour: u8, min: u8, sec: u8, nano: u32) -> usize {
    // to save space the packet can be compressed:
    match (hour, min, sec, nano) {
        // if hour, minutes, seconds and micro_seconds are all 0,
        // length is 4 and no other field is sent
        (0, 0, 0, 0) => 4,

        // if micro_seconds is 0, length is 7
        // and micro_seconds is not sent
        (_, _, _, 0) => 7,

        // otherwise length is 11
        (_, _, _, _) => 11,
    }
}

fn encode_year(buf: &mut Vec<u8>, year: u16) {
    // MySQL supports years from 1000 - 9999
    buf.extend_from_slice(&year.to_le_bytes());
}

fn encode_date(buf: &mut Vec<u8>, year: u16, month: u8, day: u8) {
    // MySQL supports years from 1000 - 9999
    buf.extend_from_slice(&year.to_le_bytes());
    buf.push(month as u8);
    buf.push(day as u8);
}

fn encode_time(buf: &mut Vec<u8>, hour: u8, minute: u8, second: u8, ms: u32) {
    buf.push(hour as u8);
    buf.push(minute as u8);
    buf.push(second as u8);
    if ms != 0 {
        buf.extend(ms.to_le_bytes());
    }
}
