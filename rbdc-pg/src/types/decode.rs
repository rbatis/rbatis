use crate::type_info::PgType;
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

impl From<PgValue> for Value {
    fn from(arg: PgValue) -> Self {
        match arg.type_info.0 {
            PgType::Bool => Value::Bool(Decode::decode(arg).unwrap()),
            PgType::Bytea => Value::U32({
                let i:i8=Decode::decode(arg).unwrap();
                i
            } as u32),
            PgType::Char => Value::String(Decode::decode(arg).unwrap()),
            PgType::Name => Value::String(Decode::decode(arg).unwrap()),
            PgType::Int8 => Value::I32(Decode::decode(arg).unwrap()),
            PgType::Int2 => Value::I32(Decode::decode(arg).unwrap()),
            PgType::Int4 => Value::I32(Decode::decode(arg).unwrap()),
            PgType::Text => Value::String(Decode::decode(arg).unwrap()),
            PgType::Oid => Value::Ext("oid", Box::new(Value::U32(Decode::decode(arg).unwrap()))),
            PgType::Json => Value::String(
                crate::types::json::Json::decode(arg)
                    .unwrap_or_default()
                    .json,
            ),
            PgType::JsonArray => {
                Value::Ext("json_array", Box::new(Value::String(
                    crate::types::json::Json::decode(arg)
                        .unwrap_or_default()
                        .json,
                )))
            }
            PgType::Point => {
                Value::Ext("point", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Lseg => {
                Value::Ext("lseg", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Path => {
                Value::Ext("path", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Box => {
                Value::Ext("box", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Polygon => {
                Value::Ext("polygon", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Line => {
                Value::Ext("line", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::LineArray => {
                Value::Ext("line_array", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Cidr => {
                Value::Ext("cidr", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::CidrArray => {
                Value::Ext("cidr_array", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Float4 => Value::F32(Decode::decode(arg).unwrap()),
            PgType::Float8 => Value::F32(Decode::decode(arg).unwrap()),
            PgType::Unknown => Value::Null,
            PgType::Circle => {
                Value::Ext("circle", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::CircleArray => {
                Value::Ext("circle_array", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Macaddr8 => {
                Value::Ext("macaddr8", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Macaddr8Array => {
                Value::Ext("macaddr8array", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Macaddr => {
                Value::Ext("macaddr", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Inet => {
                Value::Ext("inet", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::BoolArray => {
                todo!()
            }
            PgType::ByteaArray => {
                todo!()
            }
            PgType::CharArray => {
                todo!()
            }
            PgType::NameArray => {
                Value::Ext("name_array", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Int2Array => {
                todo!()
            }
            PgType::Int4Array => {
                todo!()
            }
            PgType::TextArray => {
                todo!()
            }
            PgType::BpcharArray => {
                Value::Ext("bpchar_array", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::VarcharArray => {
                todo!()
            }
            PgType::Int8Array => {
                todo!()
            }
            PgType::PointArray => {
                Value::Ext("point_array", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::LsegArray => {
                Value::Ext("LsegArray", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::PathArray => {
                Value::Ext("PathArray", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::BoxArray => {
                Value::Ext("BoxArray", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Float4Array => {
                todo!()
            }
            PgType::Float8Array => {
                todo!()
            }
            PgType::PolygonArray => {
                Value::Ext("PolygonArray", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::OidArray => {
                todo!()
            }
            PgType::MacaddrArray => {
                Value::Ext("MacaddrArray", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::InetArray => {
                Value::Ext("InetArray", Box::new(Value::Binary({
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
            PgType::Varchar => Value::String(Decode::decode(arg).unwrap()),
            PgType::Date => {
                todo!()
            }
            PgType::Time => {
                todo!()
            }
            PgType::Timestamp => Value::String({
                let fast_date: DateTime = Decode::decode(arg).unwrap();
                fast_date.to_string()
            }),
            PgType::TimestampArray => {
                todo!()
            }
            PgType::DateArray => {
                todo!()
            }
            PgType::TimeArray => {
                todo!()
            }
            PgType::Timestamptz => Value::String({
                let fast_date: DateTime = Decode::decode(arg).unwrap();
                fast_date.to_string()
            }),
            PgType::TimestamptzArray => {
                todo!()
            }
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
            PgType::IntervalArray => {
                Value::Ext("IntervalArray", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::NumericArray => {
                todo!()
            }
            PgType::Timetz => {
                todo!()
            }
            PgType::TimetzArray => {
                todo!()
            }
            PgType::Bit => {
                todo!()
            }
            PgType::BitArray => {
                todo!()
            }
            PgType::Varbit => {
                todo!()
            }
            PgType::VarbitArray => {
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
            PgType::RecordArray => {
                Value::Ext("RecordArray", Box::new(Value::Binary({
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
            PgType::UuidArray => {
                todo!()
            }
            PgType::Jsonb => {
                todo!()
            }
            PgType::JsonbArray => {
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
            PgType::Int4RangeArray => {
                Value::Ext("Int4RangeArray", Box::new(Value::Binary({
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
            PgType::NumRangeArray => {
                Value::Ext("NumRangeArray", Box::new(Value::Binary({
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
            PgType::TsRangeArray => {
                Value::Ext("TsRangeArray", Box::new(Value::Binary({
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
            PgType::TstzRangeArray => {
                Value::Ext("TstzRangeArray", Box::new(Value::Binary({
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
            PgType::DateRangeArray => {
                Value::Ext("DateRangeArray", Box::new(Value::Binary({
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
            PgType::Int8RangeArray => {
                Value::Ext("Int8RangeArray", Box::new(Value::Binary({
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
            PgType::JsonpathArray => {
                Value::Ext("JsonpathArray", Box::new(Value::Binary({
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
            PgType::MoneyArray => {
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
        }
    }
}

impl Decode for bool {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => value.as_bytes()?[0] != 0,

            PgValueFormat::Text => match value.as_str()? {
                "t" => true,
                "f" => false,

                s => {
                    return Err(format!("unexpected value {:?} for boolean", s).into());
                }
            },
        })
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
