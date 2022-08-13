use crate::ops::{Add, AsProxy};

use rbs::Value;
use std::borrow::Cow;

fn op_add_u64(value: &Value, other: u64) -> u64 {
    value.u64() + other
}

fn op_add_i64(value: &Value, other: i64) -> i64 {
    value.i64() + other
}

fn op_add_f64(value: &Value, other: f64) -> f64 {
    value.f64() + other
}

macro_rules! impl_numeric_add {
    ($($eq:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Add<$ty> for Value {
                type Output = $return_ty;
                fn op_add(self, other: $ty) -> Self::Output {
                    $eq(&self, other as _)
                }
            }

            impl Add<Value> for $ty {
                type Output = $return_ty;
                fn op_add(self, other: Value) -> Self::Output {
                    $eq(&other, self as _)
                }
            }

            impl Add<&Value> for $ty {
                type Output = $return_ty;
                fn op_add(self, other: &Value) -> Self::Output {
                    $eq(other, self as _)
                }
            }

            impl Add<&&Value> for $ty {
                type Output = $return_ty;
                fn op_add(self, other: &&Value) -> Self::Output {
                    $eq(*other, self as _)
                }
            }

            impl<'a> Add<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_add(self, other: $ty) -> Self::Output {
                    $eq(self, other as _)
                }
            }

            impl<'a> Add<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_add(self, other: &$ty) -> Self::Output {
                    $eq(self, *other as _)
                }
            }
        )*)*
    }
}

impl_numeric_add! {
    op_add_u64[u8 u16 u32 u64] -> u64
    op_add_i64[i8 i16 i32 i64 isize] -> i64
    op_add_f64[f32 f64] -> f64
}

//value
impl Add<&Value> for Value {
    type Output = Value;
    fn op_add(self, rhs: &Value) -> Self::Output {
        match self {
            Value::String(s) => Value::String(s + rhs.as_str().unwrap_or("")),
            Value::I32(s) => Value::I32(s + rhs.i32()),
            Value::I64(s) => Value::I64(s + rhs.i64()),
            Value::U32(s) => Value::U32(s + rhs.u32()),
            Value::U64(s) => Value::U64(s + rhs.u64()),
            Value::F64(s) => Value::F64(s + rhs.as_f64().unwrap_or_default()),
            Value::F32(v) => Value::F64(v as f64 + rhs.as_f64().unwrap_or_default()),
            _ => Value::Null,
        }
    }
}

impl Add<&&Value> for Value {
    type Output = Value;
    fn op_add(self, rhs: &&Value) -> Self::Output {
        match self {
            Value::String(s) => Value::String(s + rhs.as_str().unwrap_or("")),
            Value::I32(s) => Value::I32(s + rhs.i32()),
            Value::I64(s) => Value::I64(s + rhs.i64()),
            Value::U32(s) => Value::U32(s + rhs.u32()),
            Value::U64(s) => Value::U64(s + rhs.u64()),
            Value::F64(s) => Value::F64(s + rhs.as_f64().unwrap_or_default()),
            Value::F32(s) => Value::F64((s as f64 + rhs.as_f64().unwrap_or_default()) as f64),
            _ => Value::Null,
        }
    }
}

impl Add<Value> for Value {
    type Output = Value;
    fn op_add(self, rhs: Value) -> Self::Output {
        match self {
            Value::String(s) => Value::String(s + rhs.as_str().unwrap_or("")),
            Value::I32(s) => Value::I32(s + rhs.i32()),
            Value::I64(s) => Value::I64(s + rhs.i64()),
            Value::U32(s) => Value::U32(s + rhs.u32()),
            Value::U64(s) => Value::U64(s + rhs.u64()),
            Value::F64(s) => Value::F64(s + rhs.as_f64().unwrap_or_default()),
            Value::F32(s) => {
                Value::F64((s as f64 as f64 + rhs.as_f64().unwrap_or_default()) as f64)
            }
            _ => Value::Null,
        }
    }
}

impl Add<&Value> for &Value {
    type Output = Value;
    fn op_add(self, rhs: &Value) -> Self::Output {
        match self {
            Value::String(s) => Value::String(s.to_owned() + rhs.as_str().unwrap_or("")),
            Value::I32(s) => Value::I32(s + rhs.i32()),
            Value::I64(s) => Value::I64(s + rhs.i64()),
            Value::U32(s) => Value::U32(s + rhs.u32()),
            Value::U64(s) => Value::U64(s + rhs.u64()),
            Value::F64(s) => Value::F64(s + rhs.as_f64().unwrap_or_default()),
            Value::F32(s) => Value::F64(*s as f64 + rhs.as_f64().unwrap_or_default()),
            _ => Value::Null,
        }
    }
}

impl Add<&&Value> for &Value {
    type Output = Value;
    fn op_add(self, rhs: &&Value) -> Self::Output {
        match self {
            Value::String(s) => Value::String(s.to_owned() + rhs.as_str().unwrap_or("")),
            Value::I32(s) => Value::I32(s + rhs.i32()),
            Value::I64(s) => Value::I64(s + rhs.i64()),
            Value::U32(s) => Value::U32(s + rhs.u32()),
            Value::U64(s) => Value::U64(s + rhs.u64()),
            Value::F64(s) => Value::F64(s + rhs.as_f64().unwrap_or_default()),
            Value::F32(s) => Value::F64(*s as f64 + rhs.as_f64().unwrap_or_default()),
            _ => Value::Null,
        }
    }
}

impl Add<Value> for &Value {
    type Output = Value;
    fn op_add(self, rhs: Value) -> Self::Output {
        match self {
            Value::String(s) => Value::String(s.to_owned() + rhs.str()),
            Value::I32(s) => Value::I32(s + rhs.i32()),
            Value::I64(s) => Value::I64(s + rhs.i64()),
            Value::U32(s) => Value::U32(s + rhs.u32()),
            Value::U64(s) => Value::U64(s + rhs.u64()),
            Value::F64(s) => Value::F64(s + rhs.as_f64().unwrap_or_default()),
            Value::F32(s) => Value::F64(*s as f64 + rhs.as_f64().unwrap_or_default()),
            _ => Value::Null,
        }
    }
}

//str
impl Add<Value> for &str {
    type Output = String;
    fn op_add(self, rhs: Value) -> Self::Output {
        return match rhs {
            Value::String(s) => self.to_string() + s.as_str(),
            _ => String::new(),
        };
    }
}

impl Add<&Value> for &str {
    type Output = String;
    fn op_add(self, rhs: &Value) -> Self::Output {
        return match rhs {
            Value::String(s) => self.to_string() + s.as_str(),
            _ => String::new(),
        };
    }
}

impl Add<&str> for Value {
    type Output = String;
    fn op_add(self, rhs: &str) -> Self::Output {
        match self {
            Value::String(s) => s + rhs,
            _ => String::new(),
        }
    }
}

impl Add<&str> for &Value {
    type Output = String;
    fn op_add(self, rhs: &str) -> Self::Output {
        match self {
            Value::String(s) => s.to_string() + rhs,
            _ => String::new(),
        }
    }
}

impl Add<&&str> for Value {
    type Output = String;
    fn op_add(self, rhs: &&str) -> Self::Output {
        match self {
            Value::String(s) => s + *rhs,
            _ => String::new(),
        }
    }
}

impl Add<&&str> for &Value {
    type Output = String;
    fn op_add(self, rhs: &&str) -> Self::Output {
        match self {
            Value::String(s) => s.to_string() + *rhs,
            _ => String::new(),
        }
    }
}

impl Add<String> for Value {
    type Output = String;
    #[must_use]
    fn op_add(self, rhs: String) -> Self::Output {
        match self {
            Value::String(s) => s + rhs.as_str(),
            _ => String::new(),
        }
    }
}

impl Add<String> for &Value {
    type Output = String;
    fn op_add(self, rhs: String) -> Self::Output {
        match self {
            Value::String(s) => s.to_string() + rhs.as_str(),
            _ => String::new(),
        }
    }
}

//string ref
impl Add<Value> for &String {
    type Output = String;
    fn op_add(self, rhs: Value) -> Self::Output {
        return match rhs {
            Value::String(s) => self.to_string() + s.as_str(),
            _ => String::new(),
        };
    }
}

impl Add<&Value> for &String {
    type Output = String;
    fn op_add(self, rhs: &Value) -> Self::Output {
        return match rhs {
            Value::String(s) => self.to_string() + s.as_str(),
            _ => String::new(),
        };
    }
}

impl Add<&String> for Value {
    type Output = String;
    fn op_add(self, rhs: &String) -> Self::Output {
        match self {
            Value::String(s) => s + rhs.as_str(),
            _ => String::new(),
        }
    }
}

impl Add<&String> for &Value {
    type Output = String;
    fn op_add(self, rhs: &String) -> Self::Output {
        match self {
            Value::String(s) => s.to_string() + rhs.as_str(),
            _ => String::new(),
        }
    }
}

impl Add<Value> for String {
    type Output = String;
    fn op_add(self, rhs: Value) -> Self::Output {
        return match rhs {
            Value::String(s) => self + s.as_str(),
            _ => String::new(),
        };
    }
}

impl Add<&Value> for String {
    type Output = String;
    fn op_add(self, rhs: &Value) -> Self::Output {
        return match rhs {
            Value::String(s) => self + s.as_str(),
            _ => String::new(),
        };
    }
}

macro_rules! add_self {
    ([$($ty:ty)*]) => {
        $(
impl Add<$ty> for $ty{
         type Output = $ty;
      fn op_add(self, rhs: $ty) -> Self::Output {
        self+rhs
      }
    }
impl Add<&$ty> for $ty{
         type Output = $ty;
      fn op_add(self, rhs: &$ty) -> Self::Output {
        self+*rhs
      }
    }
impl Add<$ty> for &$ty{
         type Output = $ty;
      fn op_add(self, rhs: $ty) -> Self::Output {
        *self+rhs
      }
    }
impl Add<&$ty> for &$ty{
         type Output = $ty;
      fn op_add(self, rhs: &$ty) -> Self::Output {
        *self+*rhs
      }
    }
        )*
    };
}
add_self!([u8 u16 u32 u64]);
add_self!([i8 i16 i32 i64 isize]);
add_self!([f32 f64]);

impl Add<String> for String {
    type Output = String;

    fn op_add(self, rhs: String) -> Self::Output {
        self + &rhs
    }
}

impl Add<&str> for String {
    type Output = String;

    fn op_add(self, rhs: &str) -> Self::Output {
        self + rhs
    }
}

impl Add<&&str> for String {
    type Output = String;

    fn op_add(self, rhs: &&str) -> Self::Output {
        self + *rhs
    }
}

impl Add<String> for &str {
    type Output = String;

    fn op_add(self, rhs: String) -> Self::Output {
        self.to_string() + &rhs
    }
}

impl Add<&String> for &str {
    type Output = String;

    fn op_add(self, rhs: &String) -> Self::Output {
        self.to_string() + rhs.as_str()
    }
}

impl Add<&&String> for &str {
    type Output = String;

    fn op_add(self, rhs: &&String) -> Self::Output {
        self.to_string() + rhs.as_str()
    }
}
