use std::ops::{Index, IndexMut};
use serde::Serialize;
use crate::{to_value, Value};

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Value {
        match self {
            Value::Array(arr) => {
                &arr[index]
            }
            Value::Ext(_, ext) => {
                return ext.index(index);
            }
            _ => {
                panic!("not an array!")
            }
        }
    }
}

impl IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Value::Array(arr) => {
                &mut arr[index]
            }
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
            Value::Map(m) => {
                for (k, v) in m {
                    if k.as_str().unwrap_or_default().eq(index) {
                        return v;
                    }
                }
                panic!("not have index={}", index)
            }
            Value::Ext(_, ext) => {
                return ext.index_mut(index);
            }
            _ => {
                panic!("not have index={}", index)
            }
        }
    }
}

impl Value {
    pub fn insert<K: Serialize + Eq, V: Serialize>(&mut self, k: K, v: V) {
        match self {
            Value::Array(arr) => {
                let k = to_value!(k);
                arr.insert(k.as_u64().unwrap_or_default() as usize,to_value!(v));
            }
            Value::Map(m) => {
                let k = to_value!(k);
                let mut inserted = false;
                for (mk, mv) in &mut *m {
                    if k.eq(mk) {
                        *mv = to_value!(v);
                        inserted = true;
                        return;
                        ;
                    }
                }
                if !inserted {
                    m.push((to_value!(k), to_value!(v)));
                }
            }
            Value::Ext(_, e) => {
                e.insert(k, v)
            }
            _ => {}
        }
    }

    pub fn remove<S: Serialize>(&mut self, key: S) -> Option<Value> {
        let k = to_value(key).unwrap_or_default();
        match self {
            Value::Array(array) => {
                let k = k.as_u64().unwrap_or_default() as usize;
                if (k + 1) >= array.len() {
                    None
                } else {
                    Some(array.remove(k))
                }
            }
            Value::Map(map) => {
                let mut idx = 0;
                for (mkey, v) in &*map {
                    if k.eq(mkey) {
                        let (_, v) = map.remove(idx);
                        return Some(v);
                    }
                    idx += 1
                }
                return None;
            }
            Value::Ext(_, e) => {
                e.remove(k)
            }
            _ => {
                None
            }
        }
    }
}
