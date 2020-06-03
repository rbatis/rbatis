use std::str::from_utf8;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::decode::Decode;
use crate::encode::Encode;
use crate::mysql::protocol::TypeId;
use crate::mysql::type_info::MySqlTypeInfo;
use crate::mysql::{MySql, MySqlData, MySqlValue};
use crate::types::Type;
use crate::Error;

impl Type<MySql> for i8 {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::new(TypeId::TINY_INT)
    }
}

impl Encode<MySql> for i8 {
    fn encode(&self, buf: &mut Vec<u8>) {
        let _ = buf.write_i8(*self);
    }
}

impl<'de> Decode<'de, MySql> for i8 {
    fn decode(value: MySqlValue<'de>) -> crate::Result<Self> {
        match value.try_get()? {
            MySqlData::Binary(mut buf) => buf.read_i8().map_err(Into::into),

            MySqlData::Text(s) => from_utf8(s)
                .map_err(Error::decode)?
                .parse()
                .map_err(Error::decode),
        }
    }
}

impl Type<MySql> for i16 {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::new(TypeId::SMALL_INT)
    }
}

impl Encode<MySql> for i16 {
    fn encode(&self, buf: &mut Vec<u8>) {
        let _ = buf.write_i16::<LittleEndian>(*self);
    }
}

impl<'de> Decode<'de, MySql> for i16 {
    fn decode(value: MySqlValue<'de>) -> crate::Result<Self> {
        match value.try_get()? {
            MySqlData::Binary(mut buf) => buf.read_i16::<LittleEndian>().map_err(Into::into),

            MySqlData::Text(s) => from_utf8(s)
                .map_err(Error::decode)?
                .parse()
                .map_err(Error::decode),
        }
    }
}

impl Type<MySql> for i32 {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::new(TypeId::INT)
    }
}

impl Encode<MySql> for i32 {
    fn encode(&self, buf: &mut Vec<u8>) {
        let _ = buf.write_i32::<LittleEndian>(*self);
    }
}

impl<'de> Decode<'de, MySql> for i32 {
    fn decode(value: MySqlValue<'de>) -> crate::Result<Self> {
        match value.try_get()? {
            MySqlData::Binary(mut buf) => buf.read_i32::<LittleEndian>().map_err(Into::into),

            MySqlData::Text(s) => from_utf8(s)
                .map_err(Error::decode)?
                .parse()
                .map_err(Error::decode),
        }
    }
}

impl Type<MySql> for i64 {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::new(TypeId::BIG_INT)
    }
}

impl Encode<MySql> for i64 {
    fn encode(&self, buf: &mut Vec<u8>) {
        let _ = buf.write_i64::<LittleEndian>(*self);
    }
}

impl<'de> Decode<'de, MySql> for i64 {
    fn decode(value: MySqlValue<'de>) -> crate::Result<Self> {
        match value.try_get()? {
            MySqlData::Binary(mut buf) => buf.read_i64::<LittleEndian>().map_err(Into::into),

            MySqlData::Text(s) => from_utf8(s)
                .map_err(Error::decode)?
                .parse()
                .map_err(Error::decode),
        }
    }
}
