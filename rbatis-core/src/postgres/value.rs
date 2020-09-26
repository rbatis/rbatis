use std::str::from_utf8;
use geo_types::{Point};

use crate::decode::Decode;
use crate::error::UnexpectedNullError;
use crate::postgres::{PgTypeInfo, Postgres};
use crate::Result;
use crate::types::BigDecimal;
use crate::value::RawValue;

#[derive(Debug, Copy, Clone)]
pub enum PgData<'c> {
    Binary(&'c [u8]),
    Text(&'c str),
}

#[derive(Debug, Clone)]
pub struct PgValue<'c> {
    type_info: Option<PgTypeInfo>,
    data: Option<PgData<'c>>,
}

impl<'c> PgValue<'c> {
    /// Gets the binary or text data for this value; or, `UnexpectedNullError` if this
    /// is a `NULL` value.
    pub(crate) fn try_get(&self) -> crate::Result<PgData<'c>> {
        match self.data {
            Some(data) => Ok(data),
            None => Err(crate::Error::decode(UnexpectedNullError)),
        }
    }

    /// Gets the binary or text data for this value; or, `None` if this
    /// is a `NULL` value.
    #[inline]
    pub fn get(&self) -> Option<PgData<'c>> {
        self.data
    }

    pub(crate) fn null() -> Self {
        Self {
            type_info: None,
            data: None,
        }
    }

    pub(crate) fn bytes(type_info: PgTypeInfo, buf: &'c [u8]) -> Self {
        Self {
            type_info: Some(type_info),
            data: Some(PgData::Binary(buf)),
        }
    }

    pub(crate) fn utf8(type_info: PgTypeInfo, buf: &'c [u8]) -> crate::Result<Self> {
        Ok(Self {
            type_info: Some(type_info),
            data: Some(PgData::Text(from_utf8(&buf).map_err(crate::Error::decode)?)),
        })
    }

    #[cfg(test)]
    pub(crate) fn from_bytes(buf: &'c [u8]) -> Self {
        Self {
            type_info: None,
            data: Some(PgData::Binary(buf)),
        }
    }

    pub(crate) fn from_str(s: &'c str) -> Self {
        Self {
            type_info: None,
            data: Some(PgData::Text(s)),
        }
    }
}

impl<'c> RawValue<'c> for PgValue<'c> {
    type Database = Postgres;

    // The public type_info is used for type compatibility checks
    fn type_info(&self) -> Option<PgTypeInfo> {
        // For TEXT encoding the type defined on the value is unreliable
        if matches!(self.data, Some(PgData::Binary(_))) {
            self.type_info.clone()
        } else {
            None
        }
    }

    fn try_to_json(&self) -> Result<serde_json::Value> {
        if self.type_info.is_none() {
            return Ok(serde_json::Value::Null);
        }
        let type_string = format!("{}", self.type_info.as_ref().unwrap());
        match type_string.as_str() {
            "NUMERIC" => {
                //decimal
                let r: crate::Result<BigDecimal> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                return Ok(serde_json::Value::from(r.unwrap().to_string()));
            }
            "BOOL" => {
                let r: crate::Result<bool> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "BYTEA" => {
                unimplemented!();
            }
            "FLOAT4" => {
                let r: crate::Result<f32> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "FLOAT8" => {
                let r: crate::Result<f64> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "INT2" => {
                let r: crate::Result<i16> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "INT4" => {
                let r: crate::Result<i32> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "INT8" => {
                let r: crate::Result<i64> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "TEXT" | "VARCHAR" | "BPCHAR" | "CHAR" => {
                let r: crate::Result<String> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "UUID" => {
                let r: crate::Result<String> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }

            "TIME" => {
                let r: crate::Result<chrono::NaiveTime> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "DATE" => {
                let r: crate::Result<chrono::NaiveDate> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "TIMESTAMP" => {
                let r: crate::Result<chrono::NaiveDateTime> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
            "TIMESTAMPTZ" => {
                let r: crate::Result<chrono::NaiveDateTime> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }
             "POINT" =>{
                let r: crate::Result<Point<f64>> = Decode::<'_, Postgres>::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap());
                }
                let t = serde_json::to_value(&r.unwrap());
                return Ok(t.unwrap_or(serde_json::Value::Null));
            }

            _ => return Err(crate::Error::from(format!("un support database type for:{:?}!", type_string))),
        }
    }
}
