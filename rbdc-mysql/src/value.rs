use crate::protocol::text::ColumnType;
use crate::result_set::MySqlTypeInfo;
use bytes::Bytes;
use rbdc::Error;
use std::borrow::Cow;
use std::str::from_utf8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MySqlValueFormat {
    Text,
    Binary,
}

/// Implementation of [`Value`] for MySQL.
#[derive(Clone)]
pub struct MySqlValue {
    pub(crate) value: Option<Vec<u8>>,
    pub(crate) type_info: MySqlTypeInfo,
    pub(crate) format: MySqlValueFormat,
}

/// Implementation of [`ValueRef`] for MySQL.
#[derive(Clone)]
pub struct MySqlValueRef<'r> {
    pub(crate) value: Option<&'r [u8]>,
    pub(crate) type_info: MySqlTypeInfo,
    pub(crate) format: MySqlValueFormat,
}

impl MySqlValue {
    pub fn format(&self) -> MySqlValueFormat {
        self.format
    }

    pub fn as_bytes(&self) -> Result<&[u8], Error> {
        match &self.value {
            Some(v) => Ok(v),
            None => Err(Error::protocol("UnexpectedNull")),
        }
    }

    pub fn as_str(&self) -> Result<&str, Error> {
        Ok(from_utf8(self.as_bytes()?)?)
    }
}

impl<'r> MySqlValueRef<'r> {
    pub(crate) fn format(&self) -> MySqlValueFormat {
        self.format
    }

    pub(crate) fn as_bytes(&self) -> Result<&'r [u8], Error> {
        match &self.value {
            Some(v) => Ok(v),
            None => Err(Error::protocol("UnexpectedNull")),
        }
    }

    pub(crate) fn as_str(&self) -> Result<&'r str, Error> {
        Ok(from_utf8(self.as_bytes()?)?)
    }
}

impl MySqlValue {
    fn as_ref(&self) -> MySqlValueRef<'_> {
        MySqlValueRef {
            value: self.value.as_deref(),
            type_info: self.type_info.clone(),
            format: self.format,
        }
    }

    pub fn type_info(&self) -> Cow<'_, MySqlTypeInfo> {
        Cow::Borrowed(&self.type_info)
    }

    pub fn is_null(&self) -> bool {
        is_null(self.value.as_deref(), &self.type_info)
    }
}

fn is_null(value: Option<&[u8]>, ty: &MySqlTypeInfo) -> bool {
    if let Some(value) = value {
        // zero dates and date times should be treated the same as NULL
        if matches!(
            ty.r#type,
            ColumnType::Date | ColumnType::Timestamp | ColumnType::Datetime
        ) && value.get(0) == Some(&0)
        {
            return true;
        }
    }

    value.is_none()
}
