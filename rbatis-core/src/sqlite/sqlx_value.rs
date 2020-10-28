use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::sqlite::{Sqlite, SqliteValue, SqliteValueRef};
use sqlx_core::type_info::TypeInfo;
use sqlx_core::types::BigDecimal;
use sqlx_core::value::ValueRef;

use crate::convert::JsonCodec;
use crate::sqlite::type_info::SqliteType;

impl<'c> JsonCodec for SqliteValueRef<'c> {
    fn try_to_json(self) -> crate::Result<serde_json::Value> {
        //TODO batter way to match type replace use string match
        let type_string = self.type_info().name().to_owned();
        return match type_string.as_str() {
            "NULL" => {
                Ok(serde_json::Value::Null)
            }
            "TEXT" => {
                let r: Result<String, BoxDynError> = Decode::<'_, Sqlite>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                Ok(serde_json::Value::from(r.unwrap()))
            }
            "BOOLEAN" => {
                let r: Result<bool, BoxDynError> = Decode::<'_, Sqlite>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                Ok(serde_json::Value::from(r.unwrap()))
            }
            "INTEGER" => {
                let r: Result<i64, BoxDynError> = Decode::<'_, Sqlite>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                Ok(serde_json::Value::from(r.unwrap()))
            }
            "REAL" => {
                let r: Result<f64, BoxDynError> = Decode::<'_, Sqlite>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                Ok(serde_json::Value::from(r.unwrap()))
            }
            "NUMERIC" => {
                //TODO impl type
                unimplemented!()
            }
            "DATE" => {
                //TODO impl type
                unimplemented!()
            }
            "TIME" => {
                //TODO impl type
                unimplemented!()
            }
            "DATETIME" => {
                //TODO impl type
                unimplemented!()
            }
            "BLOB" => {
                unimplemented!()
            }
            _ => {
                unimplemented!()
            }
        };
    }
}
