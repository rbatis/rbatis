use rbson::{bson, Bson, to_bson};
use sqlx_core::column::Column;
use sqlx_core::decode::Decode;
use sqlx_core::error::BoxDynError;
use sqlx_core::postgres::types::{PgMoney, PgTimeTz};
use sqlx_core::postgres::PgRow;
use sqlx_core::postgres::{PgValue, PgValueRef, Postgres};
use sqlx_core::row::Row;
use sqlx_core::type_info::TypeInfo;
use sqlx_core::types::chrono::{FixedOffset, NaiveTime};
use sqlx_core::types::{BigDecimal, Json, Uuid};
use sqlx_core::value::ValueRef;

use crate::convert::{JsonCodec, RefJsonCodec, ResultCodec};
use crate::postgres::PgInterval;
use chrono::{Utc};

use crate::{to_bson_macro};
use std::option::Option::Some;
use rbson::spec::BinarySubtype;
use crate::db::db_adapter::DataDecoder;

impl<'c> JsonCodec for PgValueRef<'c> {
    fn try_to_bson(self) -> crate::Result<Bson> {
        match self.type_info().name() {
            "VOID" => {
                return Ok(Bson::Null);
            }
            "NUMERIC" => {
                //decimal
                let r: Option<BigDecimal> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(date) = r {
                    return Ok(Bson::String(date.to_string()));
                }
                return Ok(Bson::Null);
            }
            "NUMERIC[]" => {
                //decimal
                let r: Option<Vec<BigDecimal>> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(r) = r {
                    return Ok(to_bson(&r).unwrap_or_default());
                }
                return Ok(Bson::Null);
            }
            "MONEY" => {
                //decimal
                let r: Option<String> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "MONEY[]" => {
                //decimal
                let r: Option<Vec<String>> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "BOOL" => {
                let r: Option<bool> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "BOOL[]" => {
                let r: Vec<bool> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(r.into());
            }
            "BYTEA" | "BYTEA[]" => {
                let r: Option<Vec<u8>> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(r) = r {
                    return Ok(Bson::Binary(rbson::Binary {
                        subtype: BinarySubtype::Generic,
                        bytes: r,
                    }));
                }
                return Ok(Bson::Null);
            }
            "FLOAT4" => {
                let r: Option<f32> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "FLOAT4[]" => {
                let r: Option<Vec<f32>> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "FLOAT8" => {
                let r: Option<f64> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "FLOAT8[]" => {
                let r: Option<Vec<f64>> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "INT2" => {
                let r: Option<i16> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(r) = r {
                    return Ok(bson!(r as i32));
                }
                return Ok(Bson::Null);
            }
            "INT2[]" => {
                let r: Option<Vec<i16>> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(r) = r {
                    return Ok(to_bson(&r).unwrap_or_default());
                }
                return Ok(Bson::Null);
            }
            "INT4" => {
                let r: Option<i32> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "INT4[]" => {
                let r: Option<Vec<i32>> = Decode::<'_, Postgres>::decode(self)?;

                return Ok(to_bson_macro!(r));
            }
            "INT8" => {
                let r: Option<i64> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "INT8[]" => {
                let r: Option<Vec<i64>> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "OID" => {
                let r: Option<u32> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "OID[]" => {
                let r: Option<Vec<u32>> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "TEXT" | "NAME" | "VARCHAR" | "BPCHAR" | "CHAR" | "\"CHAR\"" | "UNKNOWN" => {
                let r: Option<String> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "TEXT[]" | "CHAR[]" | "VARCHAR[]" | "\"CHAR\"[]" | "NAME[]" => {
                let r: Option<Vec<String>> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson_macro!(r));
            }
            "UUID" => {
                let r: Option<Uuid> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(r) = r {
                    return Ok(to_bson(&r).unwrap_or_default());
                }
                return Ok(Bson::Null);
            }
            "UUID[]" => {
                let r: Option<Vec<Uuid>> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(r) = r {
                    let mut arr = vec![];
                    for x in r {
                        arr.push(to_bson(&x).unwrap_or_default());
                    }
                    return Ok(Bson::from(arr));
                }
                return Ok(Bson::Null);
            }
            "JSON" | "JSONB" => {
                let r: Option<Json<serde_json::Value>> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(r) = r {
                    return Ok(to_bson(&r.0).unwrap_or_default());
                }
                return Ok(Bson::Null);
            }
            "JSON[]" | "JSONB[]" => {
                let r: Option<Vec<Json<serde_json::Value>>> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(r) = r {
                    let mut arr = Vec::with_capacity(r.capacity());
                    for x in r {
                        arr.push(x.0);
                    }
                    return Ok(to_bson(&arr).unwrap_or_default());
                }
                return Ok(Bson::Null);
            }
            "TIME" => {
                let r: Option<NaiveTime> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson(&r).unwrap_or_default());
            }
            "TIME[]" => {
                let r: Option<Vec<NaiveTime>> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson(&r).unwrap_or_default());
            }
            "DATE" => {
                let r: Option<chrono::NaiveDate> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson(&r).unwrap_or_default());
            }
            "DATE[]" => {
                let r: Option<Vec<chrono::NaiveDate>> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson(&r).unwrap_or_default());
            }
            "TIMESTAMP" => {
                let r: Option<chrono::NaiveDateTime> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(dt) = r {
                    return Ok(Bson::String(dt.format("%Y-%m-%dT%H:%M:%S").to_string()));
                }
                return Ok(Bson::Null);
            }
            "TIMESTAMP[]" => {
                let r: Option<Vec<chrono::NaiveDateTime>> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(r) = r {
                    let mut dts = vec![];
                    for dt in r {
                        dts.push(Bson::String(dt.format("%Y-%m-%dT%H:%M:%S").to_string()));
                    }
                    return Ok(Bson::Array(dts));
                }
                return Ok(Bson::Null);
            }
            "TIMESTAMPTZ" => {
                let r: Option<chrono::DateTime<Utc>> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(dt) = r {
                    return Ok(Bson::String(dt.to_string()));
                }
                return Ok(Bson::Null);
            }
            "TIMESTAMPTZ[]" => {
                let r: Option<Vec<chrono::DateTime<Utc>>> = Decode::<'_, Postgres>::decode(self)?;
                if let Some(r) = r {
                    let mut dts = vec![];
                    for x in r {
                        let dt = rbson::DateTime::from_chrono(x);
                        dts.push(Bson::String(dt.to_string()));
                    }
                    return Ok(Bson::Array(dts));
                }
                return Ok(Bson::Null);
            }
            "INTERVAL" => {
                let r: Option<sqlx_core::postgres::types::PgInterval> =
                    Decode::<'_, Postgres>::decode(self)?;
                if r.is_none() {
                    return Ok(Bson::Null);
                }
                return Ok(to_bson(&PgInterval::from(r.unwrap())).unwrap_or_default());
            }
            "VARBIT" | "BIT" => {
                let r: Option<bit_vec::BitVec> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson(&r).unwrap_or_default());
            }
            "VARBIT[]" | "BIT[]" => {
                let r: Option<Vec<bit_vec::BitVec>> = Decode::<'_, Postgres>::decode(self)?;
                return Ok(to_bson(&r).unwrap_or_default());
            }
            _ => {
                //TODO
                //"CIDR" | "INET"
                //"CIDR[]" | "INET[]"
                // "JSONPATH","JSONPATH[]",
                // "INT8RANGE","INT8RANGE[]",
                // "DATERANGE","DATERANGE[]",
                // "TSTZRANGE","TSTZRANGE[]",
                // "TSRANGE","TSRANGE[]",
                // "NUMRANGE","NUMRANGE[]",
                // "INT4RANGE","INT4RANGE[]",
                // "RECORD","RECORD[]"
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
                //  you can use already Vec<u8> types to decode this
                let r: Option<Vec<u8>> = Decode::<'_, Postgres>::decode(self)?;
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

impl RefJsonCodec for Vec<PgRow> {
    fn try_to_bson(&self, decoder: &dyn DataDecoder) -> crate::Result<Bson> {
        let mut arr = Vec::with_capacity(self.len());
        for row in self {
            let mut m = rbson::Document::new();
            let columns = row.columns();
            for x in columns {
                let key = x.name();
                let v: PgValueRef = row.try_get_raw(key)?;
                let mut bson=v.try_to_bson()?;
                decoder.decode(key,&mut bson)?;
                m.insert(key.to_owned(), bson);
            }
            arr.push(Bson::Document(m));
        }
        Ok(Bson::from(arr))
    }
}
