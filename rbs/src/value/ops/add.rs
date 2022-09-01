use std::ops::Add;
use crate::Value;

impl Add<Value> for Value{
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match self{
            Value::Null => {self}
            Value::Bool(_) => {
                self
            }
            Value::I32(v) => {
                Value::I32(v+rhs.as_i64().unwrap_or_default() as i32)
            }
            Value::I64(v) => {
                Value::I64(v+rhs.as_i64().unwrap_or_default())
            }
            Value::U32(v) => {
                Value::U32(v+rhs.as_u64().unwrap_or_default() as u32)
            }
            Value::U64(v) => {
                Value::U64(v+rhs.as_u64().unwrap_or_default())
            }
            Value::F32(v) => {
                Value::F32(v+rhs.as_f64().unwrap_or_default() as f32)
            }
            Value::F64(v) => {
                Value::F64(v+rhs.as_f64().unwrap_or_default())
            }
            Value::String(v) => {
                Value::String(v+rhs.as_str().unwrap_or_default())
            }
            Value::Binary(_) => {
                self
            }
            Value::Array(_) => {self}
            Value::Map(_) => {self}
            Value::Ext(s, ext) => {
                let e= rhs.into_ext(s);
                match e{
                    Value::Ext(_,rhs)=>{
                        Value::Ext(s, Box::new(*ext+*rhs))
                    }
                    _ => {
                        Value::Ext(s, ext)
                    }
                }
            }
        }
    }
}