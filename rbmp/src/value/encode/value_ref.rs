use std::fmt::{Display, Formatter};
use std::io::Write;

use rmp::encode::{write_array_len, write_bin, write_bool, write_ext_meta, write_f32, write_f64, write_i32, write_i64, write_map_len, write_nil, write_sint, write_str, write_u32, write_u64, write_uint};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use super::Error;
use crate::value::{IntPriv, Integer, ValueRef};

/// Encodes and attempts to write the given non-owning ValueRef into the Write.
///
/// # Errors
///
/// This function returns Error with an underlying I/O error if unable to properly write entire
/// value. Interruption errors are handled internally by silent operation restarting.
///
/// # Examples
/// ```
/// use rbmp::{ValueRef, write_value_ref};
/// use rbmp::encode::write_value_ref;
///
/// let mut buf = Vec::new();
/// let val = ValueRef::from("le message");
///
/// write_value_ref(&mut buf, &val).unwrap();
/// assert_eq!(vec![0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65], buf);
/// ```
pub fn write_value_ref<W>(wr: &mut W, val: &ValueRef<'_>) -> Result<(), Error>
    where W: Write
{
    match *val {
        ValueRef::Nil => {
            write_nil(wr).map_err(Error::InvalidMarkerWrite)?;
        }
        ValueRef::Bool(val) => {
            write_bool(wr, val).map_err(Error::InvalidMarkerWrite)?;
        }
        ValueRef::I32(val) => {
            write_i32(wr, val)?;
        }
        ValueRef::I64(val) => {
            write_i64(wr, val)?;
        }
        ValueRef::U32(val) => {
            write_u32(wr, val)?;
        }
        ValueRef::U64(val) => {
            write_u64(wr, val)?;
        }
        ValueRef::F32(val) => {
            write_f32(wr, val)?;
        }
        ValueRef::F64(val) => {
            write_f64(wr, val)?;
        }
        ValueRef::String(ref s) => {
            // match s {
            //     Ok(val) => write_str(wr, &val)?,
            //     Err(err) => write_bin(wr, &err.0)?,
            // }
            write_bin(wr, s.as_bytes())?;
        }
        ValueRef::Binary(val) => {
            write_bin(wr, val)?;
        }
        ValueRef::Array(ref vec) => {
            write_array_len(wr, vec.len() as u32)?;
            for v in vec {
                write_value_ref(wr, v)?;
            }
        }
        ValueRef::Map(ref map) => {
            write_map_len(wr, map.len() as u32)?;
            for &(ref key, ref val) in map {
                write_value_ref(wr, key)?;
                write_value_ref(wr, val)?;
            }
        }
        ValueRef::Ext(ty, data) => {
            write_ext_meta(wr, data.len() as u32, ty)?;
            wr.write_all(data).map_err(Error::InvalidDataWrite)?;
        }
    }

    Ok(())
}


use serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum SerError {
    E(String)
}

impl Display for SerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SerError::E(e) => {
                e.fmt(f)
            }
        }
    }
}

impl std::error::Error for SerError {}

impl serde::ser::Error for SerError {
    fn custom<T>(msg: T) -> Self where T: Display {
        Self::E(msg.to_string())
    }
}


#[derive(Clone)]
pub struct SerRef {
    //Controls whether Cow owns data
    owner: bool,
}

pub struct SerializeStructImpl<'a> {
    s: SerRef,
    inner: Vec<(ValueRef<'a>, ValueRef<'a>)>,
}

impl<'a> SerializeStruct for SerializeStructImpl<'a> {
    type Ok = ValueRef<'a>;
    type Error = SerError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.push((ValueRef::String(key), value.serialize(self.s.clone())?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Map(self.inner))
    }
}

pub struct SerializeSeqImpl<'a> {
    s: SerRef,
    inner: Vec<ValueRef<'a>>,
}

impl<'a> SerializeSeq for SerializeSeqImpl<'a> {
    type Ok = ValueRef<'a>;
    type Error = SerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.push(value.serialize(self.s.clone())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Array(self.inner))
    }
}

pub struct SerializeTupleImpl<'a> {
    s: SerRef,
    inner: Vec<ValueRef<'a>>,
}

impl<'a> SerializeTuple for SerializeTupleImpl<'a> {
    type Ok = ValueRef<'a>;
    type Error = SerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.push(value.serialize(self.s.clone())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Array(self.inner))
    }
}

pub struct SerializeTupleStructImpl<'a> {
    s: SerRef,
    inner: Vec<ValueRef<'a>>,
}

impl<'a> SerializeTupleStruct for SerializeTupleStructImpl<'a> {
    type Ok = ValueRef<'a>;
    type Error = SerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.push(value.serialize(self.s.clone())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Array(self.inner))
    }
}

pub struct SerializeTupleVariantImpl<'a> {
    s: SerRef,
    inner: Vec<ValueRef<'a>>,
}

impl<'a> SerializeTupleVariant for SerializeTupleVariantImpl<'a> {
    type Ok = ValueRef<'a>;
    type Error = SerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.push(value.serialize(self.s.clone())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Array(self.inner))
    }
}

pub struct SerializeMapImpl<'a> {
    s: SerRef,
    inner: Vec<(ValueRef<'a>, ValueRef<'a>)>,
}

impl<'a> SerializeMap for SerializeMapImpl<'a> {
    type Ok = ValueRef<'a>;
    type Error = SerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.push((key.serialize(self.s.clone())?, ValueRef::Nil));
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        match self.inner.last_mut() {
            None => {}
            Some((_, v)) => {
                *v = value.serialize(self.s.clone())?;
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Map(self.inner))
    }
}

pub struct SerializeStructVariantImpl<'a> {
    s: SerRef,
    inner: Vec<ValueRef<'a>>,
}

impl<'a> SerializeStructVariant for SerializeStructVariantImpl<'a> {
    type Ok = ValueRef<'a>;
    type Error = SerError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.push(value.serialize(self.s.clone())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Array(self.inner))
    }
}


impl Serializer for SerRef {
    type Ok = ValueRef<'static>;
    type Error = SerError;
    type SerializeSeq = SerializeSeqImpl<'static>;
    type SerializeTuple = SerializeTupleImpl<'static>;
    type SerializeTupleStruct = SerializeTupleStructImpl<'static>;
    type SerializeTupleVariant = SerializeTupleVariantImpl<'static>;
    type SerializeMap = SerializeMapImpl<'static>;
    type SerializeStruct = SerializeStructImpl<'static>;
    type SerializeStructVariant = SerializeStructVariantImpl<'static>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::I32(v as i32))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::I32(v as i32))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::I32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::I64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::U32(v as u32))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::U32(v as u32))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::U32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::U64(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::F32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::F64(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::I64(v as i64))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::String(unsafe { &*(v as *const str) }.into()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Binary(unsafe { &*(v as *const [u8]) }.into()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Nil)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Nil)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::Nil)
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(ValueRef::String(variant.into()))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Self::SerializeSeq {
            s: self.clone(),
            inner: Vec::with_capacity(len.unwrap_or_default()),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Self::SerializeTuple {
            s: self.clone(),
            inner: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Self::SerializeTupleStruct {
            s: self.clone(),
            inner: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(Self::SerializeTupleVariant {
            s: self.clone(),
            inner: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Self::SerializeMap {
            s: self.clone(),
            inner: Vec::with_capacity(len.unwrap_or_default()),
        })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Self::SerializeStruct {
            s: self.clone(),
            inner: Vec::with_capacity(len),
        })
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Self::SerializeStructVariant {
            s: self.clone(),
            inner: Vec::with_capacity(len),
        })
    }
}

/// serialize an value ref
pub fn serialize_ref<T>(a: &T) -> Result<ValueRef, SerError> where T: serde::Serialize {
    a.serialize(SerRef { owner: false })
}

impl<'a> ValueRef<'a> {
    pub fn serialize<T: Serialize>(&mut self, arg: &'a T) -> Result<(), SerError> {
        *self = serialize_ref(arg)?;
        Ok(())
    }
}