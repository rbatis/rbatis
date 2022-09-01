//! Contains Value and ValueRef structs and its conversion traits.
//!
//! # Examples
//!
//! ```
//! ```
use crate::value::map::ValueMap;
use std::borrow::Cow;
use std::fmt::{self, Debug, Display};
use std::iter::FromIterator;
use std::ops::Deref;

pub mod ext;
pub mod map;
pub mod ops;

/// Represents any valid MessagePack value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Nil represents nil.
    Null,
    /// Bool represents true or false.
    Bool(bool),
    /// Int32
    I32(i32),
    /// Int64
    I64(i64),
    /// Uint32
    U32(u32),
    /// Uint64
    U64(u64),
    /// A 32-bit floating point number.
    F32(f32),
    /// A 64-bit floating point number.
    F64(f64),
    /// String extending Raw type represents a UTF-8 string.
    String(String),
    /// Binary extending Raw type represents a byte array.
    Binary(Vec<u8>),
    /// Array represents a sequence of objects.
    Array(Vec<Self>),
    /// Map represents key-value pairs of objects.
    Map(ValueMap),
    /// Extended implements Extension interface
    Ext(&'static str, Box<Self>),
}

impl Value {
    /// Returns true if the `Value` is a Null. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert!(Value::Null.is_null());
    /// ```
    #[inline]
    pub fn is_null(&self) -> bool {
        if let Value::Null = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if the `Value` is a Bool. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert!(Value::Bool(true).is_bool());
    ///
    /// assert!(!Value::Null.is_bool());
    /// ```
    #[inline]
    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    /// Returns true if the `Value` is convertible to an i64. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert!(Value::from(42).is_i64());
    ///
    /// assert!(!Value::from(42.0).is_i64());
    /// ```
    #[inline]
    pub fn is_i64(&self) -> bool {
        if let Value::I64(ref v) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if the `Value` is convertible to an u64. Returns false otherwise.
    ///
    #[inline]
    pub fn is_u64(&self) -> bool {
        if let Value::U64(ref v) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if (and only if) the `Value` is a f32. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert!(Value::F32(42.0).is_f32());
    ///
    /// assert!(!Value::from(42).is_f32());
    /// assert!(!Value::F64(42.0).is_f32());
    /// ```
    #[inline]
    pub fn is_f32(&self) -> bool {
        if let Value::F32(..) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if (and only if) the `Value` is a f64. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert!(Value::F64(42.0).is_f64());
    ///
    /// assert!(!Value::from(42).is_f64());
    /// assert!(!Value::F32(42.0).is_f64());
    /// ```
    #[inline]
    pub fn is_f64(&self) -> bool {
        if let Value::F64(..) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if the `Value` is a Number. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert!(Value::from(42).is_number());
    /// assert!(Value::F32(42.0).is_number());
    /// assert!(Value::F64(42.0).is_number());
    ///
    /// assert!(!Value::Null.is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        match *self {
            Value::I64(..) | Value::U64(..) | Value::F32(..) | Value::F64(..) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is a String. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert!(Value::String("value".into()).is_str());
    ///
    /// assert!(!Value::Null.is_str());
    /// ```
    #[inline]
    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    /// Returns true if the `Value` is a Binary. Returns false otherwise.
    #[inline]
    pub fn is_bin(&self) -> bool {
        self.as_slice().is_some()
    }

    /// Returns true if the `Value` is an Array. Returns false otherwise.
    #[inline]
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// Returns true if the `Value` is a Map. Returns false otherwise.
    #[inline]
    pub fn is_map(&self) -> bool {
        self.as_map().is_some()
    }

    /// Returns true if the `Value` is an Ext. Returns false otherwise.
    #[inline]
    pub fn is_ext(&self) -> bool {
        self.as_ext().is_some()
    }

    /// If the `Value` is a Bool, returns the associated bool.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert_eq!(Some(true), Value::Bool(true).as_bool());
    ///
    /// assert_eq!(None, Value::Null.as_bool());
    /// ```
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            Value::Ext(_, e) => e.as_bool(),
            _ => None,
        }
    }

    /// If the `Value` is an integer, return or cast it to a i64.
    /// Returns None otherwise.
    ///
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::F32(ref n) => Some(n.to_owned() as i64),
            Value::F64(ref n) => Some(n.to_owned() as i64),
            Value::U64(ref n) => Some(n.to_owned() as i64),
            Value::U32(ref n) => Some(n.to_owned() as i64),
            Value::I64(ref n) => Some(n.to_owned()),
            Value::I32(ref n) => Some(n.to_owned() as i64),
            Value::Ext(_, ref e) => e.as_i64(),
            _ => None,
        }
    }

    /// If the `Value` is an integer, return or cast it to a u64.
    /// Returns None otherwise.
    ///
    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::F32(ref n) => Some(n.to_owned() as u64),
            Value::F64(ref n) => Some(n.to_owned() as u64),
            Value::I64(ref n) => Some(n.to_owned() as u64),
            Value::I32(ref n) => Some(n.to_owned() as u64),
            Value::U64(ref n) => Some(n.to_owned()),
            Value::U32(ref n) => Some(n.to_owned() as u64),
            Value::Ext(_, ref e) => e.as_u64(),
            _ => None,
        }
    }

    /// If the `Value` is a number, return or cast it to a f64.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert_eq!(Some(42.0), Value::from(42).as_f64());
    /// assert_eq!(Some(42.0), Value::F32(42.0f32).as_f64());
    /// assert_eq!(Some(42.0), Value::F64(42.0f64).as_f64());
    ///
    /// assert_eq!(Some(2147483647.0), Value::from(i32::max_value() as i64).as_f64());
    ///
    /// assert_eq!(None, Value::Null.as_f64());
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::I32(n) => Some(n as f64),
            Value::U32(n) => Some(n as f64),
            Value::I64(n) => Some(n as f64),
            Value::U64(n) => Some(n as f64),
            Value::F32(n) => Some(From::from(n)),
            Value::F64(n) => Some(n),
            Value::Ext(_, ref e) => e.as_f64(),
            _ => None,
        }
    }

    /// If the `Value` is a String, returns the associated str.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert_eq!(Some("le message"), Value::String("le message".into()).as_str());
    ///
    /// assert_eq!(None, Value::Bool(true).as_str());
    /// ```
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            Value::Ext(_, s) => s.as_str(),
            _ => None,
        }
    }

    #[inline]
    pub fn into_string(self) -> Option<String> {
        match self {
            Value::String(v) => Some(v),
            Value::Ext(_, ext) => ext.into_string(),
            _ => None,
        }
    }

    /// self to Binary
    #[inline]
    pub fn into_bytes(self) -> Option<Vec<u8>> {
        match self {
            Value::Binary(v) => Some(v),
            Value::Ext(_, ext) => ext.into_bytes(),
            _ => None,
        }
    }

    /// If the `Value` is a Binary or a String, returns the associated slice.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// assert_eq!(Some(&[1, 2, 3, 4, 5][..]), Value::Binary(vec![1, 2, 3, 4, 5]).as_slice());
    ///
    /// assert_eq!(None, Value::Bool(true).as_slice());
    /// ```
    pub fn as_slice(&self) -> Option<&[u8]> {
        if let Value::Binary(ref val) = *self {
            Some(val)
        } else if let Value::String(ref val) = *self {
            Some(val.as_bytes())
        } else if let Value::Ext(_, ref val) = *self {
            val.as_slice()
        } else {
            None
        }
    }

    /// If the `Value` is an Array, returns the associated vector.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbs::Value;
    ///
    /// let val = Value::Array(vec![Value::Null, Value::Bool(true)]);
    ///
    /// assert_eq!(Some(&vec![Value::Null, Value::Bool(true)]), val.as_array());
    ///
    /// assert_eq!(None, Value::Null.as_array());
    /// ```
    #[inline]
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(ref array) = *self {
            Some(&*array)
        } else if let Value::Ext(_, ref ext) = *self {
            ext.as_array()
        } else {
            None
        }
    }

    /// If the `Value` is a Map, returns the associated vector of key-value tuples.
    /// Returns None otherwise.
    ///
    #[inline]
    pub fn as_map(&self) -> Option<&ValueMap> {
        if let Value::Map(ref map) = *self {
            Some(map)
        } else if let Value::Ext(_, ref map) = *self {
            map.as_map()
        } else {
            None
        }
    }

    /// If the `Value` is an Ext, returns the associated tuple with a ty and slice.
    /// Returns None otherwise.
    ///
    #[inline]
    pub fn as_ext(&self) -> Option<(&str, &Box<Value>)> {
        if let Value::Ext(ref ty, ref buf) = *self {
            Some((ty, buf))
        } else {
            None
        }
    }
}

impl From<bool> for Value {
    #[inline]
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<u8> for Value {
    #[inline]
    fn from(v: u8) -> Self {
        Value::U64(From::from(v))
    }
}

impl From<u16> for Value {
    #[inline]
    fn from(v: u16) -> Self {
        Value::U64(From::from(v))
    }
}

impl From<u32> for Value {
    #[inline]
    fn from(v: u32) -> Self {
        Value::U64(From::from(v))
    }
}

impl From<u64> for Value {
    #[inline]
    fn from(v: u64) -> Self {
        Value::U64(From::from(v))
    }
}

impl From<usize> for Value {
    #[inline]
    fn from(v: usize) -> Self {
        Value::U64(From::from(v as u64))
    }
}

impl From<i8> for Value {
    #[inline]
    fn from(v: i8) -> Self {
        Value::I64(From::from(v))
    }
}

impl From<i16> for Value {
    #[inline]
    fn from(v: i16) -> Self {
        Value::I64(From::from(v))
    }
}

impl From<i32> for Value {
    #[inline]
    fn from(v: i32) -> Self {
        Value::I64(From::from(v))
    }
}

impl From<i64> for Value {
    #[inline]
    fn from(v: i64) -> Self {
        Value::I64(From::from(v))
    }
}

impl From<isize> for Value {
    #[inline]
    fn from(v: isize) -> Self {
        Value::I64(From::from(v as i64))
    }
}

impl From<f32> for Value {
    #[inline]
    fn from(v: f32) -> Self {
        Value::F32(v)
    }
}

impl From<f64> for Value {
    #[inline]
    fn from(v: f64) -> Self {
        Value::F64(v)
    }
}

impl From<String> for Value {
    #[inline]
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl<'a> From<&'a str> for Value {
    #[inline]
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    #[inline]
    fn from(v: Cow<'a, str>) -> Self {
        Value::String(v.to_string())
    }
}

impl From<Vec<u8>> for Value {
    #[inline]
    fn from(v: Vec<u8>) -> Self {
        Value::Binary(v)
    }
}

impl<'a> From<&'a [u8]> for Value {
    #[inline]
    fn from(v: &[u8]) -> Self {
        Value::Binary(v.into())
    }
}

impl<'a> From<Cow<'a, [u8]>> for Value {
    #[inline]
    fn from(v: Cow<'a, [u8]>) -> Self {
        Value::Binary(v.into_owned())
    }
}

impl From<Vec<Value>> for Value {
    #[inline]
    fn from(v: Vec<Value>) -> Self {
        Value::Array(v)
    }
}

impl From<Vec<(Value, Value)>> for Value {
    #[inline]
    fn from(v: Vec<(Value, Value)>) -> Self {
        Value::Map(ValueMap(v))
    }
}

///from tuple for ext
impl From<(&'static str, Value)> for Value {
    fn from(arg: (&'static str, Value)) -> Self {
        Value::Ext(arg.0, Box::new(arg.1))
    }
}

/// Note that an `Iterator<Item = u8>` will be collected into an
/// [`Array`](crate::Value::Array), rather than a
/// [`Binary`](crate::Value::Binary)
impl<V> FromIterator<V> for Value
where
    V: Into<Value>,
{
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        let v: Vec<Value> = iter.into_iter().map(|v| v.into()).collect();
        Value::Array(v)
    }
}

impl Display for Value {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            Value::Null => f.write_str("null"),
            Value::Bool(val) => Display::fmt(&val, f),
            Value::I32(ref val) => Display::fmt(&val, f),
            Value::I64(ref val) => Display::fmt(&val, f),
            Value::U32(ref val) => Display::fmt(&val, f),
            Value::U64(ref val) => Display::fmt(&val, f),
            Value::F32(val) => Display::fmt(&val, f),
            Value::F64(val) => Display::fmt(&val, f),
            Value::String(ref val) => {
                f.write_str("\"")?;
                Display::fmt(&self.as_str().unwrap_or_default(), f)?;
                f.write_str("\"")
            }
            Value::Binary(ref val) => Debug::fmt(val, f),
            Value::Array(ref vec) => {
                f.write_str("[")?;
                let mut i = 0;
                for x in vec {
                    Display::fmt(&x, f)?;
                    i += 1;
                    if i != vec.len() {
                        f.write_str(",")?;
                    }
                }
                f.write_str("]")?;
                Ok(())
            }
            Value::Map(ref vec) => Display::fmt(vec, f),
            Value::Ext(ref ty, ref data) => {
                write!(f, "{}({})", ty, data.deref())
            }
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl IntoIterator for Value {
    type Item = (Value, Value);
    type IntoIter = std::vec::IntoIter<(Value, Value)>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Value::Map(v) => v.into_iter(),
            Value::Array(arr) => {
                let mut v = ValueMap::with_capacity(arr.len());
                let mut idx = 0;
                for x in arr {
                    v.push((Value::U32(idx), x));
                    idx += 1;
                }
                v.into_iter()
            }
            Value::Ext(_, e) => e.into_iter(),
            _ => {
                let v = ValueMap::with_capacity(0);
                v.into_iter()
            }
        }
    }
}

impl<'a> IntoIterator for &'a Value {
    type Item = (Cow<'a, Value>, &'a Value);
    type IntoIter = std::vec::IntoIter<(Cow<'a, Value>, &'a Value)>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Value::Map(m) => {
                let mut arr = Vec::with_capacity(m.len());
                for (k, v) in m {
                    arr.push((Cow::Borrowed(k), v));
                }
                arr.into_iter()
            }
            Value::Array(arr) => {
                let mut v = Vec::with_capacity(arr.len());
                let mut idx = 0;
                for x in arr {
                    let b = Box::new(Value::U32(idx));
                    v.push((Cow::Owned(Value::U32(idx)), x));
                    idx += 1;
                }
                v.into_iter()
            }
            Value::Ext(_, e) => e.deref().into_iter(),
            _ => {
                let v = Vec::with_capacity(0);
                v.into_iter()
            }
        }
    }
}

impl From<Value> for bool {
    fn from(arg: Value) -> Self {
        arg.as_bool().unwrap_or_default()
    }
}

impl From<&Value> for bool {
    fn from(arg: &Value) -> Self {
        arg.as_bool().unwrap_or_default()
    }
}

impl From<Value> for f64 {
    fn from(arg: Value) -> Self {
        arg.as_f64().unwrap_or_default()
    }
}

impl From<&Value> for f64 {
    fn from(arg: &Value) -> Self {
        arg.as_f64().unwrap_or_default()
    }
}

impl From<Value> for i64 {
    fn from(arg: Value) -> Self {
        arg.as_i64().unwrap_or_default()
    }
}

impl From<&Value> for i64 {
    fn from(arg: &Value) -> Self {
        arg.as_i64().unwrap_or_default()
    }
}

impl From<Value> for u64 {
    fn from(arg: Value) -> Self {
        arg.as_u64().unwrap_or_default()
    }
}

impl From<&Value> for u64 {
    fn from(arg: &Value) -> Self {
        arg.as_u64().unwrap_or_default()
    }
}

impl From<Value> for String {
    fn from(arg: Value) -> Self {
        arg.as_str().unwrap_or_default().to_string()
    }
}

impl From<&Value> for String {
    fn from(arg: &Value) -> Self {
        arg.as_str().unwrap_or_default().to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::Value;

    #[test]
    fn test_iter() {
        let v = Value::Array(vec![Value::I32(1), Value::I32(2), Value::I32(3)]);
        for (k, v) in &v {
            if Value::I32(1).eq(v) {
                assert_eq!(&Value::U32(0), k.as_ref());
            }
            if Value::I32(2).eq(v) {
                assert_eq!(&Value::U32(1), k.as_ref());
            }
            if Value::I32(3).eq(v) {
                assert_eq!(&Value::U32(2), k.as_ref());
            }
        }
    }
}
