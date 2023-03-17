use crate::Value;
use std::ops::{Index, IndexMut};

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Value {
        match self {
            Value::Array(arr) => &arr[index],
            _ => &Value::Null,
        }
    }
}

impl IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Value::Array(arr) => &mut arr[index],
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
                return m.index(index);
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
            _ => {
                panic!("not have index={}", index)
            }
        }
    }
}


impl Index<Value> for Value {
    type Output = Value;

    fn index(&self, index: Value) -> &Self::Output {
        match self {
            Value::Array(v) => {
                let index = index.as_u64();
                if index.is_none() {
                    panic!("index mut be int/uint,idnex={:?}", index)
                }
                let index = index.unwrap_or_default() as usize;
                &v[index]
            }
            Value::Map(v) => {
                &v[index]
            }
            _ => {
                panic!("not have index={}", index)
            }
        }
    }
}

impl IndexMut<Value> for Value {
    fn index_mut(&mut self, index: Value) -> &mut Self::Output {
        match self {
            Value::Array(v) => {
                let index = index.as_u64();
                if index.is_none() {
                    panic!("index mut be int/uint,idnex={:?}", index)
                }
                let index = index.unwrap_or_default() as usize;
                &mut v[index]
            }
            Value::Map(v) => {
                &mut v[index]
            }
            _ => {
                panic!("not have index={}", index)
            }
        }
    }
}

impl Index<&Value> for Value {
    type Output = Value;

    fn index(&self, index: &Value) -> &Self::Output {
        match self {
            Value::Array(v) => {
                let index = index.as_u64();
                if index.is_none() {
                    panic!("index mut be int/uint,idnex={:?}", index)
                }
                let index = index.unwrap_or_default() as usize;
                &v[index]
            }
            Value::Map(v) => {
                &v[index]
            }
            _ => {
                panic!("not have index={}", index)
            }
        }
    }
}

impl IndexMut<&Value> for Value {
    fn index_mut(&mut self, index: &Value) -> &mut Self::Output {
        match self {
            Value::Array(v) => {
                let index = index.as_u64();
                if index.is_none() {
                    panic!("index mut be int/uint,idnex={:?}", index)
                }
                let index = index.unwrap_or_default() as usize;
                &mut v[index]
            }
            Value::Map(v) => {
                &mut v[index]
            }
            _ => {
                panic!("not have index={}", index)
            }
        }
    }
}