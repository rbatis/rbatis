use crate::ops::AsProxy;
use crate::ops::PartialOrd;
use rbs::Value;
use std::cmp::{Ordering, PartialOrd as P};

#[inline]
fn cmp_u64(value: u64, rhs: u64) -> Option<Ordering> {
    Some(value.cmp(&rhs))
}

#[inline]
fn cmp_i64(value: i64, rhs: i64) -> Option<Ordering> {
    Some(value.cmp(&rhs))
}

#[inline]
fn cmp_f64(value: f64, rhs: f64) -> Option<Ordering> {
    value.partial_cmp(&rhs)
}

#[inline]
fn cmp_bool(value: bool, rhs: bool) -> Option<Ordering> {
    Some(value.cmp(&rhs))
}

/**
PartialOrd
 **/

fn eq_u64(value: &Value, rhs: u64) -> Option<Ordering> {
    let value = value.u64();
    if value == rhs {
        Some(Ordering::Equal)
    } else if value > rhs {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_i64(value: &Value, rhs: i64) -> Option<Ordering> {
    let value = value.i64();
    if value == rhs {
        Some(Ordering::Equal)
    } else if value > rhs {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_f64(value: &Value, rhs: f64) -> Option<Ordering> {
    let value = value.f64();
    if value == rhs {
        Some(Ordering::Equal)
    } else if value > rhs {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_bool(value: &Value, rhs: bool) -> Option<Ordering> {
    let value = value.bool();
    if value == rhs {
        Some(Ordering::Equal)
    } else if value == true && rhs == false {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_str(value: &Value, rhs: &str) -> Option<Ordering> {
    let value = value.clone().string();
    if value == rhs {
        Some(Ordering::Equal)
    } else if value.as_str() > rhs {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn op_partial_cmp_value(left: &Value, rhs: &Value) -> Option<Ordering> {
    match left {
        Value::Null => Some(Ordering::Equal),
        Value::Bool(b) => cmp_bool(*b, rhs.bool()),
        Value::I32(n) => cmp_f64(*n as f64, rhs.f64()),
        Value::I64(n) => cmp_f64(*n as f64, rhs.f64()),
        Value::U32(n) => cmp_u64(*n as u64, rhs.u64()),
        Value::U64(n) => cmp_u64(*n as u64, rhs.u64()),
        Value::F32(n) => cmp_f64(*n as f64, rhs.f64()),
        Value::F64(n) => cmp_f64(*n, rhs.f64()),
        Value::String(s) => Some(s.as_str().cmp(&rhs.clone().string())),
        Value::Ext(_, e) => op_partial_cmp_value(e.as_ref(), rhs),
        _ => None,
    }
}

impl PartialOrd<&Value> for &Value {
    fn op_partial_cmp(&self, rhs: &&Value) -> Option<Ordering> {
        op_partial_cmp_value(self, rhs)
    }
}

impl PartialOrd<Value> for Value {
    fn op_partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
        op_partial_cmp_value(self, rhs)
    }
}

impl PartialOrd<Value> for &Value {
    fn op_partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
        op_partial_cmp_value(self, rhs)
    }
}

impl PartialOrd<&&Value> for &Value {
    fn op_partial_cmp(&self, rhs: &&&Value) -> Option<Ordering> {
        op_partial_cmp_value(self, rhs)
    }
}

impl PartialOrd<&Value> for Value {
    fn op_partial_cmp(&self, rhs: &&Value) -> Option<Ordering> {
        op_partial_cmp_value(self, rhs)
    }
}

impl PartialOrd<&&Value> for Value {
    fn op_partial_cmp(&self, rhs: &&&Value) -> Option<Ordering> {
        op_partial_cmp_value(self, rhs)
    }
}

macro_rules! impl_numeric_cmp {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialOrd<$ty> for Value {
                fn op_partial_cmp(&self, rhs: &$ty) -> Option<Ordering> {
                    $eq(self, *rhs as _)
                }
            }

            impl PartialOrd<&$ty> for Value {
                fn op_partial_cmp(&self, rhs: &&$ty) -> Option<Ordering> {
                    $eq(self, **rhs as _)
                }
            }

            impl<'a> PartialOrd<$ty> for &'a Value {
                fn op_partial_cmp(&self, rhs: &$ty) -> Option<Ordering> {
                    $eq(*self, *rhs as _)
                }
            }

            impl<'a> PartialOrd<&$ty> for &'a Value {
                fn op_partial_cmp(&self, rhs: &&$ty) -> Option<Ordering> {
                    $eq(*self, **rhs as _)
                }
            }

            impl PartialOrd<Value> for $ty {
                fn op_partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
                    $eq(rhs, *self as _)
                }
            }

            impl PartialOrd<&Value> for $ty {
                fn op_partial_cmp(&self, rhs: &&Value)  -> Option<Ordering> {
                    $eq(*rhs, *self as _)
                }
            }

            impl PartialOrd<Value> for &$ty {
                fn op_partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
                    $eq(rhs, **self as _)
                }
            }

            impl PartialOrd<&Value> for &$ty {
                fn op_partial_cmp(&self, rhs: &&Value)  -> Option<Ordering> {
                    $eq(*rhs, **self as _)
                }
            }

            // for unary
            impl PartialOrd<&&Value> for $ty {
                fn op_partial_cmp(&self, rhs: &&&Value)  -> Option<Ordering> {
                    $eq(*rhs, *self as _)
                }
            }
        )*)*
    }
}

impl_numeric_cmp! {
    eq_u64[u8 u16 u32 u64]
    eq_i64[i8 i16 i32 i64 isize usize]
    eq_f64[f32 f64]
    eq_bool[bool]
    eq_str[&str]
}

macro_rules! self_cmp {
    ($eq:ident[$($ty:ty)*]) => {
        $(
impl PartialOrd<$ty> for $ty{
      fn op_partial_cmp(&self, rhs: &$ty) ->  Option<Ordering> {
        $eq(*self as _, *rhs as _)
      }
}
impl PartialOrd<&$ty> for $ty{
      fn op_partial_cmp(&self, rhs: &&$ty) ->  Option<Ordering> {
        $eq(*self as _, **rhs as _)
      }
}
impl PartialOrd<$ty> for &$ty{
      fn op_partial_cmp(&self, rhs: &$ty) ->  Option<Ordering> {
        $eq(**self as _, *rhs as _)
      }
}
impl PartialOrd<&$ty> for &$ty{
      fn op_partial_cmp(&self, rhs: &&$ty) ->  Option<Ordering> {
        $eq(**self as _, **rhs as _)
      }
}
        )*
    };
}

self_cmp!(cmp_u64[u8 u16 u32 u64]);
self_cmp!(cmp_i64[i8 i16 i32 i64 isize usize]);
self_cmp!(cmp_f64[f32 f64]);

impl PartialOrd<&str> for &str {
    fn op_partial_cmp(&self, rhs: &&str) -> Option<Ordering> {
        self.partial_cmp(rhs)
    }
}

impl PartialOrd<&str> for String {
    fn op_partial_cmp(&self, rhs: &&str) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs)
    }
}

impl PartialOrd<String> for String {
    fn op_partial_cmp(&self, rhs: &String) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs.as_str())
    }
}

impl PartialOrd<&String> for String {
    fn op_partial_cmp(&self, rhs: &&String) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs.as_str())
    }
}

impl PartialOrd<&&String> for String {
    fn op_partial_cmp(&self, rhs: &&&String) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs.as_str())
    }
}

impl PartialOrd<&str> for &String {
    fn op_partial_cmp(&self, rhs: &&str) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs)
    }
}

impl PartialOrd<&&str> for &String {
    fn op_partial_cmp(&self, rhs: &&&str) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs)
    }
}

impl PartialOrd<&&&str> for &String {
    fn op_partial_cmp(&self, rhs: &&&&str) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs)
    }
}

impl PartialOrd<String> for &String {
    fn op_partial_cmp(&self, rhs: &String) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs.as_str())
    }
}
impl PartialOrd<&String> for &String {
    fn op_partial_cmp(&self, rhs: &&String) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs.as_str())
    }
}
impl PartialOrd<&&String> for &String {
    fn op_partial_cmp(&self, rhs: &&&String) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs.as_str())
    }
}

impl PartialOrd<String> for &&String {
    fn op_partial_cmp(&self, rhs: &String) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs.as_str())
    }
}

impl PartialOrd<&String> for &&String {
    fn op_partial_cmp(&self, rhs: &&String) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs.as_str())
    }
}

impl PartialOrd<&&String> for &&String {
    fn op_partial_cmp(&self, rhs: &&&String) -> Option<Ordering> {
        self.as_str().partial_cmp(rhs.as_str())
    }
}

macro_rules! cmp_diff {
    ($eq:ident[$(($ty1:ty,$ty2:ty),)*]) => {
        $(
        impl PartialOrd<$ty1> for $ty2{
              fn op_partial_cmp(&self, rhs: &$ty1) ->  Option<Ordering> {
                $eq(*self as _, *rhs as _)
              }
        }
        impl PartialOrd<&$ty1> for $ty2{
              fn op_partial_cmp(&self, rhs: &&$ty1) ->  Option<Ordering> {
                $eq(*self as _, **rhs as _)
              }
        }
        impl PartialOrd<$ty1> for &$ty2{
              fn op_partial_cmp(&self, rhs: &$ty1) ->  Option<Ordering> {
                $eq(**self as _, *rhs as _)
              }
        }
        impl PartialOrd<&$ty1> for &$ty2{
              fn op_partial_cmp(&self, rhs: &&$ty1) ->  Option<Ordering> {
                $eq(**self as _, **rhs as _)
              }
        }
        )*
    };
}

cmp_diff!(cmp_i64[(i64,i8),(i64,i16),(i64,i32),]);
cmp_diff!(cmp_i64[(i64,u8),(i64,u16),(i64,u32),(i64,u64),(i64,usize),]);
cmp_diff!(cmp_f64[(i64,f32),(i64,f64),]);

cmp_diff!(cmp_i64[(u64,i8),(u64,i16),(u64,i32),(u64,i64),]);
cmp_diff!(cmp_u64[(u64,u8),(u64,u16),(u64,u32),(u64,usize),]);
cmp_diff!(cmp_f64[(u64,f32),(u64,f64),]);

cmp_diff!(cmp_f64[(f64,u8),(f64,u16),(f64,u32),(f64,u64),(f64,usize),]);
cmp_diff!(cmp_f64[(f64,i8),(f64,i16),(f64,i32),(f64,i64),]);
cmp_diff!(cmp_f64[(f64,f32),]);
