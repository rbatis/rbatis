use crate::{to_value, Value};
use serde::Serialize;
use std::ops::{Index, IndexMut};

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Value {
        match self {
            Value::Array(arr) => &arr[index],
            Value::Ext(_, ext) => {
                return ext.index(index);
            }
            _ => {
                &Value::Null
            }
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
