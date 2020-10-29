use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::postgres::{PgValue, PgValueRef, Postgres};
use sqlx_core::type_info::TypeInfo;
use sqlx_core::types::BigDecimal;
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec};
use sqlx_core::postgres::PgRow;
use sqlx_core::row::Row;
use sqlx_core::column::Column;
use crate::db_adapter::convert_result;
use serde_json::{json, Value};

impl<'c> JsonCodec for PgValueRef<'c> {
    fn try_to_json(self) -> crate::Result<serde_json::Value> {
        let type_string = self.type_info().name().to_owned();
        match type_string.as_str() {
            "NUMERIC" => {
                //decimal
                let r: Result<Option<BigDecimal>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                if r.as_ref().unwrap().is_none(){
                    return Ok(serde_json::Value::Null);
                }
                return Ok(json!(r.unwrap().unwrap().to_string()));
            }
            "BOOL" => {
                let r: Result<Option<bool>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "BYTEA" => {
                unimplemented!();
            }
            "FLOAT4" => {
                let r: Result<Option<f32>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "FLOAT8" => {
                let r: Result<Option<f64>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "INT2" => {
                let r: Result<Option<i16>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "INT4" => {
                let r: Result<Option<i32>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "INT8" => {
                let r: Result<Option<i64>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "TEXT" | "VARCHAR" | "BPCHAR" | "CHAR" | "UUID" => {
                let r: Result<Option<String>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "TIME" => {
                let r: Result<Option<chrono::NaiveTime>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "DATE" => {
                let r: Result<Option<chrono::NaiveDate>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "TIMESTAMP" => {
                let r: Result<Option<chrono::NaiveDateTime>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "TIMESTAMPTZ" => {
                let r: Result<Option<chrono::NaiveDateTime>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
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
        for row in self {
            let mut m = serde_json::Map::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v:PgValueRef = convert_result(row.try_get_raw(key))?;
                m.insert(key.to_owned(), v.try_to_json()?);
            }
            arr.push(serde_json::Value::Object(m));
        }
        Ok(serde_json::Value::from(arr))
    }
}