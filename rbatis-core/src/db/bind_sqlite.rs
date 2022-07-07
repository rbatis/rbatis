use rbson::Bson;
use rbson::spec::BinarySubtype;
use sqlx_core::encode::{Encode, IsNull};
use sqlx_core::sqlite::{Sqlite, SqliteArguments, SqliteArgumentValue, SqliteTypeInfo};
use sqlx_core::query::Query;
use sqlx_core::types::Type;
use crate::error::Error;
use crate::types::{DateNative, DateTimeNative, DateTimeUtc, DateUtc, Decimal, TimeNative, TimeUtc};
use crate::Uuid;

pub struct SqliteNull {}

impl Type<Sqlite> for SqliteNull {
    fn type_info() -> SqliteTypeInfo {
        serde_json::from_str("NULL").unwrap()
    }
}

impl Encode<'_, Sqlite> for SqliteNull {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue>) -> IsNull {
        IsNull::Yes
    }
}

#[inline]
pub fn bind<'a>(t: Bson, mut q: Query<'a, Sqlite, SqliteArguments<'a>>) -> crate::Result<Query<'a, Sqlite, SqliteArguments<'a>>> {
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
            q = q.bind(SqliteNull{});
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
            q = q.bind(n as i64);
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
                    q = q.bind(crate::types::Uuid::from(d).to_string());
                }
                BinarySubtype::UserDefined(type_id) => {
                    match type_id {
                        crate::types::BINARY_SUBTYPE_JSON => {
                            q = q.bind(d.bytes);
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
            q = q.bind(crate::types::Timestamp::from(d).inner.to_string());
        }
        _ => {
            return crate::Result::Err(crate::Error::from("unsupported type!"));
        }
    }
    return Ok(q);
}