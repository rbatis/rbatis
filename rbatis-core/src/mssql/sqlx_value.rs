use sqlx_core::column::Column;
use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::mssql::{Mssql, MssqlRow, MssqlValue, MssqlValueRef};
use sqlx_core::row::Row;
use sqlx_core::type_info::TypeInfo;
use sqlx_core::types::BigDecimal;
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec};
use crate::db_adapter::convert_result;
use serde_json::{json, Value};

impl<'r> JsonCodec for sqlx_core::mssql::MssqlValueRef<'r> {
    fn try_to_json(self) -> crate::Result<serde_json::Value> {
        //TODO batter way to match type replace use string match
        let type_string = self.type_info().name().to_owned();
        match type_string.as_str() {
            "NULL" => {
                return Ok(serde_json::Value::Null);
            }
            "TINYINT" => {
                let r: Result<Option<i8>, BoxDynError> = Decode::<'_, Mssql>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            "SMALLINT" => {
                let r: Result<Option<i16>, BoxDynError> = Decode::<'_, Mssql>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            "INT" => {
                let r: Result<Option<i32>, BoxDynError> = Decode::<'_, Mssql>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            "BIGINT" => {
                let r: Result<Option<i64>, BoxDynError> = Decode::<'_, Mssql>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            "REAL" => {
                let r: Result<Option<f32>, BoxDynError> = Decode::<'_, Mssql>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            "FLOAT" => {
                let r: Result<Option<f64>, BoxDynError> = Decode::<'_, Mssql>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }

            "VARCHAR" | "NVARCHAR" | "BIGVARCHAR" | "CHAR" | "BIGCHAR" | "NCHAR" => {
                let r: Result<Option<String>, BoxDynError> = Decode::<'_, Mssql>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }

            //TODO convert types
            // "NEWDECIMAL" => {
            //     let r: Result<BigDecimal, BoxDynError> = Decode::<'_, Mssql>::decode(self);
            //     if r.is_err() {
            //         return Err(crate::Error::from(r.err().unwrap().to_string()));
            //     }
            //     return Ok(serde_json::Value::from(r.unwrap().to_string()));
            // }
            // "DATE" => {
            //     let r: Result<chrono::NaiveDate, BoxDynError> = Decode::<'_, Mssql>::decode(self);
            //     if r.is_err() {
            //         return Err(crate::Error::from(r.err().unwrap().to_string()));
            //     }
            //     let t = serde_json::to_value(&r.unwrap());
            //     return Ok(t.unwrap_or(serde_json::Value::Null));
            // }
            // "TIME" => {
            //     let r: Result<chrono::NaiveTime, BoxDynError> = Decode::<'_, Mssql>::decode(self);
            //     if r.is_err() {
            //         return Err(crate::Error::from(r.err().unwrap().to_string()));
            //     }
            //     let t = serde_json::to_value(&r.unwrap());
            //     return Ok(t.unwrap_or(serde_json::Value::Null));
            // }
            // "DATETIME" => {
            //     let r: Result<chrono::NaiveDateTime, BoxDynError> = Decode::<'_, Mssql>::decode(self);
            //     if r.is_err() {
            //         return Err(crate::Error::from(r.err().unwrap().to_string()));
            //     }
            //     let t = serde_json::to_value(&r.unwrap());
            //     return Ok(t.unwrap_or(serde_json::Value::Null));
            // }
            // "TIMESTAMP" => {
            //     let r: Result<chrono::NaiveDateTime, BoxDynError> = Decode::<'_, Mssql>::decode(self);
            //     if r.is_err() {
            //         return Err(crate::Error::from(r.err().unwrap().to_string()));
            //     }
            //     let t = serde_json::to_value(&r.unwrap());
            //     return Ok(t.unwrap_or(serde_json::Value::Null));
            // }
            _ => return Err(crate::Error::from(format!("un support database type for:{:?}!", type_string))),
        }
    }
}


impl RefJsonCodec for Vec<MssqlRow> {
    fn try_to_json(&self) -> crate::Result<serde_json::Value> {
        let mut arr = vec![];
        for row in self {
            let mut m = serde_json::Map::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v: MssqlValueRef = convert_result(row.try_get_raw(key))?;
                m.insert(key.to_owned(), v.try_to_json()?);
            }
            arr.push(serde_json::Value::Object(m));
        }
        Ok(serde_json::Value::from(arr))
    }
}