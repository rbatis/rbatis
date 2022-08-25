use crate::Value;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::vec::IntoIter;

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ValueMap(pub Vec<(Value, Value)>);

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
    pub fn remove(&mut self, k: &Value) -> Option<Value> {
        let mut idx = 0;
        for (mkey, v) in &self.0 {
            if k.eq(mkey) {
                let (_, v) = self.0.remove(idx);
                return Some(v);
            }
            idx += 1
        }
        return None;
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
    {$($k:expr=>$v:expr$(,)*)+} => {
        {
        let mut m  = $crate::value::map::ValueMap::new();
        $(m.insert($crate::to_value!($k),$crate::to_value!($v));)+
        m
        }
    };
}