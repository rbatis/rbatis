use crate::error::Error;
use crate::types::DateTimeNative;
use crate::types::{DateNative, DateTimeUtc, DateUtc, Decimal, TimeNative, TimeUtc, TimestampZ};
use crate::Uuid;
use bigdecimal_::BigDecimal;
use rbson::spec::BinarySubtype;
use rbson::Bson;
use sqlx_core::encode::{Encode, IsNull};
use sqlx_core::postgres::{PgArgumentBuffer, PgArguments, PgHasArrayType, PgTypeInfo, Postgres};
use sqlx_core::query::Query;
use sqlx_core::types::Type;
use std::str::FromStr;
use std::time::SystemTime;

pub struct PgNull {}

impl Type<Postgres> for PgNull {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("UNKNOWN")
    }
}

impl PgHasArrayType for PgNull {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("UNKNOWN")
    }
}

impl Encode<'_, Postgres> for PgNull {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        IsNull::Yes
    }
}

#[inline]
pub fn bind(
    t: Bson,
    mut q: Query<Postgres, PgArguments>,
) -> crate::Result<Query<Postgres, PgArguments>> {
    match t {
        Bson::String(s) => {
            if s.starts_with("TimestampZ(") {
                let data: TimestampZ = rbson::from_bson(Bson::String(s))?;
                q = q.bind(data.inner);
                return Ok(q);
            }
            if s.starts_with("DateTimeUtc(") {
                let data: DateTimeUtc = rbson::from_bson(Bson::String(s))?;
                q = q.bind(data.inner);
                return Ok(q);
            }
            if s.starts_with("DateTimeNative(") {
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
                q = q.bind(data.inner);
                return Ok(q);
            }
            q = q.bind(Some(s));
        }
        Bson::Null => {
            q = q.bind(PgNull {});
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
            q = q.bind(BigDecimal::from_str(&d.to_string()).unwrap_or_default());
        }
        Bson::Binary(d) => match d.subtype {
            BinarySubtype::Generic => {
                q = q.bind(d.bytes);
            }
            BinarySubtype::Uuid => {
                q = q.bind(crate::types::Uuid::from(d).inner);
            }
            BinarySubtype::UserDefined(type_id) => match type_id {
                crate::types::BINARY_SUBTYPE_JSON => {
                    q = q.bind(
                        serde_json::from_slice::<serde_json::Value>(&d.bytes).unwrap_or_default(),
                    );
                }
                _ => {
                    return Err(Error::from("un supported bind type!"));
                }
            },
            _ => {
                return Err(Error::from("un supported bind type!"));
            }
        },
        Bson::DateTime(d) => {
            q = q.bind(DateTimeNative::from(d).inner);
        }
        Bson::Timestamp(d) => {
            let timestamp = {
                let upper = (d.time.to_le() as u64) << 32;
                let lower = d.increment.to_le() as u64;
                (upper | lower) as i64
            };
            let sec = timestamp / 1000;
            let ns = (timestamp % 1000 * 1000000) as u32;
            let primitive_date_time = chrono::NaiveDateTime::from_timestamp(sec, ns);
            q = q.bind(primitive_date_time);
        }
        Bson::Array(arr) => {
            let mut arr_null = vec![];
            let mut arr_str = vec![];
            let mut arr_i32 = vec![];
            let mut arr_i64 = vec![];
            let mut arr_double = vec![];
            let mut arr_bool = vec![];
            let mut arr_decimal = vec![];
            let mut arr_bytes = vec![];
            let mut arr_uuid = vec![];
            let mut arr_json = vec![];
            let mut arr_timestampz = vec![];
            let mut arr_datetime_utc = vec![];
            let mut arr_datetime_native = vec![];
            let mut arr_datenative = vec![];
            let mut arr_dateutc = vec![];
            let mut arr_timeutc = vec![];
            let mut arr_timenative = vec![];
            for x in arr {
                match x {
                    Bson::String(s) => {
                        if s.starts_with("TimestampZ(") {
                            let data: TimestampZ = rbson::from_bson(Bson::String(s))?;
                            arr_timestampz.push(data.inner);
                            continue;
                        }
                        if s.starts_with("DateTimeUtc(") {
                            let data: DateTimeUtc = rbson::from_bson(Bson::String(s))?;
                            arr_datetime_utc.push(data.inner);
                            continue;
                        }
                        if s.starts_with("DateTimeNative(") {
                            let data: DateTimeNative = rbson::from_bson(Bson::String(s))?;
                            arr_datetime_native.push(data.inner);
                            continue;
                        }
                        if s.starts_with("DateNative(") {
                            let data: DateNative = rbson::from_bson(Bson::String(s))?;
                            arr_datenative.push(data.inner);
                            continue;
                        }
                        if s.starts_with("DateUtc(") {
                            let data: DateUtc = rbson::from_bson(Bson::String(s))?;
                            arr_dateutc.push(data.inner);
                            continue;
                        }
                        if s.starts_with("TimeUtc(") {
                            let data: TimeUtc = rbson::from_bson(Bson::String(s))?;
                            arr_timeutc.push(data.inner);
                            continue;
                        }
                        if s.starts_with("TimeNative(") {
                            let data: TimeNative = rbson::from_bson(Bson::String(s))?;
                            arr_timenative.push(data.inner);
                            continue;
                        }
                        if s.starts_with("Decimal(") {
                            let data: Decimal = rbson::from_bson(Bson::String(s))?;
                            arr_decimal.push(data.inner);
                            continue;
                        }
                        if s.starts_with("Uuid(") {
                            let data: Uuid = rbson::from_bson(Bson::String(s))?;
                            arr_uuid.push(data.inner);
                            continue;
                        }
                        arr_str.push(s);
                    }
                    Bson::Null => {
                        arr_null.push(PgNull {});
                    }
                    Bson::Int32(n) => {
                        arr_i32.push(n);
                    }
                    Bson::Int64(n) => {
                        arr_i64.push(n);
                    }
                    Bson::Double(n) => {
                        arr_double.push(n);
                    }
                    Bson::Boolean(b) => {
                        arr_bool.push(b);
                    }
                    Bson::Decimal128(d) => {
                        arr_decimal.push(BigDecimal::from_str(&d.to_string()).unwrap_or_default());
                    }
                    Bson::Binary(d) => match d.subtype {
                        BinarySubtype::Generic => {
                            arr_bytes.push(d.bytes);
                        }
                        BinarySubtype::Uuid => {
                            arr_uuid.push(crate::types::Uuid::from(d).inner);
                        }
                        BinarySubtype::UserDefined(type_id) => match type_id {
                            crate::types::BINARY_SUBTYPE_JSON => {
                                arr_json.push(
                                    serde_json::from_slice::<serde_json::Value>(&d.bytes)
                                        .unwrap_or_default(),
                                );
                            }
                            _ => {
                                return Err(Error::from("un supported bind type!"));
                            }
                        },
                        _ => {
                            return Err(Error::from("un supported bind type!"));
                        }
                    },
                    Bson::DateTime(d) => {
                        q = q.bind(DateTimeNative::from(d).inner);
                    }
                    Bson::Timestamp(d) => {
                        let timestamp = {
                            let upper = (d.time.to_le() as u64) << 32;
                            let lower = d.increment.to_le() as u64;
                            (upper | lower) as i64
                        };
                        let sec = timestamp / 1000;
                        let ns = (timestamp % 1000 * 1000000) as u32;
                        let primitive_date_time = chrono::NaiveDateTime::from_timestamp(sec, ns);
                        q = q.bind(primitive_date_time);
                    }
                    _ => {
                        return crate::Result::Err(crate::Error::from("unsupported type!"));
                    }
                }
            }
            if !arr_null.is_empty() {
                q = q.bind(arr_null);
            }
            if !arr_str.is_empty() {
                q = q.bind(arr_str);
            }
            if !arr_i32.is_empty() {
                q = q.bind(arr_i32);
            }
            if !arr_i64.is_empty() {
                q = q.bind(arr_i64);
            }
            if !arr_double.is_empty() {
                q = q.bind(arr_double);
            }
            if !arr_decimal.is_empty() {
                q = q.bind(arr_decimal);
            }
            if !arr_bytes.is_empty() {
                q = q.bind(arr_bytes);
            }
            if !arr_uuid.is_empty() {
                q = q.bind(arr_uuid);
            }
            if !arr_json.is_empty() {
                q = q.bind(arr_json);
            }
            if !arr_timestampz.is_empty() {
                q = q.bind(arr_timestampz);
            }
            if !arr_datetime_utc.is_empty() {
                q = q.bind(arr_datetime_utc);
            }
            if !arr_datetime_native.is_empty() {
                q = q.bind(arr_datetime_native);
            }
            if !arr_datenative.is_empty() {
                q = q.bind(arr_datenative);
            }
            if !arr_dateutc.is_empty() {
                q = q.bind(arr_dateutc);
            }
            if !arr_timeutc.is_empty() {
                q = q.bind(arr_timeutc);
            }
            if !arr_timenative.is_empty() {
                q = q.bind(arr_timenative);
            }
        }
        _ => {
            return crate::Result::Err(crate::Error::from("unsupported type!"));
        }
    }
    return Ok(q);
}
