use rbson::{bson, Bson};
use sqlx_core::column::Column;
use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::mssql::{Mssql, MssqlRow, MssqlValue, MssqlValueRef};
use sqlx_core::row::Row;
use sqlx_core::type_info::TypeInfo;
use sqlx_core::types::BigDecimal;
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec, ResultCodec};

use crate::db::db_adapter::DataDecoder;
use crate::to_bson_macro;
use crate::types::Decimal;

impl<'r> JsonCodec for sqlx_core::mssql::MssqlValueRef<'r> {
    fn try_to_bson(self) -> crate::Result<Bson> {
        //TODO batter way to match type replace use string match
        match self.type_info().name() {
            "NULL" => {
                return Ok(Bson::Null);
            }
            "TINYINT" => {
                let r: Option<i8> = Decode::<'_, Mssql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(bson!(r as i32));
                }
                return Ok(Bson::Null);
            }
            "SMALLINT" => {
                let r: Option<i16> = Decode::<'_, Mssql>::decode(self)?;
                if let Some(r) = r {
                    return Ok(bson!(r as i32));
                }
                return Ok(Bson::Null);
            }
            "INT" => {
                let r: Option<i32> = Decode::<'_, Mssql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "BIGINT" => {
                let r: Option<i64> = Decode::<'_, Mssql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "REAL" => {
                let r: Option<f32> = Decode::<'_, Mssql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "FLOAT" => {
                let r: Option<f64> = Decode::<'_, Mssql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }

            "VARCHAR" | "NVARCHAR" | "BIGVARCHAR" | "CHAR" | "BIGCHAR" | "NCHAR" => {
                let r: Option<String> = Decode::<'_, Mssql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }

            "NEWDECIMAL" => {
                let r: Option<String> = Decode::<'_, Mssql>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }

            //TODO "DATE" | "TIME" | "DATETIME" | "TIMESTAMP" => {}
            // you can use types package to save date to string
            _ => {
                return Err(crate::Error::from(format!(
                    "un support database type for:{:?}!",
                    self.type_info().name()
                )));
            }
        }
    }
}

impl RefJsonCodec for Vec<MssqlRow> {
    fn try_to_bson(&self, decoder: &dyn DataDecoder) -> crate::Result<Bson> {
        let mut arr = Vec::with_capacity(self.len());
        for row in self {
            let mut m = rbson::Document::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v: MssqlValueRef = row.try_get_raw(key)?;
                let mut bson = v.try_to_bson()?;
                decoder.decode(key, &mut bson)?;
                m.insert(key.to_owned(), bson);
            }
            arr.push(Bson::Document(m));
        }
        Ok(Bson::from(arr))
    }
}
