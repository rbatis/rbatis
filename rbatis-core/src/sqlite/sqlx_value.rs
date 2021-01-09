use serde_json::{json, Value};
use sqlx_core::column::Column;
use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::row::Row;
use sqlx_core::sqlite::{Sqlite, SqliteValue, SqliteValueRef};
use sqlx_core::sqlite::SqliteRow;
use sqlx_core::type_info::TypeInfo;
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec, ResultCodec};

impl<'c> JsonCodec for SqliteValueRef<'c> {
    fn try_to_json(self) -> crate::Result<serde_json::Value> {
        let type_string = self.type_info().name().to_owned();
        return match type_string.as_str() {
            "NULL" => {
                Ok(serde_json::Value::Null)
            }
            "TEXT" => {
                let r: Option<String> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(json!(r));
            }
            "BOOLEAN" => {
                let r: Option<bool> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(json!(r));
            }
            "INTEGER" => {
                let r: Option<i64> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(json!(r));
            }
            "REAL" => {
                let r: Option<f64> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(json!(r));
            }
            "BLOB" => {
                let r: Option<Vec<u8>> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(json!(r));
            }
            _ => {
                //TODO "NUMERIC" |"DATE" | "TIME" | "DATETIME"
                //you can use already supported types to decode this
                let r: Option<Vec<u8>> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(json!(r));
            }
        };
    }
}

impl RefJsonCodec for Vec<SqliteRow> {
    fn try_to_json(&self) -> crate::Result<serde_json::Value> {
        let mut arr = Vec::with_capacity(self.len());
        for row in self {
            let mut m = serde_json::Map::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v: SqliteValueRef = row.try_get_raw(key)?;
                m.insert(key.to_owned(), v.try_to_json()?);
            }
            arr.push(serde_json::Value::Object(m));
        }
        Ok(serde_json::Value::from(arr))
    }
}
