use std::fmt::{self, Display, Formatter};
use std::vec::IntoIter;

use serde::de::{DeserializeSeed, IntoDeserializer, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

use crate::value::map::ValueMap;
use crate::value::Value;

use super::{Error, ValueExt};

/// from_value
#[inline]
pub fn from_value<T>(val: Value) -> Result<T, Error>
where
    T: for<'de> Deserialize<'de>,
{
    deserialize_from(val)
}

/// deserialize_from
#[inline]
pub fn deserialize_from<'de, T, D>(val: D) -> Result<T, Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de, Error = Error>,
{
    Deserialize::deserialize(val)
}

impl serde::de::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Self {
        Error::Syntax(format!("{}", msg))
    }
}

impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            #[cold]
            fn expecting(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
                "any valid MessagePack value".fmt(fmt)
            }

            #[inline]
            fn visit_some<D>(self, de: D) -> Result<Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                Deserialize::deserialize(de)
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::Bool(value))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::U32(v))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                Ok(Value::U64(value))
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::I32(v))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::I64(value))
            }

            #[inline]
            fn visit_f32<E>(self, value: f32) -> Result<Value, E> {
                Ok(Value::F32(value))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Value::F64(value))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::String(value))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = {
                    match visitor.size_hint() {
                        None => {
                            vec![]
                        }
                        Some(l) => Vec::with_capacity(l),
                    }
                };
                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }
                Ok(Value::Array(vec))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Binary(v.to_owned()))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Binary(v))
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pairs = {
                    match visitor.size_hint() {
                        None => {
                            vec![]
                        }
                        Some(l) => Vec::with_capacity(l),
                    }
                };
                while let Some(key) = visitor.next_key()? {
                    let val = visitor.next_value()?;
                    pairs.push((key, val));
                }

                Ok(Value::Map(ValueMap(pairs)))
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_newtype_struct("", self)
            }
        }

        de.deserialize_any(ValueVisitor)
    }
}

impl<'de> Deserializer<'de> for Value {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_none(),
            Value::Bool(v) => visitor.visit_bool(v),
            Value::I32(v) => visitor.visit_i32(v),
            Value::I64(v) => visitor.visit_i64(v),
            Value::U32(v) => visitor.visit_u32(v),
            Value::U64(v) => visitor.visit_u64(v),
            Value::F32(v) => visitor.visit_f32(v),
            Value::F64(v) => visitor.visit_f64(v),
            Value::String(v) => visitor.visit_string(v),
            Value::Binary(v) => visitor.visit_byte_buf(v),
            Value::Array(v) => {
                let len = v.len();
                let mut de = SeqDeserializer::new(v.into_iter());
                let seq = visitor.visit_seq(&mut de)?;
                if de.iter.len() == 0 {
                    Ok(seq)
                } else {
                    Err(serde::de::Error::invalid_length(len, &"fewer elements in array"))
                }
            }
            Value::Map(v) => {
                let len = v.len();
                let mut de = MapDeserializer::new(v.0.into_iter());
                let map = visitor.visit_map(&mut de)?;
                if de.iter.len() == 0 {
                    Ok(map)
                } else {
                    Err(serde::de::Error::invalid_length(len, &"fewer elements in map"))
                }
            }
            Value::Ext(_tag, data) => Deserializer::deserialize_any(*data, visitor),
        }
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ValueBase::deserialize_option(self, visitor)
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ValueBase::deserialize_enum(self, visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ValueBase::deserialize_unit_struct(self, visitor)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq
        bytes byte_buf map tuple_struct struct
        identifier tuple ignored_any
    }
}

struct SeqDeserializer<I> {
    iter: I,
}

impl<I> SeqDeserializer<I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<'de, I, U> SeqAccess<'de> for SeqDeserializer<I>
where
    I: Iterator<Item = U>,
    U: Deserializer<'de, Error = Error>,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(val) => seed.deserialize(val).map(Some),
            None => Ok(None),
        }
    }
}

impl<'de, I, U> Deserializer<'de> for SeqDeserializer<I>
where
    I: ExactSizeIterator<Item = U>,
    U: Deserializer<'de, Error = Error>,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self.iter.len();
        if len == 0 {
            visitor.visit_unit()
        } else {
            let ret = visitor.visit_seq(&mut self)?;
            let rem = self.iter.len();
            if rem == 0 {
                Ok(ret)
            } else {
                Err(serde::de::Error::invalid_length(len, &"fewer elements in array"))
            }
        }
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct identifier tuple enum ignored_any
    }
}

struct MapDeserializer<I, U> {
    val: Option<U>,
    iter: I,
}

impl<I, U> MapDeserializer<I, U> {
    fn new(iter: I) -> Self {
        Self { val: None, iter }
    }
}

impl<'de, I, U> serde::de::MapAccess<'de> for MapDeserializer<I, U>
where
    I: Iterator<Item = (U, U)>,
    U: ValueBase<'de>,
{
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, val)) => {
                self.val = Some(val);
                seed.deserialize(key).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.val.take() {
            Some(val) => seed.deserialize(val),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }
}

impl<'de, I, U> Deserializer<'de> for MapDeserializer<I, U>
where
    I: Iterator<Item = (U, U)>,
    U: ValueBase<'de>,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct identifier tuple enum ignored_any
    }
}

struct EnumDeserializer {
    variant: String,
    value: Option<Value>,
}

impl<'de> serde::de::EnumAccess<'de> for EnumDeserializer {
    type Error = Error;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, VariantDeserializer), Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = VariantDeserializer { value: self.value };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

struct VariantDeserializer {
    value: Option<Value>,
}

impl<'de> serde::de::VariantAccess<'de> for VariantDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        match self.value {
            Some(_v) => Ok(()),
            None => Err(serde::de::Error::invalid_value(
                Unexpected::Other(&format!("none")),
                &"not support",
            )),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.value {
            Some(v) => {
                let m = v.into_map();
                if m.is_none() {
                    return Err(serde::de::Error::custom(format!(
                        "Deserialize newtype_variant must be {}, and len = 1",
                        "{\"key\",\"v\"}"
                    )));
                }
                let m = m.unwrap();
                if m.len() != 1 {
                    return Err(serde::de::Error::custom(format!(
                        "Deserialize newtype_variant must be {}, and len = 1",
                        "{\"key\",\"v\"}"
                    )));
                }
                let mut v = m.0;
                seed.deserialize(v.pop().unwrap().1)
            }
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        //todo impl tuple_variant
        return Err(Error::Syntax(
            "rbs Deserialize unimplemented tuple_variant".to_string(),
        ));
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        //todo impl struct_variant
        return Err(Error::Syntax(
            "rbs Deserialize unimplemented struct_variant".to_string(),
        ));
    }
}

trait ValueBase<'de>: Deserializer<'de, Error = Error> + ValueExt
where
    Self: 'static,
{
    type Item: ValueBase<'de>;
    type Iter: ExactSizeIterator<Item = Self::Item>;
    type MapIter: Iterator<Item = (Self::Item, Self::Item)>;
    type MapDeserializer: Deserializer<'de>;

    fn into_value(self) -> Value;
    fn is_null(&self) -> bool;

    fn into_iter(self) -> Result<Self::Iter, Self::Item>;
    fn into_map_iter(self) -> Result<Self::MapIter, Self::Item>;

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_null() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    #[inline]
    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v = self.into_value();
        let v = match v {
            Value::String(v) => visitor.visit_enum(EnumDeserializer {
                variant: v.clone(),
                value: Some(Value::String(v)),
            }),
            Value::Map(m) => {
                if m.is_empty() || m.len() != 1 {
                    return Err(serde::de::Error::invalid_type(
                        Unexpected::Other(&format!("{:?}", m)),
                        &"must be object map {\"Key\":\"Value\"}",
                    ));
                }
                visitor.visit_enum(EnumDeserializer {
                    variant: m.0[0].0.clone().into_string().unwrap(),
                    value: Some(Value::Map(m)),
                })
            }
            _ => {
                return Err(serde::de::Error::invalid_type(
                    Unexpected::Other(&format!("{:?}", v)),
                    &"string or map",
                ));
            }
        };
        v
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_unit_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.into_iter() {
            Ok(iter) => {
                if iter.len() == 0 {
                    visitor.visit_unit()
                } else {
                    Err(serde::de::Error::invalid_type(Unexpected::Seq, &"empty array"))
                }
            }
            Err(other) => Err(serde::de::Error::invalid_type(other.unexpected(), &"empty array")),
        }
    }
}

impl<'de> ValueBase<'de> for Value {
    type Item = Value;
    type Iter = IntoIter<Value>;
    type MapIter = IntoIter<(Value, Value)>;
    type MapDeserializer = MapDeserializer<Self::MapIter, Self::Item>;

    fn into_value(self) -> Value {
        self
    }

    #[inline]
    fn is_null(&self) -> bool {
        if let Value::Null = *self {
            true
        } else {
            false
        }
    }

    #[inline]
    fn into_iter(self) -> Result<Self::Iter, Self::Item> {
        match self {
            Value::Array(v) => Ok(v.into_iter()),
            other => Err(other),
        }
    }

    #[inline]
    fn into_map_iter(self) -> Result<Self::MapIter, Self::Item> {
        match self {
            Value::Map(v) => Ok(v.0.into_iter()),
            other => Err(other),
        }
    }
}
