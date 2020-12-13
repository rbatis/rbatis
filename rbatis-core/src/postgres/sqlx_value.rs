use serde_json::{json, Value};
use sqlx_core::column::Column;
use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::postgres::{PgValue, PgValueRef, Postgres};
use sqlx_core::postgres::PgRow;
use sqlx_core::postgres::types::{PgMoney, PgTimeTz};
use sqlx_core::row::Row;
use sqlx_core::type_info::TypeInfo;
use sqlx_core::types::{BigDecimal, Json, Uuid};
use sqlx_core::types::chrono::{FixedOffset, NaiveTime};
use sqlx_core::types::ipnetwork::IpNetwork;
use sqlx_core::types::time::Time;
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec};
use crate::db::convert_result;
use crate::postgres::PgInterval;

impl<'c> JsonCodec for PgValueRef<'c> {
    fn try_to_json(self) -> crate::Result<serde_json::Value> {
        let type_string = self.type_info().name().to_owned();
        match type_string.as_str() {
            "VOID" => {
                return Ok(serde_json::Value::Null);
            }
            "NUMERIC" => {
                //decimal
                let r: Result<Option<BigDecimal>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                if r.as_ref().unwrap().is_none() {
                    return Ok(serde_json::Value::Null);
                }
                return Ok(json!(r.unwrap().unwrap().to_string()));
            }
            "NUMERIC[]" => {
                //decimal
                let r: Result<Option<Vec<BigDecimal>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                if r.as_ref().unwrap().is_none() {
                    return Ok(serde_json::Value::Null);
                }
                let data = r.unwrap().unwrap();
                let mut datas = vec![];
                for x in data {
                    datas.push(x.to_string());
                }
                return Ok(json!(datas));
            }
            "MONEY" => {
                //decimal
                let r: Result<Option<PgMoney>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                if r.as_ref().unwrap().is_none() {
                    return Ok(serde_json::Value::Null);
                }
                return Ok(json!(r.unwrap().unwrap().0.to_string()));
            }
            "MONEY[]" => {
                //decimal
                let r: Result<Option<Vec<PgMoney>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                if r.as_ref().unwrap().is_none() {
                    return Ok(serde_json::Value::Null);
                }
                let data = r.unwrap().unwrap();
                let mut datas = vec![];
                for x in data {
                    datas.push(x.0.to_string());
                }
                return Ok(json!(datas));
            }
            "BOOL" => {
                let r: Result<Option<bool>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            "BOOL[]" => {
                let r: Result<Vec<bool>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            "BYTEA" | "BYTEA[]" => {
                let r: Result<Option<Vec<u8>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            "FLOAT4" => {
                let r: Result<Option<f32>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            "FLOAT4[]" => {
                let r: Result<Option<Vec<f32>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
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
            "FLOAT8[]" => {
                let r: Result<Option<Vec<f64>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
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
            "INT2[]" => {
                let r: Result<Option<Vec<i16>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
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
            "INT4[]" => {
                let r: Result<Option<Vec<i32>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
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
            "INT8[]" => {
                let r: Result<Option<Vec<i64>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }

            "OID" => {
                let r: Result<Option<u32>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }

            "OID[]" => {
                let r: Result<Option<Vec<u32>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }

            "TEXT" | "NAME" | "VARCHAR" | "BPCHAR" | "CHAR" | "\"CHAR\"" | "UNKNOWN" => {
                let r: Result<Option<String>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }
            "TEXT[]" | "CHAR[]" | "VARCHAR[]" | "\"CHAR\"[]" | "NAME[]" => {
                let r: Result<Option<Vec<String>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }

            "UUID" => {
                let r: Result<Option<Uuid>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }

            "UUID[]" => {
                let r: Result<Option<Vec<Uuid>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                return Ok(json!(r.unwrap()));
            }

            "JSON" | "JSON[]" | "JSONB" | "JSONB[]" => {
                let r: Result<Option<Json<serde_json::Value>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let data = serde_json::to_value(r.unwrap());
                return Ok(data.unwrap_or(serde_json::Value::Null));
            }
            "TIME" => {
                let r: Result<Option<Time>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = json!(&r.unwrap());
                return Ok(t);
            }
            "TIME[]" => {
                let r: Result<Option<Vec<Time>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = json!(&r.unwrap());
                return Ok(t);
            }
            "DATE" => {
                let r: Result<Option<chrono::NaiveDate>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "DATE[]" => {
                let r: Result<Option<Vec<chrono::NaiveDate>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
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
            "TIMESTAMP[]" => {
                let r: Result<Option<Vec<chrono::NaiveDateTime>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
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
            "TIMESTAMPTZ[]" => {
                let r: Result<Option<Vec<chrono::NaiveDateTime>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "CIDR" | "INET" => {
                let r: Result<Option<IpNetwork>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "CIDR[]" | "INET[]" => {
                let r: Result<Option<Vec<IpNetwork>>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }

            "INTERVAL" => {
                let r: Result<Option<sqlx_core::postgres::types::PgInterval>, BoxDynError> = Decode::<'_, Postgres>::decode(self);
                if r.is_err() {
                    return Err(crate::Error::from(r.err().unwrap().to_string()));
                }
                let t = serde_json::to_value(PgInterval::from(r.unwrap().unwrap()));
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            _ => {
                //TODO
                // "JSONPATH","JSONPATH[]",
                // "INT8RANGE","INT8RANGE[]",
                // "DATERANGE","DATERANGE[]",
                // "TSTZRANGE","TSTZRANGE[]",
                // "TSRANGE","TSRANGE[]",
                // "NUMRANGE","NUMRANGE[]",
                // "INT4RANGE","INT4RANGE[]",
                // "RECORD","RECORD[]"
                // ,"VARBIT" "VARBIT[]"
                // "BIT" "BIT[]"
                // "TIMETZ" "TIMETZ[]"
                // "INTERVAL[]"
                // "POINT","POINT[],"
                // LSEG","LSEG[]",
                // "PATH","PATH[]",
                // "BOX","BOX[]",
                // "POLYGON","POLYGON[]",
                // "LINE","LINE[]",
                // "CIRCLE", "CIRCLE[]",
                // "MACADDR8","MACADDR8[]",
                // "MACADDR","MACADDR[]",
                //  you can use already supported types to decode this
                return Err(crate::Error::from(format!("un support database type for:{:?}!", type_string)));
            }
        }
    }
}


impl RefJsonCodec for Vec<PgRow> {
    fn try_to_json(&self) -> crate::Result<serde_json::Value> {
        let mut arr = vec![];
        for row in self {
            let mut m = serde_json::Map::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v: PgValueRef = convert_result(row.try_get_raw(key))?;
                m.insert(key.to_owned(), v.try_to_json()?);
            }
            arr.push(serde_json::Value::Object(m));
        }
        Ok(serde_json::Value::from(arr))
    }
}