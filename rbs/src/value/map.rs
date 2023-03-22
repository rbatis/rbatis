use crate::Value;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserializer, Serializer};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::vec::IntoIter;

#[derive(PartialEq)]
pub struct ValueMap(pub Vec<(Value, Value)>);

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
        f.debug_map()
            .entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
            .finish()
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
        ValueMap(vec![])
    }
    pub fn with_capacity(n: usize) -> Self {
        ValueMap(Vec::with_capacity(n))
    }
    pub fn insert(&mut self, k: Value, v: Value) {
        for (mk, mv) in &mut self.0 {
            if k.eq(mk) {
                *mv = v;
                return;
            }
        }
        self.0.push((k, v));
    }
    pub fn remove(&mut self, k: &Value) -> Value {
        let mut idx = 0;
        for (mkey, _v) in &self.0 {
            if k.eq(mkey) {
                let (_, v) = self.0.remove(idx);
                return v;
            }
            idx += 1
        }
        return Value::Null;
    }

    pub fn is(&mut self, k: &str, v: Value) {
        let k = Value::String(k.to_string());
        for (mk, mv) in &mut self.0 {
            if k.eq(mk) {
                *mv = v;
                return;
            }
        }
        self.0.push((k, v));
    }

    pub fn rm(&mut self, k: &str) -> Value {
        let mut idx = 0;
        for (key, _v) in &self.0 {
            if k.eq(key.as_str().unwrap_or_default()) {
                let (_, v) = self.0.remove(idx);
                return v;
            }
            idx += 1
        }
        return Value::Null;
    }
}

impl Deref for ValueMap {
    type Target = Vec<(Value, Value)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<&str> for ValueMap {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        for (k, v) in &self.0 {
            if k.as_str().unwrap_or_default().eq(index) {
                return v;
            }
        }
        return &Value::Null;
    }
}

impl Index<i64> for ValueMap {
    type Output = Value;

    fn index(&self, index: i64) -> &Self::Output {
        for (k, v) in &self.0 {
            if k.as_i64().unwrap_or_default().eq(&index) {
                return v;
            }
        }
        return &Value::Null;
    }
}

impl IndexMut<&str> for ValueMap {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        for (k, v) in &mut self.0 {
            if k.as_str().unwrap_or_default().eq(index) {
                return v;
            }
        }
        panic!("not have index={}", index)
    }
}

impl IndexMut<i64> for ValueMap {
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        for (k, v) in &mut self.0 {
            if k.as_i64().unwrap_or_default().eq(&index) {
                return v;
            }
        }
        panic!("not have index={}", index)
    }
}

impl<'a> IntoIterator for &'a ValueMap {
    type Item = &'a (Value, Value);
    type IntoIter = std::slice::Iter<'a, (Value, Value)>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref().into_iter()
    }
}

impl<'a> IntoIterator for &'a mut ValueMap {
    type Item = &'a mut (Value, Value);
    type IntoIter = std::slice::IterMut<'a, (Value, Value)>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref_mut().into_iter()
    }
}

impl IntoIterator for ValueMap {
    type Item = (Value, Value);
    type IntoIter = IntoIter<(Value, Value)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[macro_export]
macro_rules! value_map {
      {$($k:tt:$v:expr $(,)+ )*} => {
        {
        let mut m  = $crate::value::map::ValueMap::new();
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
