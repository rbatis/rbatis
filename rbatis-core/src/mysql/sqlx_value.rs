use rbson::{Binary, Bson, to_bson};
use sqlx_core::column::Column;
use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::mysql::{MySql, MySqlRow, MySqlValue, MySqlValueRef};
use sqlx_core::row::Row;
use sqlx_core::type_info::TypeInfo;
use sqlx_core::types::{BigDecimal, Json};
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec, ResultCodec};
use chrono::{DateTime, Utc, NaiveDate, Local};

use crate::{to_bson_macro};
use rbson::bson;
use rbson::spec::BinarySubtype;
use crate::db::db_adapter::DataDecoder;


impl<'r> JsonCodec for sqlx_core::mysql::MySqlValueRef<'r> {
    fn try_to_bson(self) -> crate::Result<Bson> {
        match self.type_info().name() {
            "NULL" => {
                return Ok(Bson::Null);
            }
            "DECIMAL" => {
                let r: Option<String> = Decode::<'_, MySql>::decode(self)?;
                if let Some(date) = r {
                    return Ok(Bson::String(date));
                }
                return Ok(Bson::Null);
            }
            "BIGINT UNSIGNED" => {
                let r: Option<u64> = Decode::<'_, MySql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(bson!(r));
                }
                return Ok(Bson::Null);
            }
            "BIGINT" => {
                let r: Option<i64> = Decode::<'_, MySql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "INT UNSIGNED" | "MEDIUMINT UNSIGNED" => {
                let r: Option<u32> = Decode::<'_, MySql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "INT" | "MEDIUMINT" => {
                let r: Option<i32> = Decode::<'_, MySql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "SMALLINT" => {
                let r: Option<i16> = Decode::<'_, MySql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(bson!(r as i32));
                }
                return Ok(Bson::Null);
            }
            "SMALLINT UNSIGNED" => {
                let r: Option<u16> = Decode::<'_, MySql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(bson!(r as u32));
                }
                return Ok(Bson::Null);
            }
            "TINYINT UNSIGNED" => {
                let r: Option<u8> = Decode::<'_, MySql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(bson!(r as u32));
                }
                return Ok(Bson::Null);
            }
            "TINYINT" => {
                let r: Option<i8> = Decode::<'_, MySql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(bson!(r as i32));
                }
                return Ok(Bson::Null);
            }
            "FLOAT" => {
                let r: Option<f32> = Decode::<'_, MySql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "DOUBLE" => {
                let r: Option<f64> = Decode::<'_, MySql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "BINARY" | "VARBINARY" | "CHAR" | "VARCHAR" | "TEXT" | "ENUM" => {
                let r: Option<String> = Decode::<'_, MySql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "BLOB" | "TINYBLOB" | "MEDIUMBLOB" | "LONGBLOB" | "TINYTEXT" | "MEDIUMTEXT"
            | "LONGTEXT" => {
                let r: Option<Vec<u8>> = Decode::<'_, MySql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(Bson::Binary(rbson::Binary {
                        subtype: BinarySubtype::Generic,
                        bytes: r,
                    }));
                }
                return Ok(Bson::Null);
            }
            "BIT" => {
                let r: Option<u8> = Decode::<'_, MySql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(bson!(r as i32));
                }
                return Ok(Bson::Null);
            }
            "BOOLEAN" => {
                let r: Option<u8> = Decode::<'_, MySql>::decode(self)?;
                if let Some(r) = r {
                    let mut b = false;
                    if r == 1 {
                        b = true;
                    }
                    return Ok(bson!(b));
                }
                return Ok(Bson::Null);
            }
            "DATE" => {
                let r: Option<chrono::NaiveDate> = Decode::<'_, MySql>::decode(self)?;
                return Ok(to_bson(&r).unwrap_or_default());
            }
            "TIME" | "YEAR" => {
                let r: Option<chrono::NaiveTime> = Decode::<'_, MySql>::decode(self)?;
                return Ok(to_bson(&r).unwrap_or_default());
            }
            "DATETIME" => {
                let r: Option<chrono::NaiveDateTime> = Decode::<'_, MySql>::decode(self)?;
                if let Some(dt) = r {
                    return Ok(Bson::String(dt.format("%Y-%m-%dT%H:%M:%S").to_string()));
                }
                return Ok(Bson::Null);
            }
            "TIMESTAMP" => {
                let r: Option<DateTime<Local>> = Decode::<'_, MySql>::decode(self)?;
                if let Some(dt) = r {
                    return Ok(Bson::String(dt.format("%Y-%m-%dT%H:%M:%S").to_string()));
                }
                return Ok(Bson::Null);
            }
            "JSON" => {
                let r: Option<Json<serde_json::Value>> = Decode::<'_, MySql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(to_bson(&r.0).unwrap_or_default());
                }
                return Ok(Bson::Null);
            }
            _ => {
                //TODO "GEOMETRY" support. for now you can use already supported types to decode this
                let r: Option<Vec<u8>> = Decode::<'_, MySql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(Bson::Binary(rbson::Binary {
                        subtype: BinarySubtype::Generic,
                        bytes: r,
                    }));
                }
                return Ok(Bson::Null);
            }
        }
    }
}

impl RefJsonCodec for Vec<MySqlRow> {
    fn try_to_bson(&self, decoder: &dyn DataDecoder) -> crate::Result<rbson::Bson> {
        let mut arr = Vec::with_capacity(self.len());
        for row in self {
            let mut m = rbson::Document::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v: MySqlValueRef = row.try_get_raw(key)?;
                let mut bson = v.try_to_bson()?;
                decoder.decode(key, &mut bson)?;
                m.insert(key.to_owned(), bson);
            }
            arr.push(rbson::Bson::Document(m));
        }
        Ok(Bson::Array(arr))
    }
}
