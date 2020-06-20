use crate::error::UnexpectedNullError;
use crate::postgres::{PgTypeInfo, Postgres};
use crate::value::RawValue;
use std::str::from_utf8;
use serde_json::Value;
use crate::decode::Decode;
use serde_json::value::Value::Array;
use time::{date, offset, Date, NumericalDuration, OffsetDateTime, PrimitiveDateTime, Time};

#[derive(Debug, Copy, Clone)]
pub enum PgData<'c> {
    Binary(&'c [u8]),
    Text(&'c str),
}

#[derive(Debug,Clone)]
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

    fn try_to_json(&self) -> Result<Value, String> {
        if self.type_info.is_none() {
            return Ok(serde_json::Value::Null);
        }
        //TODO batter way to match type replace use string match
        if self.type_info == None {
            return return Ok(serde_json::Value::Null);
        }
        let type_string = format!("{}", self.type_info.as_ref().unwrap());
        match type_string.as_str() {
            "NUMERIC" =>{
                //decimal
                let r = String::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "BOOL" => {
                let r = bool::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "BYTEA" => {
               unimplemented!();
            }
            "TIME" | "DATE" | "TIMESTAMP" | "TIMESTAMPTZ" => {
                let r = String::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "FLOAT4" =>{
                let r = f32::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "FLOAT8" =>{
                let r = f64::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "INT4" =>{
                let r = i32::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "INT8" =>{
                let r = i64::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "TEXT" => {
                let r = String::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "UUID" => {
                let r = String::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            _ => return Err(format!("un support database type for:{}!",type_string).to_string()),
        }
    }
}
