use crate::value::map::ValueMap;
use crate::{Error, Value};
use serde::ser::{
    self, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple, SerializeTupleStruct,
};
use serde::Serialize;


impl Serialize for Value {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *self {
            Value::Null => s.serialize_unit(),
            Value::Bool(v) => s.serialize_bool(v),
            Value::I32(v) => s.serialize_i32(v),
            Value::I64(v) => s.serialize_i64(v),
            Value::U32(v) => s.serialize_u32(v),
            Value::U64(v) => s.serialize_u64(v),
            Value::F32(v) => s.serialize_f32(v),
            Value::F64(v) => s.serialize_f64(v),
            Value::String(ref v) => s.serialize_str(v),
            Value::Binary(ref v) => s.serialize_bytes(v),
            Value::Array(ref array) => {
                let mut state = s.serialize_seq(Some(array.len()))?;
                for item in array {
                    state.serialize_element(item)?;
                }
                state.end()
            }
            Value::Map(ref map) => {
                let mut state = s.serialize_map(Some(map.len()))?;
                for (key, val) in map {
                    state.serialize_entry(key, val)?;
                }
                state.end()
            }
            Value::Ext(ref ty, ref value) => s.serialize_newtype_struct(ty, value),
        }
    }
}


/// Convert a `T` into `rbs::Value` which is an enum that can represent any valid MessagePack data.
///
/// This conversion can fail if `T`'s implementation of `Serialize` decides to fail.
///
/// ```rust
/// # use rbs::Value;
///
/// let val = rbs::to_value("John Smith").unwrap();
///
/// assert_eq!(Value::String("John Smith".into()), val);
/// ```
#[inline]
pub fn to_value<T: Serialize>(mut value: T) -> Result<Value, Error> {
    let type_name = std::any::type_name::<T>();
    if type_name == std::any::type_name::<Value>() {
        let addr = std::ptr::addr_of_mut!(value);
        let v = unsafe { &mut *(addr as *mut _ as *mut Value) };
        return Ok(std::mem::take(v));
    }
    if type_name == std::any::type_name::<&Value>() {
        let addr = std::ptr::addr_of!(value);
        return Ok(unsafe { *(addr as *const _ as *const &Value) }.clone());
    }
    if type_name == std::any::type_name::<&&Value>() {
        let addr = std::ptr::addr_of!(value);
        return Ok(unsafe { **(addr as *const _ as *const &&Value) }.clone());
    }
    value.serialize(Value::Null)
}

#[inline]
pub fn to_value_def<T: Serialize>(value: T) -> Value {
    to_value(value).unwrap_or_default()
}

impl ser::Serializer for Value {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = SerializeTupleStructVec;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = DefaultSerializeMap;
    type SerializeStruct = DefaultSerializeMap;
    type SerializeStructVariant = DefaultSerializeMap;

    #[inline]
    fn serialize_bool(self, val: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bool(val))
    }

    #[inline]
    fn serialize_i8(self, val: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I32(val as i32))
    }

    #[inline]
    fn serialize_i16(self, val: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I32(val as i32))
    }

    #[inline]
    fn serialize_i32(self, val: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I32(val))
    }

    #[inline]
    fn serialize_i64(self, val: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I64(val))
    }

    #[inline]
    fn serialize_u8(self, val: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U32(val as u32))
    }

    #[inline]
    fn serialize_u16(self, val: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U32(val as u32))
    }

    #[inline]
    fn serialize_u32(self, val: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U32(val))
    }

    #[inline]
    fn serialize_u64(self, val: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U64(val))
    }

    #[inline]
    fn serialize_f32(self, val: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::F32(val))
    }

    #[inline]
    fn serialize_f64(self, val: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::F64(val))
    }

    #[inline]
    fn serialize_char(self, val: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = String::new();
        buf.push(val);
        self.serialize_str(&buf)
    }

    #[inline]
    fn serialize_str(self, val: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(val.into()))
    }

    #[inline]
    fn serialize_bytes(self, val: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Binary(val.into()))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(Vec::new()))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(variant.to_string()))
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        return Ok(Value::Ext(name, Box::new(value.serialize(self)?)));
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut values = ValueMap::with_capacity(1);
        values.insert(Value::String(variant.to_string()), value.serialize(self)?);
        Ok(Value::Map(values))
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let se = SerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        };
        Ok(se)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStructVec {
            name: name,
            vec: vec![],
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        //TODO impl serialize_tuple_variant
        return Err(Error::E(
            "rbs Serialize unimplemented serialize_tuple_variant".to_string(),
        ));
        // let se = SerializeTupleVariant {
        //     idx,
        //     vec: Vec::with_capacity(len),
        // };
        // Ok(se)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        let se = DefaultSerializeMap {
            map: ValueMap::with_capacity(len.unwrap_or(0)),
            next_key: None,
        };
        Ok(se)
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        let se = DefaultSerializeMap {
            map: ValueMap::with_capacity(len),
            next_key: None,
        };
        Ok(se)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        //TODO impl serialize_struct_variant
        return Err(Error::E(
            "rbs Serialize unimplemented serialize_struct_variant".to_string(),
        ));
        // let se = DefaultSerializeMap {
        //     map: Vec::with_capacity(len),
        //     next_key: None,
        // };
        // Ok(se)
    }
}

#[doc(hidden)]
pub struct SerializeVec {
    vec: Vec<Value>,
}

#[doc(hidden)]
pub struct SerializeTupleStructVec {
    pub name: &'static str,
    pub vec: Vec<Value>,
}

impl SerializeTupleStruct for SerializeTupleStructVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.vec.push(to_value(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Ext(self.name, Box::new(Value::Array(self.vec))))
    }
}

/// Default implementation for tuple variant serialization. It packs given enums as a tuple of an
/// index with a tuple of arguments.
#[doc(hidden)]
pub struct SerializeTupleVariant {
    idx: u32,
    vec: Vec<Value>,
}

#[doc(hidden)]
pub struct DefaultSerializeMap {
    map: ValueMap,
    next_key: Option<Value>,
}


impl SerializeSeq for SerializeVec {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.vec.push(to_value(&value)?);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Array(self.vec))
    }
}

impl SerializeTuple for SerializeVec {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        ser::SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for SerializeVec {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.vec.push(to_value(&value)?);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Array(vec![
            Value::from(self.idx),
            Value::Array(self.vec),
        ]))
    }
}

impl ser::SerializeMap for DefaultSerializeMap {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.next_key = Some(to_value(key)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        // Panic because this indicates a bug in the program rather than an
        // expected failure.
        let key = self
            .next_key
            .take()
            .expect("`serialize_value` called before `serialize_key`");
        self.map.insert(key, to_value(&value)?);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Map(self.map))
    }
}

impl ser::SerializeStruct for DefaultSerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map
            .insert(Value::String(key.to_string()), to_value(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.map))
    }
}

impl ser::SerializeStructVariant for DefaultSerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map
            .insert(Value::String(key.to_string()), to_value(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.map))
    }
}

impl SerializeStruct for SerializeVec {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        ser::SerializeSeq::end(self)
    }
}
