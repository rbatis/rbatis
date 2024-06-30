use crate::Value;
use std::ops::{Index, IndexMut};

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Value {
        match self {
            Value::Array(arr) => &arr[index],
            Value::Ext(_, ext) => {
                return ext.index(index);
            }
            _ => &Value::Null,
        }
    }
}

impl IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Value::Array(arr) => &mut arr[index],
            Value::Ext(_, ext) => {
                return ext.index_mut(index);
            }
            _ => {
                panic!("not an array!")
            }
        }
    }
}

impl Index<&str> for Value {
    type Output = Value;
    fn index(&self, index: &str) -> &Self::Output {
        match self {
            Value::Map(m) => {
                for (k, v) in m {
                    if k.as_str().unwrap_or_default().eq(index) {
                        return v;
                    }
                }
                return &Value::Null;
            }
            Value::Ext(_, ext) => {
                return ext.index(index);
            }
            _ => {
                return &Value::Null;
            }
        }
    }
}

impl IndexMut<&str> for Value {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match self {
            Value::Map(m) => m.index_mut(index),
            Value::Ext(_, ext) => {
                return ext.index_mut(index);
            }
            _ => {
                panic!("not map type")
            }
        }
    }
}


impl Index<Value> for Value {
    type Output = Value;

    fn index(&self, index: Value) -> &Self::Output {
        return match self {
            Value::Array(arr) => {
                let idx = index.as_u64().unwrap_or_default() as usize;
                arr.index(idx)
            }
            Value::Map(map) => {
                let s = index.as_str().unwrap_or_default();
                map.index(s)
            }
            Value::Ext(_, ext) => {
                ext.index(index)
            }
            _ => {
                &Value::Null
            }
        };
    }
}


impl Index<&Value> for Value {
    type Output = Value;

    fn index(&self, index: &Value) -> &Self::Output {
        return match self {
            Value::Array(arr) => {
                let idx = index.as_u64().unwrap_or_default() as usize;
                arr.index(idx)
            }
            Value::Map(map) => {
                let s = index.as_str().unwrap_or_default();
                map.index(s)
            }
            Value::Ext(_, ext) => {
                ext.index(index)
            }
            _ => {
                &Value::Null
            }
        };
    }
}


impl IndexMut<Value> for Value {
    fn index_mut(&mut self, index: Value) -> &mut Self::Output {
        match self {
            Value::Array(arr) => {
                let idx = index.as_u64().unwrap_or_default() as usize;
                arr.index_mut(idx)
            }
            Value::Map(map) => {
                let s = index.as_str().unwrap_or_default();
                map.index_mut(s)
            }
            Value::Ext(_, ext) => {
                ext.index_mut(index)
            }
            _ => {
                panic!("not map/array type")
            }
        }
    }
}


impl IndexMut<&Value> for Value {
    fn index_mut(&mut self, index: &Value) -> &mut Self::Output {
        match self {
            Value::Array(arr) => {
                let idx = index.as_u64().unwrap_or_default() as usize;
                arr.index_mut(idx)
            }
            Value::Map(map) => {
                let s = index.as_str().unwrap_or_default();
                map.index_mut(s)
            }
            Value::Ext(_, ext) => {
                ext.index_mut(index)
            }
            _ => {
                panic!("not map/array type")
            }
        }
    }
}

impl Value {
    pub fn insert(&mut self, key: Value, value: Value) -> Option<Value> {
        match self {
            Value::Null => None,
            Value::Bool(_) => None,
            Value::I32(_) => None,
            Value::I64(_) => None,
            Value::U32(_) => None,
            Value::U64(_) => None,
            Value::F32(_) => None,
            Value::F64(_) => None,
            Value::String(_) => None,
            Value::Binary(_) => None,
            Value::Array(arr) => {
                arr.insert(key.as_u64().unwrap_or_default() as usize, value);
                None
            }
            Value::Map(m) => m.insert(key, value),
            Value::Ext(_, m) => m.insert(key, value),
        }
    }

    pub fn remove(&mut self, key: &Value) -> Value {
        match self {
            Value::Null => Value::Null,
            Value::Bool(_) => Value::Null,
            Value::I32(_) => Value::Null,
            Value::I64(_) => Value::Null,
            Value::U32(_) => Value::Null,
            Value::U64(_) => Value::Null,
            Value::F32(_) => Value::Null,
            Value::F64(_) => Value::Null,
            Value::String(_) => Value::Null,
            Value::Binary(_) => Value::Null,
            Value::Array(arr) => {
                let index = key.as_u64().unwrap_or_default() as usize;
                if index >= arr.len() {
                    return Value::Null;
                }
                arr.remove(index)
            }
            Value::Map(m) => m.remove(key),
            Value::Ext(_, e) => e.remove(key),
        }
    }
}
