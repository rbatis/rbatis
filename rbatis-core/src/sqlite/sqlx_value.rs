use rbson::{bson, Bson, to_bson};
use rbson::spec::BinarySubtype;
use chrono::{Utc};
use sqlx_core::column::Column;
use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::row::Row;
use sqlx_core::sqlite::SqliteRow;
use sqlx_core::sqlite::{Sqlite, SqliteValue, SqliteValueRef};
use sqlx_core::type_info::TypeInfo;
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec, ResultCodec};
use crate::{to_bson_macro};
use crate::db::db_adapter::DataDecoder;

impl<'c> JsonCodec for SqliteValueRef<'c> {
    fn try_to_bson(self) -> crate::Result<Bson> {
        let type_string = self.type_info().name().to_owned();
        return match type_string.as_str() {
            "NULL" => Ok(Bson::Null),
            "TEXT" => {
                let r: Option<String> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "BOOLEAN" => {
                let r: Option<bool> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "INTEGER" => {
                let r: Option<i64> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "REAL" => {
                let r: Option<f64> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "BLOB" => {
                let r: Option<Vec<u8>> = Decode::<'_, Sqlite>::decode(self)?;
                if let Some(r) = r {
                    return Ok(Bson::Binary(rbson::Binary {
                        subtype: BinarySubtype::Generic,
                        bytes: r,
                    }));
                }
                return Ok(Bson::Null);
            }
            "DATE" => {
                let r: Option<chrono::NaiveDate> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(to_bson(&r).unwrap_or_default());
            }
            "TIME" => {
                let r: Option<chrono::NaiveTime> = Decode::<'_, Sqlite>::decode(self)?;
                return Ok(to_bson(&r).unwrap_or_default());
            }
            "DATETIME" => {
                let r: Option<chrono::NaiveDateTime> = Decode::<'_, Sqlite>::decode(self)?;
                if let Some(dt) = r {
                    return Ok(Bson::String(dt.format("%Y-%m-%dT%H:%M:%S").to_string()));
                }
                return Ok(Bson::Null);
            }
            _ => {
                //TODO "NUMERIC"
                //you can use already supported types to decode this
                let r: Option<Vec<u8>> = Decode::<'_, Sqlite>::decode(self)?;
                if let Some(r) = r {
                    return Ok(Bson::Binary(rbson::Binary {
                        subtype: BinarySubtype::Generic,
                        bytes: r,
                    }));
                }
                return Ok(Bson::Null);
            }
        };
    }
}

impl RefJsonCodec for Vec<SqliteRow> {
    fn try_to_bson(&self, decoder: &dyn DataDecoder) -> crate::Result<Bson> {
        let mut arr = Vec::with_capacity(self.len());
        for row in self {
            let mut m = rbson::Document::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v: SqliteValueRef = row.try_get_raw(key)?;
                let mut bson = v.try_to_bson()?;
                decoder.decode(key, &mut bson)?;
                m.insert(key.to_owned(), bson);
            }
            arr.push(Bson::Document(m));
        }
        Ok(Bson::from(arr))
    }
}
