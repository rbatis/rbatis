use crate::io::MySqlBufMutExt;
use crate::protocol::text::ColumnType;
use crate::result_set::MySqlTypeInfo;
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
                buf.put_bytes_lenenc(v);
                MySqlTypeInfo::from_type(ColumnType::Blob)
            }
            Value::Array(v) => MySqlTypeInfo::null(),
            Value::Map(m) => {
                if m.len() == 1 {
                    // match m[0].0.as_str() {
                    //     None => MySqlTypeInfo::null(),
                    //     Some(s) => match s.as_ref() {
                    //         "decimal" => {
                    //             buf.put_bytes_lenenc(m[0].1.into_string().into_bytes());
                    //             MySqlTypeInfo::from_type(ColumnType::NewDecimal)
                    //         }
                    //         "year" => {}
                    //         "date" => {}
                    //         "time" => {}
                    //         "timestamp" => {}
                    //         "datetime" => {}
                    //         "json" => {}
                    //         "new_decimal" => {}
                    //         "enum" => {}
                    //         "set" => {}
                    //         "geometry" => {}
                    //         _ => {}
                    //     },
                    // }
                    todo!()
                } else {
                    MySqlTypeInfo::null()
                }
            }
            Value::Ext(_, _) => MySqlTypeInfo::null(),
        };
    }
}

fn encode_date(buf: &mut Vec<u8>, year: u16, month: u8, day: u8) {
    // MySQL supports years from 1000 - 9999
    buf.extend_from_slice(&year.to_le_bytes());
    buf.push(month as u8);
    buf.push(day as u8);
}

fn encode_time(buf: &mut Vec<u8>, hour: u8, minute: u8, second: u8, micros: Option<u32>) {
    buf.push(hour as u8);
    buf.push(minute as u8);
    buf.push(second as u8);
    if let Some(micros) = micros {
        buf.extend(micros.to_le_bytes());
    }
}
