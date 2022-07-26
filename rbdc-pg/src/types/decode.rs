use crate::type_info::{PgType, PgTypeKind};
use crate::types::Oid;
use crate::value::{PgValue, PgValueFormat, PgValueRef};
use byteorder::{BigEndian, ByteOrder};
use fastdate::{Date, DateTime};
use rbdc::Error;
use rbs::Value;
use std::str::FromStr;
use std::time::Duration;

pub trait Decode: Sized {
    /// Decode a new value of this type using a raw value from the database.
    fn decode(value: PgValue) -> Result<Self, Error>;
}

impl Decode for Value {
    fn decode(arg: PgValue) -> Result<Self, Error> {
        Ok(match arg.type_info.0 {
            PgType::Bool => Value::Bool(Decode::decode(arg)?),
            PgType::Bytea => Value::U32({let i:i8=Decode::decode(arg)?;i} as u32),
            PgType::Char => Value::String(Decode::decode(arg)?),
            PgType::Name => Value::String(Decode::decode(arg)?),
            PgType::Int8 => Value::I32(Decode::decode(arg)?),
            PgType::Int2 => Value::I32(Decode::decode(arg)?),
            PgType::Int4 => Value::I32(Decode::decode(arg)?),
            PgType::Text => Value::String(Decode::decode(arg)?),
            PgType::Oid => Value::Ext("Oid", Box::new(Value::U32(Decode::decode(arg)?))),
            PgType::Json => Value::Ext("Json", Box::new(Value::String(
                crate::types::json::Json::decode(arg)
                    .unwrap_or_default()
                    .json,
            ))),
            PgType::Point => {
                Value::Ext("Point", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Lseg => {
                Value::Ext("Lseg", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Path => {
                Value::Ext("Path", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Box => {
                Value::Ext("Box", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Polygon => {
                Value::Ext("Polygon", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Line => {
                Value::Ext("Line", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Cidr => {
                Value::Ext("Cidr", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }

            PgType::Float4 => Value::F32(Decode::decode(arg)?),
            PgType::Float8 => Value::F32(Decode::decode(arg)?),
            PgType::Unknown => Value::Null,
            PgType::Circle => {
                Value::Ext("Circle", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Macaddr8 => {
                Value::Ext("Macaddr8", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Macaddr => {
                Value::Ext("Macaddr", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Inet => {
                Value::Ext("Inet", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Bpchar => {
                Value::Ext("Bpchar", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Varchar => Value::String(Decode::decode(arg)?),
            PgType::Date => {
                todo!()
            }
            PgType::Time => {
                todo!()
            }
            PgType::Timestamp => Value::String({
                let fast_date: DateTime = Decode::decode(arg)?;
                fast_date.to_string()
            }),
            PgType::Timestamptz => Value::String({
                let fast_date: DateTime = Decode::decode(arg)?;
                fast_date.to_string()
            }),
            PgType::Interval => {
                Value::Ext("Interval", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Timetz => {
                todo!()
            }
            PgType::Bit => {
                todo!()
            }
            PgType::Varbit => {
                todo!()
            }
            PgType::Numeric => {
                todo!()
            }
            PgType::Record => {
                Value::Ext("Record", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Uuid => {
                todo!()
            }
            PgType::Jsonb => {
                todo!()
            }
            PgType::Int4Range => {
                Value::Ext("Int4Range", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::NumRange => {
                Value::Ext("NumRange", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::TsRange => {
                Value::Ext("TsRange", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::TstzRange => {
                Value::Ext("TstzRange", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::DateRange => {
                Value::Ext("DateRange", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Int8Range => {
                Value::Ext("Int8Range", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Jsonpath => {
                Value::Ext("Jsonpath", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Money => {
                todo!()
            }
            PgType::Void => {
                todo!()
            }
            PgType::Custom(_) => {
                Value::Ext("Custom", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::DeclareWithName(_) => {
                Value::Ext("DeclareWithName", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::DeclareWithOid(_) => {
                Value::Ext("DeclareWithOid", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::JsonArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::LineArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::CidrArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::CircleArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Macaddr8Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::BoolArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::ByteaArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::CharArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::NameArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Int2Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Int4Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TextArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::BpcharArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::VarcharArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Int8Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::PointArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::LsegArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::PathArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::BoxArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Float4Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Float8Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::PolygonArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::OidArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::MacaddrArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::InetArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TimestampArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::DateArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TimeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TimestamptzArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::IntervalArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::NumericArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TimetzArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::BitArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::VarbitArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::RecordArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::UuidArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::JsonbArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Int4RangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::NumRangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TsRangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TstzRangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::DateRangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Int8RangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::JsonpathArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::MoneyArray => {
                Value::Array(Decode::decode(arg)?)
            }
        })
    }
}

impl From<PgValue> for Value {
    fn from(arg: PgValue) -> Self {
        Decode::decode(arg).unwrap()
    }
}


impl Decode for String {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(value.as_str()?.to_owned())
    }
}

impl Decode for i64 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_i64(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for i32 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_i32(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for u32 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_u32(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for i16 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_i16(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for f64 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_f64(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for f32 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_f32(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for i8 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        // note: in the TEXT encoding, a value of "0" here is encoded as an empty string
        Ok(value.as_bytes()?.get(0).copied().unwrap_or_default() as i8)
    }
}

impl Decode for DateTime {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIMESTAMP is encoded as the microseconds since the epoch
                let epoch = DateTime {
                    micro: 0,
                    sec: 0,
                    min: 0,
                    hour: 0,
                    day: 1,
                    mon: 1,
                    year: 2000,
                };
                let us: i64 = Decode::decode(value)?;
                epoch + Duration::from_micros(us as u64)
            }
            PgValueFormat::Text => {
                //2022-07-22 05:22:22.123456+00
                let s = value.as_str()?;
                let bytes = s.as_bytes();
                if bytes[bytes.len() - 3] == '+' as u8 {
                    //have zone
                    let mut dt = DateTime::from_str(&s[0..s.len() - 3])
                        .map_err(|e| Error::from(e.to_string()))?;
                    let hour: i32 = s[s.len() - 2..s.len()].parse().unwrap_or_default();
                    dt = dt + Duration::from_secs((hour * 3600) as u64);
                    dt
                } else {
                    DateTime::from_str(s).map_err(|e| Error::from(e.to_string()))?
                }
            }
        })
    }
}
