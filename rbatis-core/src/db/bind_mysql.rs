use rbson::Bson;
use rbson::spec::BinarySubtype;
use sqlx_core::mysql::{MySql, MySqlArguments};
use sqlx_core::query::Query;
use crate::error::Error;
use crate::types::{DateNative, DateTimeNative, DateTimeUtc, DateUtc, Decimal, TimeNative, TimeUtc};
use crate::Uuid;

#[inline]
pub fn bind(t: Bson, mut q: Query<MySql, MySqlArguments>) -> crate::Result<Query<MySql, MySqlArguments>> {
    match t {
        Bson::String(s) => {
            if s.starts_with("DateTimeUtc(")  {
                let data: DateTimeUtc = rbson::from_bson(Bson::String(s))?;
                q = q.bind(data.inner);
                return Ok(q);
            }
            if s.starts_with("DateTimeNative(")  {
                let data: DateTimeNative = rbson::from_bson(Bson::String(s))?;
                q = q.bind(data.inner);
                return Ok(q);
            }
            if s.starts_with("DateNative(") {
                let data: DateNative = rbson::from_bson(Bson::String(s))?;
                q = q.bind(data.inner);
                return Ok(q);
            }
            if s.starts_with("DateUtc(") {
                let data: DateUtc = rbson::from_bson(Bson::String(s))?;
                q = q.bind(data.inner);
                return Ok(q);
            }
            if s.starts_with("TimeUtc(") {
                let data: TimeUtc = rbson::from_bson(Bson::String(s))?;
                q = q.bind(data.inner);
                return Ok(q);
            }
            if s.starts_with("TimeNative(") {
                let data: TimeNative = rbson::from_bson(Bson::String(s))?;
                q = q.bind(data.inner);
                return Ok(q);
            }
            if s.starts_with("Decimal(") {
                let data: Decimal = rbson::from_bson(Bson::String(s))?;
                q = q.bind(data.inner.to_string());
                return Ok(q);
            }
            if s.starts_with("Uuid(") {
                let data: Uuid = rbson::from_bson(Bson::String(s))?;
                q = q.bind(data.inner.to_string());
                return Ok(q);
            }
            q = q.bind(Some(s));
        }
        Bson::Null => {
            q = q.bind(Option::<String>::None);
        }
        Bson::Int32(n) => {
            q = q.bind(n);
        }
        Bson::Int64(n) => {
            q = q.bind(n);
        }
        Bson::UInt32(n) => {
            q = q.bind(n);
        }
        Bson::UInt64(n) => {
            q = q.bind(n);
        }
        Bson::Double(n) => {
            q = q.bind(n);
        }
        Bson::Boolean(b) => {
            q = q.bind(b);
        }
        Bson::Decimal128(d) => {
            q = q.bind(d.to_string());
        }
        Bson::Binary(d) => {
            match d.subtype {
                BinarySubtype::Generic => {
                    q = q.bind(d.bytes);
                }
                BinarySubtype::Uuid => {
                    q = q.bind(crate::types::Uuid::from(d).inner);
                }
                BinarySubtype::UserDefined(type_id) => {
                    match type_id {
                        crate::types::BINARY_SUBTYPE_JSON => {
                            q = q.bind(serde_json::from_slice::<serde_json::Value>(&d.bytes).unwrap_or_default());
                        }
                        _ => {
                            return Err(Error::from("un supported bind type!"));
                        }
                    }
                }
                _ => {
                    return Err(Error::from("un supported bind type!"));
                }
            }
        }
        Bson::DateTime(d) => {
            q = q.bind(DateTimeNative::from(d).inner);
        }
        Bson::Timestamp(d) => {
            q = q.bind(crate::types::Timestamp::from(d).inner);
        }
        _ => {
            return crate::Result::Err(crate::Error::from("unsupported type!"));
        }
    }
    return Ok(q);
}