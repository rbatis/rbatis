use crate::arguments::PgArgumentBuffer;
use crate::type_info::PgType;
use crate::type_info::PgTypeInfo;
use crate::types::byte::Bytea;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::types::money::Money;
use crate::types::timestamptz::Timestamptz;
use crate::types::timetz::Timetz;
use crate::types::Oid;
use crate::types::TypeInfo;
use crate::value::{PgValue, PgValueFormat};
use rbdc::date::Date;
use rbdc::datetime::DateTime;
use rbdc::decimal::Decimal;
use rbdc::json::Json;
use rbdc::timestamp::Timestamp;
use rbdc::types::time::Time;
use rbdc::uuid::Uuid;
use rbdc::{Error, RBDCString};
use rbs::Value;
use std::str::FromStr;

impl TypeInfo for Value {
    fn type_info(&self) -> PgTypeInfo {
        match self {
            Value::Null => PgTypeInfo::UNKNOWN,
            Value::Bool(_) => PgTypeInfo::BOOL,
            Value::I32(_) => PgTypeInfo::INT4,
            Value::I64(_) => PgTypeInfo::INT8,
            Value::U32(_) => PgTypeInfo::INT4,
            Value::U64(_) => PgTypeInfo::INT8,
            Value::F32(_) => PgTypeInfo::FLOAT4,
            Value::F64(_) => PgTypeInfo::FLOAT8,
            Value::String(v) => {
                let t = {
                    if Date::is(&v) != "" {
                        "Date"
                    } else if DateTime::is(&v) != "" {
                        "DateTime"
                    } else if Time::is(&v) != "" {
                        "Time"
                    } else if Timestamp::is(&v) != "" {
                        "Timestamp"
                    } else if Decimal::is(&v) != "" {
                        "Decimal"
                    } else if Uuid::is(&v) != "" {
                        "Uuid"
                    } else if v.ends_with("Bytea") {
                        "Bytea"
                    } else if v.ends_with("Char") {
                        "Char"
                    } else if v.ends_with("Name") {
                        "Name"
                    } else if v.ends_with("Int2") {
                        "Int2"
                    } else if v.ends_with("Text") {
                        "Text"
                    } else if v.ends_with("Oid") {
                        "Oid"
                    } else if v.ends_with("Json") {
                        "Json"
                    } else if v.ends_with("Point") {
                        "Point"
                    } else if v.ends_with("Lseg") {
                        "Lseg"
                    } else if v.ends_with("Path") {
                        "Path"
                    } else if v.ends_with("Box") {
                        "Box"
                    } else if v.ends_with("Polygon") {
                        "Polygon"
                    } else if v.ends_with("Line") {
                        "Line"
                    } else if v.ends_with("Cidr") {
                        "Cidr"
                    } else if v.ends_with("Unknown") {
                        "Unknown"
                    } else if v.ends_with("Circle") {
                        "Circle"
                    } else if v.ends_with("Macaddr8") {
                        "Macaddr8"
                    } else if v.ends_with("Macaddr") {
                        "Macaddr"
                    } else if v.ends_with("Inet") {
                        "Inet"
                    } else if v.ends_with("Bpchar") {
                        "Bpchar"
                    } else if v.ends_with("Varchar") {
                        "Varchar"
                    } else if v.ends_with("Timestamptz") {
                        "Timestamptz"
                    } else if v.ends_with("Interval") {
                        "Interval"
                    } else if v.ends_with("Timetz") {
                        "Timetz"
                    } else if v.ends_with("Bit") {
                        "Bit"
                    } else if v.ends_with("Varbit") {
                        "Varbit"
                    } else if v.ends_with("Numeric") {
                        "Numeric"
                    } else if v.ends_with("Record") {
                        "Record"
                    } else if v.ends_with("Jsonb") {
                        "Jsonb"
                    } else if v.ends_with("Int4Range") {
                        "Int4Range"
                    } else if v.ends_with("NumRange") {
                        "NumRange"
                    } else if v.ends_with("TsRange") {
                        "TsRange"
                    } else if v.ends_with("TstzRange") {
                        "TstzRange"
                    } else if v.ends_with("Record") {
                        "Record"
                    } else if v.ends_with("Jsonb") {
                        "Jsonb"
                    } else if v.ends_with("Int4Range") {
                        "Int4Range"
                    } else if v.ends_with("NumRange") {
                        "NumRange"
                    } else if v.ends_with("TsRange") {
                        "TsRange"
                    } else if v.ends_with("TstzRange") {
                        "TstzRange"
                    } else if v.ends_with("DateRange") {
                        "DateRange"
                    } else if v.ends_with("Int8Range") {
                        "Int8Range"
                    } else if v.ends_with("Jsonpath") {
                        "Jsonpath"
                    } else if v.ends_with("Money") {
                        "Money"
                    } else if v.ends_with("Void") {
                        "Void"
                    } else if v.ends_with("Custom") {
                        "Custom"
                    } else if v.ends_with("DeclareWithName") {
                        "DeclareWithName"
                    } else if v.ends_with("DeclareWithOid") {
                        "DeclareWithOid"
                    } else {
                        ""
                    }
                };
                match t {
                    "Uuid" => PgTypeInfo::UUID,
                    //decimal = 12345678
                    "Decimal" => PgTypeInfo::NUMERIC,
                    //Date = "1993-02-06"
                    "Date" => PgTypeInfo::DATE,
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => PgTypeInfo::TIME,
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => PgTypeInfo::TIMESTAMP,
                    "DateTime" => PgTypeInfo::TIMESTAMP,
                    "Bool" => PgTypeInfo::BOOL,
                    "Bytea" => PgTypeInfo::BYTEA,
                    "Char" => PgTypeInfo::CHAR,
                    "Name" => PgTypeInfo::NAME,
                    "Int8" => PgTypeInfo::INT8,
                    "Int2" => PgTypeInfo::INT2,
                    "Int4" => PgTypeInfo::INT4,
                    "Text" => PgTypeInfo::TEXT,
                    "Oid" => PgTypeInfo::OID,
                    "Json" => PgTypeInfo::JSON,
                    "Point" => PgTypeInfo::POINT,
                    "Lseg" => PgTypeInfo::LSEG,
                    "Path" => PgTypeInfo::PATH,
                    "Box" => PgTypeInfo::BOX,
                    "Polygon" => PgTypeInfo::POLYGON,
                    "Line" => PgTypeInfo::LINE,
                    "Cidr" => PgTypeInfo::CIDR,
                    "Float4" => PgTypeInfo::FLOAT4,
                    "Float8" => PgTypeInfo::FLOAT8,
                    "Unknown" => PgTypeInfo::UNKNOWN,
                    "Circle" => PgTypeInfo::CIRCLE,
                    "Macaddr8" => PgTypeInfo::MACADDR8,
                    "Macaddr" => PgTypeInfo::MACADDR,
                    "Inet" => PgTypeInfo::INET,
                    "Bpchar" => PgTypeInfo::BPCHAR,
                    "Varchar" => PgTypeInfo::VARCHAR,
                    "Timestamptz" => PgTypeInfo::TIMESTAMPTZ,
                    "Interval" => PgTypeInfo::INTERVAL,
                    "Timetz" => PgTypeInfo::TIMETZ,
                    "Bit" => PgTypeInfo::BIT,
                    "Varbit" => PgTypeInfo::VARBIT,
                    "Numeric" => PgTypeInfo::NUMERIC,
                    "Record" => PgTypeInfo::RECORD,
                    "Jsonb" => PgTypeInfo::JSONB,
                    "Int4Range" => PgTypeInfo::INT4_RANGE,
                    "NumRange" => PgTypeInfo::NUM_RANGE,
                    "TsRange" => PgTypeInfo::TS_RANGE,
                    "TstzRange" => PgTypeInfo::TSTZ_RANGE,
                    "DateRange" => PgTypeInfo::DATE_RANGE,
                    "Int8Range" => PgTypeInfo::INT8_RANGE,
                    "Jsonpath" => PgTypeInfo::JSONPATH,
                    "Money" => PgTypeInfo::MONEY,
                    "Void" => PgTypeInfo::VOID,
                    "Custom" => PgTypeInfo::UNKNOWN,
                    "DeclareWithName" => PgTypeInfo::UNKNOWN,
                    "DeclareWithOid" => PgTypeInfo::UNKNOWN,
                    _ => PgTypeInfo::UNKNOWN,
                }
            }
            Value::Binary(_) => PgTypeInfo::BYTEA_ARRAY,
            Value::Array(_) => {
                // if arr.len() == 0 {
                //     return PgTypeInfo::UNKNOWN;
                // }
                // arr[0]
                //     .type_info()
                //     .clone()
                //     .to_array_type()
                //     .unwrap_or(PgTypeInfo::UNKNOWN)
                PgTypeInfo::JSON
            }
            Value::Map(_) => PgTypeInfo::JSON,
        }
    }
}

impl Decode for Value {
    fn decode(arg: PgValue) -> Result<Self, Error> {
        if arg.value.is_none() {
            return Ok(Value::Null);
        }
        Ok(match arg.type_info().0 {
            PgType::Bool => Value::Bool(Decode::decode(arg)?),
            PgType::Bytea => Bytea::decode(arg)?.into(),
            PgType::Char => Value::String(Decode::decode(arg)?),
            PgType::Name => Value::String(Decode::decode(arg)?),
            PgType::Int8 => Value::I64(Decode::decode(arg)?),
            PgType::Int2 => Value::I32({
                let i16: i16 = Decode::decode(arg)?;
                i16 as i32
            }),
            PgType::Int4 => Value::I32(Decode::decode(arg)?),
            PgType::Text => Value::String(Decode::decode(arg)?),
            PgType::Oid => Value::from(Value::U32(Decode::decode(arg)?)),
            PgType::Json => Json::decode(arg)?.into(),
            PgType::Point => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Lseg => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Path => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Box => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Polygon => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Line => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Cidr => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),

            PgType::Float4 => Value::F32(Decode::decode(arg)?),
            PgType::Float8 => Value::F32(Decode::decode(arg)?),
            PgType::Unknown => Value::Null,
            PgType::Circle => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Macaddr8 => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Macaddr => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Inet => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Bpchar => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Varchar => Value::String(Decode::decode(arg)?),
            PgType::Date => {
                let v: Date = Decode::decode(arg)?;
                v
            }
            .into(),
            PgType::Time => {
                let v: Time = Decode::decode(arg)?;
                v
            }
            .into(),
            PgType::Timestamp => {
                let v: Timestamp = Decode::decode(arg)?;
                v
            }
            .into(),
            PgType::Timestamptz => Timestamptz::decode(arg)?.into(),
            PgType::Interval => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Timetz => Timetz::decode(arg)?.into(),
            PgType::Bit => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Varbit => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Numeric => Decimal::decode(arg)?.into(),
            PgType::Record => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Uuid => Uuid::decode(arg)?.into(),
            PgType::Jsonb => Json::decode(arg)?.into(),
            PgType::Int4Range => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::NumRange => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::TsRange => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::TstzRange => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::DateRange => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Int8Range => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Jsonpath => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Money => Money::decode(arg)?.into(),
            PgType::Void => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::Custom(_) => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::DeclareWithName(_) => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::DeclareWithOid(_) => Value::from(Value::Binary({
                match arg.format() {
                    PgValueFormat::Binary => arg.as_bytes()?.to_owned(),
                    PgValueFormat::Text => arg.as_str()?.as_bytes().to_vec(),
                }
            })),
            PgType::JsonArray => Value::Array(Decode::decode(arg)?),
            PgType::LineArray => Value::Array(Decode::decode(arg)?),
            PgType::CidrArray => Value::Array(Decode::decode(arg)?),
            PgType::CircleArray => Value::Array(Decode::decode(arg)?),
            PgType::Macaddr8Array => Value::Array(Decode::decode(arg)?),
            PgType::BoolArray => Value::Array(Decode::decode(arg)?),
            PgType::ByteaArray => Value::Array(Decode::decode(arg)?),
            PgType::CharArray => Value::Array(Decode::decode(arg)?),
            PgType::NameArray => Value::Array(Decode::decode(arg)?),
            PgType::Int2Array => Value::Array(Decode::decode(arg)?),
            PgType::Int4Array => Value::Array(Decode::decode(arg)?),
            PgType::TextArray => Value::Array(Decode::decode(arg)?),
            PgType::BpcharArray => Value::Array(Decode::decode(arg)?),
            PgType::VarcharArray => Value::Array(Decode::decode(arg)?),
            PgType::Int8Array => Value::Array(Decode::decode(arg)?),
            PgType::PointArray => Value::Array(Decode::decode(arg)?),
            PgType::LsegArray => Value::Array(Decode::decode(arg)?),
            PgType::PathArray => Value::Array(Decode::decode(arg)?),
            PgType::BoxArray => Value::Array(Decode::decode(arg)?),
            PgType::Float4Array => Value::Array(Decode::decode(arg)?),
            PgType::Float8Array => Value::Array(Decode::decode(arg)?),
            PgType::PolygonArray => Value::Array(Decode::decode(arg)?),
            PgType::OidArray => Value::Array(Decode::decode(arg)?),
            PgType::MacaddrArray => Value::Array(Decode::decode(arg)?),
            PgType::InetArray => Value::Array(Decode::decode(arg)?),
            PgType::TimestampArray => Value::Array(Decode::decode(arg)?),
            PgType::DateArray => Value::Array(Decode::decode(arg)?),
            PgType::TimeArray => Value::Array(Decode::decode(arg)?),
            PgType::TimestamptzArray => Value::Array(Decode::decode(arg)?),
            PgType::IntervalArray => Value::Array(Decode::decode(arg)?),
            PgType::NumericArray => Value::Array(Decode::decode(arg)?),
            PgType::TimetzArray => Value::Array(Decode::decode(arg)?),
            PgType::BitArray => Value::Array(Decode::decode(arg)?),
            PgType::VarbitArray => Value::Array(Decode::decode(arg)?),
            PgType::RecordArray => Value::Array(Decode::decode(arg)?),
            PgType::UuidArray => Value::Array(Decode::decode(arg)?),
            PgType::JsonbArray => Value::Array(Decode::decode(arg)?),
            PgType::Int4RangeArray => Value::Array(Decode::decode(arg)?),
            PgType::NumRangeArray => Value::Array(Decode::decode(arg)?),
            PgType::TsRangeArray => Value::Array(Decode::decode(arg)?),
            PgType::TstzRangeArray => Value::Array(Decode::decode(arg)?),
            PgType::DateRangeArray => Value::Array(Decode::decode(arg)?),
            PgType::Int8RangeArray => Value::Array(Decode::decode(arg)?),
            PgType::JsonpathArray => Value::Array(Decode::decode(arg)?),
            PgType::MoneyArray => Value::Array(Decode::decode(arg)?),
        })
    }
}

impl Encode for Value {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        Ok(match self {
            Value::Null => IsNull::Yes,
            Value::Bool(v) => v.encode(buf)?,
            Value::I32(v) => v.encode(buf)?,
            Value::I64(v) => v.encode(buf)?,
            Value::U32(v) => v.encode(buf)?,
            Value::U64(v) => v.encode(buf)?,
            Value::F32(v) => v.encode(buf)?,
            Value::F64(v) => v.encode(buf)?,
            Value::String(v) => {
                let r;
                if v.ends_with(Uuid::ends_name()) {
                    r = "Uuid";
                } else if v.ends_with(Decimal::ends_name()) {
                    r = "Decimal";
                } else if v.ends_with(Date::ends_name()) {
                    r = "Date";
                } else if v.ends_with(Time::ends_name()) {
                    r = "Time";
                } else if v.ends_with(Timestamp::ends_name()) {
                    r = "Timestamp";
                } else if v.ends_with(DateTime::ends_name()) {
                    r = "DateTime";
                } else if v.ends_with("Bytea") {
                    r = "Bytea";
                } else if v.ends_with("Char") {
                    r = "Char";
                } else if v.ends_with("Name") {
                    r = "Name";
                } else if v.ends_with("Int8") {
                    r = "Int8";
                } else if v.ends_with("Int2") {
                    r = "Int2";
                } else if v.ends_with("Int4") {
                    r = "Int4";
                } else if v.ends_with("Text") {
                    r = "Text";
                } else if v.ends_with("Oid") {
                    r = "Oid";
                } else if v.ends_with("Json") {
                    r = "Json";
                } else if v.ends_with("Point") {
                    r = "Point";
                } else if v.ends_with("Lseg") {
                    r = "Lseg";
                } else if v.ends_with("Path") {
                    r = "Path";
                } else if v.ends_with("Box") {
                    r = "Box";
                } else if v.ends_with("Polygon") {
                    r = "Polygon";
                } else if v.ends_with("Line") {
                    r = "Line";
                } else if v.ends_with("Cidr") {
                    r = "Cidr";
                } else if v.ends_with("Float4") {
                    r = "Float4";
                } else if v.ends_with("Float8") {
                    r = "Float8";
                } else if v.ends_with("Unknown") {
                    r = "Unknown";
                } else if v.ends_with("Circle") {
                    r = "Circle";
                } else if v.ends_with("Macaddr8") {
                    r = "Macaddr8";
                } else if v.ends_with("Macaddr") {
                    r = "Macaddr";
                } else if v.ends_with("Inet") {
                    r = "Inet";
                } else if v.ends_with("Bpchar") {
                    r = "Bpchar";
                } else if v.ends_with("Varchar") {
                    r = "Varchar";
                } else if v.ends_with("Timestamptz") {
                    r = "Timestamptz";
                } else if v.ends_with("Interval") {
                    r = "Interval";
                } else if v.ends_with("Timetz") {
                    r = "Timetz";
                } else if v.ends_with("Bit") {
                    r = "Bit";
                } else if v.ends_with("Varbit") {
                    r = "Varbit";
                } else if v.ends_with("Numeric") {
                    r = "Numeric";
                } else if v.ends_with("Record") {
                    r = "Record";
                } else if v.ends_with("Jsonb") {
                    r = "Jsonb";
                } else if v.ends_with("Int4Range") {
                    r = "Int4Range";
                } else if v.ends_with("NumRange") {
                    r = "NumRange";
                } else if v.ends_with("TsRange") {
                    r = "TsRange";
                } else if v.ends_with("TstzRange") {
                    r = "TstzRange";
                } else if v.ends_with("DateRange") {
                    r = "DateRange";
                } else if v.ends_with("Int8Range") {
                    r = "Int8Range";
                } else if v.ends_with("Jsonpath") {
                    r = "Jsonpath";
                } else if v.ends_with("Money") {
                    r = "Money";
                } else {
                    r = ""
                }
                match r {
                    "Uuid" => Uuid::from(v).encode(buf)?,
                    //decimal = 12345678
                    "Decimal" => Decimal::from(v).encode(buf)?,
                    //Date = "1993-02-06"
                    "Date" => Date::from(fastdate::Date::from_str(&v).unwrap()).encode(buf)?,
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => Time::from(fastdate::Time::from_str(&v).unwrap()).encode(buf)?,
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => {
                        Timestamp::from(v.parse::<u64>().unwrap_or_default()).encode(buf)?
                    }
                    "DateTime" => {
                        DateTime::from(fastdate::DateTime::from_str(&v).unwrap()).encode(buf)?
                    }
                    "Bytea" => Bytea(v.parse::<u8>().unwrap_or_default()).encode(buf)?,
                    "Char" => v.encode(buf)?,
                    "Name" => v.encode(buf)?,
                    "Int8" => (v.parse::<i32>().unwrap_or_default()).encode(buf)?,
                    "Int2" => (v.parse::<i8>().unwrap_or_default()).encode(buf)?,
                    "Int4" => (v.parse::<i16>().unwrap_or_default()).encode(buf)?,
                    "Text" => v.encode(buf)?,
                    "Oid" => Oid::from(v.parse::<u32>().unwrap_or_default()).encode(buf)?,
                    "Point" => v.into_bytes().encode(buf)?,
                    "Lseg" => v.into_bytes().encode(buf)?,
                    "Path" => v.into_bytes().encode(buf)?,
                    "Box" => v.into_bytes().encode(buf)?,
                    "Polygon" => v.into_bytes().encode(buf)?,
                    "Line" => v.into_bytes().encode(buf)?,
                    "Cidr" => v.into_bytes().encode(buf)?,
                    "Float4" => (v.parse::<f32>().unwrap_or_default()).encode(buf)?,
                    "Float8" => v.parse::<f64>().unwrap_or_default().encode(buf)?,
                    "Unknown" => v.into_bytes().encode(buf)?,
                    "Circle" => v.into_bytes().encode(buf)?,
                    "Macaddr8" => v.into_bytes().encode(buf)?,
                    "Macaddr" => v.into_bytes().encode(buf)?,
                    "Inet" => v.into_bytes().encode(buf)?,
                    "Bpchar" => v.into_bytes().encode(buf)?,
                    "Varchar" => v.into_bytes().encode(buf)?,
                    "Timestamptz" => {
                        Timestamptz(v.parse::<u64>().unwrap_or_default()).encode(buf)?
                    }
                    "Interval" => v.into_bytes().encode(buf)?,
                    "Timetz" => v.into_bytes().encode(buf)?,
                    "Bit" => v.into_bytes().encode(buf)?,
                    "Varbit" => v.into_bytes().encode(buf)?,
                    "Numeric" => Decimal::from(v).encode(buf)?,
                    "Record" => v.into_bytes().encode(buf)?,
                    "Int4Range" => v.into_bytes().encode(buf)?,
                    "NumRange" => v.into_bytes().encode(buf)?,
                    "TsRange" => v.into_bytes().encode(buf)?,
                    "TstzRange" => v.into_bytes().encode(buf)?,
                    "DateRange" => v.into_bytes().encode(buf)?,
                    "Int8Range" => v.into_bytes().encode(buf)?,
                    "Jsonpath" => v.into_bytes().encode(buf)?,
                    "Money" => Money(v.parse::<i64>().unwrap_or_default()).encode(buf)?,
                    "Void" => v.into_bytes().encode(buf)?,
                    "Custom" => v.into_bytes().encode(buf)?,
                    "DeclareWithName" => v.into_bytes().encode(buf)?,
                    "DeclareWithOid" => v.into_bytes().encode(buf)?,
                    _ => v.encode(buf)?,
                }
            }
            Value::Binary(v) => v.encode(buf)?,
            Value::Array(v) => v.encode(buf)?,
            Value::Map(m) => rbdc::types::json::Json::from(Value::Map(m)).encode(buf)?,
        })
    }
}
