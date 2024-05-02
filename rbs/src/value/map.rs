use crate::Value;
use indexmap::IndexMap;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserializer, Serializer};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(PartialEq)]
pub struct ValueMap(pub IndexMap<Value, Value>);

impl serde::Serialize for ValueMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut m = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in &self.0 {
            m.serialize_key(&k)?;
            m.serialize_value(&v)?;
        }
        m.end()
    }
}

struct IndexMapVisitor;

impl<'de> Visitor<'de> for IndexMapVisitor {
    type Value = ValueMap;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = ValueMap::with_capacity(map.size_hint().unwrap_or(0));
        while let Some((key, value)) = map.next_entry()? {
            values.insert(key, value);
        }
        Ok(values)
    }
}

impl<'de> serde::Deserialize<'de> for ValueMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let m = deserializer.deserialize_map(IndexMapVisitor {})?;
        Ok(m)
    }
}

impl Clone for ValueMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Debug for ValueMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl Display for ValueMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        let mut idx = 0;
        for (k, v) in &self.0 {
            Display::fmt(k, f)?;
            f.write_str(":")?;
            Display::fmt(v, f)?;
            if idx + 1 != self.len() {
                Display::fmt(",", f)?;
            }
            idx += 1;
        }
        f.write_str("}")
    }
}

impl ValueMap {
    pub fn new() -> Self {
        ValueMap(IndexMap::new())
    }
    pub fn with_capacity(n: usize) -> Self {
        ValueMap(IndexMap::with_capacity(n))
    }
    pub fn insert(&mut self, k: Value, v: Value) -> Option<Value> {
        self.0.insert(k, v)
    }
    pub fn remove(&mut self, k: &Value) -> Value {
        self.0.swap_remove(k).unwrap_or_default()
    }

    pub fn rm(&mut self, k: &Value) -> Value {
        self.remove(k)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Index<&str> for ValueMap {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        self.0.index(&Value::String(index.to_string()))
    }
}

impl Index<i64> for ValueMap {
    type Output = Value;

    fn index(&self, index: i64) -> &Self::Output {
        self.0.index(&Value::I64(index))
    }
}

impl IndexMut<&str> for ValueMap {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.0.index_mut(&Value::String(index.to_string()))
    }
}

impl IndexMut<i64> for ValueMap {
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        self.0.index_mut(&Value::I64(index))
    }
}

impl<'a> IntoIterator for &'a ValueMap {
    type Item = (&'a Value, &'a Value);
    type IntoIter = indexmap::map::Iter<'a, Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut ValueMap {
    type Item = (&'a Value, &'a mut Value);
    type IntoIter = indexmap::map::IterMut<'a, Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut().into_iter()
    }
}

impl IntoIterator for ValueMap {
    type Item = (Value, Value);
    type IntoIter = indexmap::map::IntoIter<Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[macro_export]
macro_rules! value_map {
      {$($k:tt:$v:expr $(,)+ )*} => {
        {
        let mut m  = $crate::value::map::ValueMap::with_capacity(50);
        $(m.insert($crate::to_value!($k),$crate::to_value!($v));)*
        m
        }
    };
}

#[cfg(test)]
mod test {
    use crate::value::map::ValueMap;

    #[test]
    fn test_fmt() {
        let mut m = ValueMap::new();
        m.insert("1".into(), 1.into());
        m.insert("2".into(), 2.into());
        assert_eq!(m.to_string(), r#"{"1":1,"2":2}"#);
    }
}
