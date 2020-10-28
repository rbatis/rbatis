use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::postgres::{PgValue, PgValueRef, Postgres};
use sqlx_core::type_info::TypeInfo;
use sqlx_core::types::BigDecimal;
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec};
use sqlx_core::postgres::PgRow;
use sqlx_core::row::Row;

impl<'c> JsonCodec for PgValueRef<'c> {
    fn try_to_json(self) -> crate::Result<serde_json::Value> {
        let type_string = self.type_info().name().to_owned();
        match type_string.as_str() {
            "NUMERIC" => {
                //decimal
                let r: Result<BigDecimal, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(serde_json::Value::from(r.unwrap().to_string()));
            }
            "BOOL" => {
                let r: Result<bool, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "BYTEA" => {
                unimplemented!();
            }
            "FLOAT4" => {
                let r: Result<f32, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "FLOAT8" => {
                let r: Result<f64, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "INT2" => {
                let r: Result<i16, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "INT4" => {
                let r: Result<i32, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "INT8" => {
                let r: Result<i64, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "TEXT" | "VARCHAR" | "BPCHAR" | "CHAR" => {
                let r: Result<String, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "UUID" => {
                let r: Result<String, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }

            "TIME" => {
                let r: Result<chrono::NaiveTime, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "DATE" => {
                let r: Result<chrono::NaiveDate, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "TIMESTAMP" => {
                let r: Result<chrono::NaiveDateTime, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "TIMESTAMPTZ" => {
                let r: Result<chrono::NaiveDateTime, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            _ => return Err(crate::Error::from(format!("un support database type for:{:?}!", type_string))),
        }
    }
}


impl RefJsonCodec for Vec<PgRow>{
    fn try_to_json(&self) -> crate::Result<serde_json::Value> {
        let mut arr = vec![];
        for row in &self {
            let mut m = serde_json::Map::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v:PgValueRef = row.get(key);
                m.insert(key.to_owned(), v.try_to_json()?);
            }
            arr.push(serde_json::Value::Object(m));
        }
        Ok(Value::from(arr))
    }
}