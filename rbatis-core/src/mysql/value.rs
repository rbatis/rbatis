use crate::error::UnexpectedNullError;
use crate::mysql::{MySql, MySqlTypeInfo};
use crate::value::RawValue;
use serde_json::Value;
use crate::mysql::protocol::TypeId;
use crate::decode::Decode;
use crate::types::BigDecimal;

#[derive(Debug, Copy, Clone)]
pub enum MySqlData<'c> {
    Binary(&'c [u8]),
    Text(&'c [u8]),
}

#[derive(Debug, Clone)]
pub struct MySqlValue<'c> {
    type_info: Option<MySqlTypeInfo>,
    data: Option<MySqlData<'c>>,
}

impl<'c> MySqlValue<'c> {
    /// Gets the binary or text data for this value; or, `UnexpectedNullError` if this
    /// is a `NULL` value.
    pub(crate) fn try_get(&self) -> crate::Result<MySqlData<'c>> {
        match self.data {
            Some(data) => Ok(data),
            None => Err(crate::Error::decode(UnexpectedNullError)),
        }
    }

    /// Gets the binary or text data for this value; or, `None` if this
    /// is a `NULL` value.
    #[inline]
    pub fn get(&self) -> Option<MySqlData<'c>> {
        self.data
    }

    pub(crate) fn null() -> Self {
        Self {
            type_info: None,
            data: None,
        }
    }

    pub(crate) fn binary(type_info: MySqlTypeInfo, buf: &'c [u8]) -> Self {
        Self {
            type_info: Some(type_info),
            data: Some(MySqlData::Binary(buf)),
        }
    }

    pub(crate) fn text(type_info: MySqlTypeInfo, buf: &'c [u8]) -> Self {
        Self {
            type_info: Some(type_info),
            data: Some(MySqlData::Text(buf)),
        }
    }
}

impl<'c> RawValue<'c> for MySqlValue<'c> {
    type Database = MySql;

    fn type_info(&self) -> Option<MySqlTypeInfo> {
        self.type_info.clone()
    }

    fn try_to_json(&self) -> Result<serde_json::Value, String> {
        if self.type_info.is_none() {
            return Ok(serde_json::Value::Null);
        }
        //TODO batter way to match type replace use string match
        let type_string = format!("{}", self.type_info.as_ref().unwrap());
        match type_string.as_str() {
            "NULL" => return Ok(serde_json::Value::Null),
            "NEWDECIMAL" => {
                let r = BigDecimal::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap().to_string()));
            }
            "BIGINT UNSIGNED" => {
                let r = u64::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "BIGINT" => {
                let r = i64::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "INT UNSIGNED" => {
                let r = u32::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "INT" => {
                let r = i32::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "SMALLINT" => {
                let r = i16::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "SMALLINT UNSIGNED" => {
                let r = u16::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "TINYINT UNSIGNED" => {
                let r = u8::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "TINYINT" => {
                let r = i8::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "FLOAT" => {
                let r = f64::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "DOUBLE" => {
                let r = f64::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "BINARY" | "VARBINARY" | "BLOB" | "CHAR" | "VARCHAR" | "TEXT" => {
                let r = String::decode(self.clone());
                if r.is_err() {
                    return Err(r.err().unwrap().to_string());
                }
                return Ok(serde_json::Value::from(r.unwrap()));
            }
            "DATE" | "TIME" | "DATETIME" | "TIMESTAMP" => {
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
