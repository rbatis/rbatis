pub const BINARY_SUBTYPE_JSON: u8 = 0x90;





pub mod json;

use std::fmt::Formatter;
use rbson::Bson;
use rbson::spec::BinarySubtype;
pub use json::*;

pub mod uuids;

pub use uuids::*;

pub mod bytes;

pub use bytes::*;

pub mod datetime_native;

pub use datetime_native::*;

pub mod datetime_utc;

pub use datetime_utc::*;

pub mod date_native;

pub use date_native::*;

pub mod date_utc;

pub use date_utc::*;

pub mod time_native;

pub use time_native::*;

pub mod time_utc;

pub use time_utc::*;

pub mod decimal;

pub use decimal::*;

pub mod timestamp;

pub use timestamp::*;

pub mod timestamp_z;

pub use timestamp_z::*;

pub mod bools;

pub use bools::*;

pub trait Format {
    fn do_format(&self) -> String;
}


impl Format for Bson {
    fn do_format(&self) -> String {
        match self {
            Bson::Double(d) => { format!("{}", d) }
            Bson::String(s) => { format!("\"{}\"", s) }
            Bson::Array(arr) => {
                arr.do_format()
            }
            Bson::Document(d) => {
                let mut buf = String::new();
                buf.push_str("{");
                for (k, v) in d {
                    buf.push_str("\"");
                    buf.push_str(k);
                    buf.push_str("\"");
                    buf.push_str(":");
                    buf.push_str(&v.do_format());
                    buf.push_str(",");
                }
                buf.pop();
                buf.push_str("}");
                buf
            }
            Bson::Boolean(b) => { format!("{}", b) }
            Bson::Null => { format!("null") }
            Bson::RegularExpression(j) => { format!("{:?}", j) }
            Bson::JavaScriptCode(j) => { format!("{:?}", j) }
            Bson::JavaScriptCodeWithScope(j) => { format!("{:?}", j) }
            Bson::Int32(i) => { format!("{}", i) }
            Bson::Int64(i) => { format!("{}", i) }
            Bson::UInt32(i) => { format!("{}", i) }
            Bson::UInt64(i) => { format!("{}", i) }
            Bson::Timestamp(s) => {
                format!("\"{}\"", Timestamp::from(s.clone()))
            }
            Bson::Binary(d) => {
                match d.subtype {
                    BinarySubtype::Generic => {
                        let bytes_len = d.bytes.len();
                        if bytes_len > 8192 {
                            //> 1kb
                            format!("bytes({})", d.bytes.len())
                        } else {
                            self.to_string()
                        }
                    }
                    BinarySubtype::Uuid => {
                        format!("\"{}\"", crate::types::Uuid::from(d))
                    }
                    BinarySubtype::UserDefined(type_id) => {
                        match type_id {
                            crate::types::BINARY_SUBTYPE_JSON => {
                                format!("{}", String::from_utf8(d.bytes.to_owned()).unwrap_or_default())
                            }
                            _ => {
                                format!("un supported!")
                            }
                        }
                    }
                    _ => {
                        format!("un supported!")
                    }
                }
            }
            Bson::ObjectId(id) => {
                format!("\"{}\"", id)
            }
            Bson::DateTime(dt) => {
                format!("\"{}\"", DateTimeNative::from(dt.clone()))
            }
            Bson::Symbol(s) => { format!("{}", s) }
            Bson::Decimal128(d) => {
                format!("{}", d)
            }
            Bson::Undefined => {
                format!("{}", "Undefined")
            }
            Bson::MaxKey => { format!("{}", "MaxKey") }
            Bson::MinKey => { format!("{}", "MinKey") }
            Bson::DbPointer(p) => { format!("{:?}", p) }
        }
    }
}

impl Format for Vec<Bson> {
    fn do_format(&self) -> String {
        let mut buf = String::new();
        buf.push_str("[");
        for item in self {
            buf.push_str(&item.do_format());
            buf.push_str(",");
        }
        buf.pop();
        buf.push_str("]");
        buf
    }
}
