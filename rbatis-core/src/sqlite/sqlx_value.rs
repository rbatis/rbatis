use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::sqlite::{Sqlite, SqliteValue, SqliteValueRef};
use sqlx_core::type_info::TypeInfo;
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec};
use sqlx_core::sqlite::SqliteRow;
use sqlx_core::row::Row;
use sqlx_core::column::Column;
use crate::db::convert_result;
use serde_json::{json, Value};

impl<'c> JsonCodec for SqliteValueRef<'c> {
    fn try_to_json(self) -> crate::Result<serde_json::Value> {
        let type_string = self.type_info().name().to_owned();
        return match type_string.as_str() {
            "NULL" => {
                Ok(serde_json::Value::Null)
            }
            "TEXT" => {
                let r: Result<Option<String>, BoxDynError> = Decode::<'_, Sqlite>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "BOOLEAN" => {
                let r: Result<Option<bool>, BoxDynError> = Decode::<'_, Sqlite>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "INTEGER" => {
                let r: Result<Option<i64>, BoxDynError> = Decode::<'_, Sqlite>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "REAL" => {
                let r: Result<Option<f64>, BoxDynError> = Decode::<'_, Sqlite>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                 return Ok(json!(r.unwrap()));
            }
            "BLOB" => {
                let r: Result<Option<Vec<u8>>, BoxDynError> = Decode::<'_, Sqlite>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            _ => {
                //TODO "NUMERIC" |"DATE" | "TIME" | "DATETIME"
                //you can use already supported types to decode this
                return Err(crate::Error::from(format!("un support database type for:{:?}!", type_string)));
            }
        };
    }
}

impl RefJsonCodec for Vec<SqliteRow>{
    fn try_to_json(&self) -> crate::Result<serde_json::Value> {
        let mut arr = vec![];
        for row in self {
            let mut m = serde_json::Map::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v:SqliteValueRef = convert_result( row.try_get_raw(key))?;
                m.insert(key.to_owned(), v.try_to_json()?);
            }
            arr.push(serde_json::Value::Object(m));
        }
        Ok(serde_json::Value::from(arr))
    }
}
