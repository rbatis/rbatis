use std::str::FromStr;
use rbson::Bson;
use rbson::spec::BinarySubtype;
use sqlx_core::query::Query;
use crate::error::Error;
use crate::types::{DateNative, DateTimeUtc, DateUtc, Decimal, TimeNative, TimestampZ, TimeUtc};
use std::time::SystemTime;
use bigdecimal_::BigDecimal;
use sqlx_core::postgres::{Postgres, PgArguments};
use crate::types::DateTimeNative;
use crate::Uuid;

#[inline]
pub fn bind(t: Bson, mut q: Query<Postgres, PgArguments>) -> crate::Result<Query<Postgres, PgArguments>> {
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
                q = q.bind(data.inner);
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
            let systime = SystemTime::from(crate::types::Timestamp::from(d).inner);
            let primitive_date_time = time::PrimitiveDateTime::from(systime);
            q = q.bind(primitive_date_time);
        }
        Bson::ObjectId(d) => {
            q = q.bind(d.to_string());
        }
        Bson::Array(arr) => {
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
                        if s.starts_with("DateTimeNative(")  {
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
                        return crate::Result::Err(crate::Error::from("unsupported type!"));
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
                    Bson::Binary(d) => {
                        match d.subtype {
                            BinarySubtype::Generic => {
                                arr_bytes.push(d.bytes);
                            }
                            BinarySubtype::Uuid => {
                                arr_uuid.push(crate::types::Uuid::from(d).inner);
                            }
                            BinarySubtype::UserDefined(type_id) => {
                                match type_id {
                                    crate::types::BINARY_SUBTYPE_JSON => {
                                        arr_json.push(serde_json::from_slice::<serde_json::Value>(&d.bytes).unwrap_or_default());
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
                        let systime = SystemTime::from(crate::types::Timestamp::from(d).inner);
                        let primitive_date_time = time::PrimitiveDateTime::from(systime);
                        q = q.bind(primitive_date_time);
                    }
                    Bson::ObjectId(d) => {
                        q = q.bind(d.to_string());
                    }
                    _ => {
                        return crate::Result::Err(crate::Error::from("unsupported type!"));
                    }
                }
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