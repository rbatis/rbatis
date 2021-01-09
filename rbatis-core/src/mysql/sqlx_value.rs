use serde_json::{json, Value};
use sqlx_core::column::Column;
use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::mysql::{MySql, MySqlRow, MySqlValue, MySqlValueRef};
use sqlx_core::row::Row;
use sqlx_core::type_info::TypeInfo;
use sqlx_core::types::{BigDecimal, Json};
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec, ResultCodec};

impl<'r> JsonCodec for sqlx_core::mysql::MySqlValueRef<'r> {
    fn try_to_json(self) -> crate::Result<serde_json::Value> {
        let type_string = self.type_info().name().to_owned();
        match type_string.as_str() {
            "NULL" => {
                return Ok(serde_json::Value::Null);
            }
            "DECIMAL" => {
                let r: Option<BigDecimal> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "BIGINT UNSIGNED" => {
                let r: Option<u64> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "BIGINT" => {
                let r: Option<i64> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "INT UNSIGNED" | "MEDIUMINT UNSIGNED" => {
                let r: Option<u32> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "INT" | "MEDIUMINT" => {
                let r: Option<i32> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "SMALLINT" => {
                let r: Option<i16> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "SMALLINT UNSIGNED" => {
                let r: Option<u16> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "TINYINT UNSIGNED" => {
                let r: Option<u8> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "TINYINT" => {
                let r: Option<i8> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "FLOAT" => {
                let r: Option<f32> = Decode::<'_, MySql>::decode(self)?;

                return Ok(json!(r));
            }
            "DOUBLE" => {
                let r: Option<f64> = Decode::<'_, MySql>::decode(self)?;

                return Ok(json!(r));
            }
            "BINARY" | "VARBINARY" | "CHAR" | "VARCHAR" | "TEXT" | "ENUM" => {
                let r: Option<String> = Decode::<'_, MySql>::decode(self)?;

                return Ok(json!(r));
            }
            "BLOB" | "TINYBLOB" | "MEDIUMBLOB" | "LONGBLOB" | "TINYTEXT" | "MEDIUMTEXT" | "LONGTEXT" => {
                let r: Option<Vec<u8>> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "BIT" | "BOOLEAN" => {
                let r: Option<u8> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "DATE" => {
                let r: Option<chrono::NaiveDate> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "TIME" | "YEAR" => {
                let r: Option<chrono::NaiveTime> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "DATETIME" => {
                let r: Option<chrono::NaiveDateTime> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "TIMESTAMP" => {
                let r: Option<chrono::NaiveDateTime> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            "JSON" => {
                let r: Option<Json<serde_json::Value>> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
            _ => {
                //TODO "GEOMETRY" support. for now you can use already supported types to decode this
                let r: Option<Vec<u8>> = Decode::<'_, MySql>::decode(self)?;
                return Ok(json!(r));
            }
        }
    }
}


impl RefJsonCodec for Vec<MySqlRow> {
    fn try_to_json(&self) -> crate::Result<serde_json::Value> {
        let mut arr = Vec::with_capacity(self.len());
        for row in self {
            let mut m = serde_json::Map::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v: MySqlValueRef = row.try_get_raw(key)?;
                m.insert(key.to_owned(), v.try_to_json()?);
            }
            arr.push(serde_json::Value::Object(m));
        }
        Ok(json!(arr))
    }
}