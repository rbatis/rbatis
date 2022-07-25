use crate::type_info::PgType;
use crate::types::Oid;
use crate::value::{PgValue, PgValueFormat};
use byteorder::{BigEndian, ByteOrder};
use fastdate::{Date, DateTime};
use rbdc::Error;
use rbs::value::RBox;
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
            PgType::Bytea => {
                todo!()
            }
            PgType::Char => Value::String(Decode::decode(arg).unwrap()),
            PgType::Name => Value::String(Decode::decode(arg).unwrap()),
            PgType::Int8 => Value::I32(Decode::decode(arg).unwrap()),
            PgType::Int2 => Value::I32(Decode::decode(arg).unwrap()),
            PgType::Int4 => Value::I32(Decode::decode(arg).unwrap()),
            PgType::Text => Value::String(Decode::decode(arg).unwrap()),
            PgType::Oid => Value::Ext("oid", RBox::new(Value::U32(Decode::decode(arg).unwrap()))),
            PgType::Json => Value::String(
                crate::types::json::Json::decode(arg)
                    .unwrap_or_default()
                    .json,
            ),
            PgType::JsonArray => {
                todo!()
            }
            PgType::Point => {
                todo!()
            }
            PgType::Lseg => {
                todo!()
            }
            PgType::Path => {
                todo!()
            }
            PgType::Box => {
                todo!()
            }
            PgType::Polygon => {
                todo!()
            }
            PgType::Line => {
                todo!()
            }
            PgType::LineArray => {
                todo!()
            }
            PgType::Cidr => {
                todo!()
            }
            PgType::CidrArray => {
                todo!()
            }
            PgType::Float4 => Value::F32(Decode::decode(arg).unwrap()),
            PgType::Float8 => Value::F32(Decode::decode(arg).unwrap()),
            PgType::Unknown => Value::Null,
            PgType::Circle => {
                todo!()
            }
            PgType::CircleArray => {
                todo!()
            }
            PgType::Macaddr8 => {
                todo!()
            }
            PgType::Macaddr8Array => {
                todo!()
            }
            PgType::Macaddr => {
                todo!()
            }
            PgType::Inet => {
                todo!()
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
                todo!()
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
                todo!()
            }
            PgType::VarcharArray => {
                todo!()
            }
            PgType::Int8Array => {
                todo!()
            }
            PgType::PointArray => {
                todo!()
            }
            PgType::LsegArray => {
                todo!()
            }
            PgType::PathArray => {
                todo!()
            }
            PgType::BoxArray => {
                todo!()
            }
            PgType::Float4Array => {
                todo!()
            }
            PgType::Float8Array => {
                todo!()
            }
            PgType::PolygonArray => {
                todo!()
            }
            PgType::OidArray => {
                todo!()
            }
            PgType::MacaddrArray => {
                todo!()
            }
            PgType::InetArray => {
                todo!()
            }
            PgType::Bpchar => {
                todo!()
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
                todo!()
            }
            PgType::IntervalArray => {
                todo!()
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
                todo!()
            }
            PgType::RecordArray => {
                todo!()
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
                todo!()
            }
            PgType::Int4RangeArray => {
                todo!()
            }
            PgType::NumRange => {
                todo!()
            }
            PgType::NumRangeArray => {
                todo!()
            }
            PgType::TsRange => {
                todo!()
            }
            PgType::TsRangeArray => {
                todo!()
            }
            PgType::TstzRange => {
                todo!()
            }
            PgType::TstzRangeArray => {
                todo!()
            }
            PgType::DateRange => {
                todo!()
            }
            PgType::DateRangeArray => {
                todo!()
            }
            PgType::Int8Range => {
                todo!()
            }
            PgType::Int8RangeArray => {
                todo!()
            }
            PgType::Jsonpath => {
                todo!()
            }
            PgType::JsonpathArray => {
                todo!()
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
                todo!()
            }
            PgType::DeclareWithName(_) => {
                todo!()
            }
            PgType::DeclareWithOid(_) => {
                todo!()
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
