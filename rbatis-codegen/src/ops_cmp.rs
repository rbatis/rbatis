use crate::ops::AsProxy;
use crate::ops::PartialOrd;
use rbs::Value;
use std::cmp::{Ordering, PartialOrd as P};

#[inline]
fn cmp_u64(value: u64, other: u64) -> Option<Ordering> {
    Some(value.cmp(&other))
}

#[inline]
fn cmp_i64(value: i64, other: i64) -> Option<Ordering> {
    Some(value.cmp(&other))
}

#[inline]
fn cmp_f64(value: f64, other: f64) -> Option<Ordering> {
    value.partial_cmp(&other)
}

#[inline]
fn cmp_bool(value: bool, other: bool) -> Option<Ordering> {
    Some(value.cmp(&other))
}

#[inline]
fn cmp_str(value: &str, other: &str) -> Option<Ordering> {
    value.partial_cmp(&other)
}
/**
PartialOrd
 **/

fn eq_u64(value: &Value, other: u64) -> Option<Ordering> {
    let value = value.u64();
    if value == other {
        Some(Ordering::Equal)
    } else if value > other {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_i64(value: &Value, other: i64) -> Option<Ordering> {
    let value = value.i64();
    if value == other {
        Some(Ordering::Equal)
    } else if value > other {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_f64(value: &Value, other: f64) -> Option<Ordering> {
    let value = value.f64();
    if value == other {
        Some(Ordering::Equal)
    } else if value > other {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_bool(value: &Value, other: bool) -> Option<Ordering> {
    let value = value.bool();
    if value == other {
        Some(Ordering::Equal)
    } else if value == true && other == false {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_str(value: &Value, other: &str) -> Option<Ordering> {
    let value = value.str();
    if value == other {
        Some(Ordering::Equal)
    } else if value > other {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

impl PartialOrd<Value> for Value {
    fn op_partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match self {
            Value::Null => Some(Ordering::Equal),
            Value::Bool(b) => cmp_bool(*b, other.bool()),
            Value::I32(n) => cmp_f64(*n as f64, other.f64()),
            Value::I64(n) => cmp_f64(*n as f64, other.f64()),
            Value::U32(n) => cmp_u64(*n as u64, other.u64()),
            Value::U64(n) => cmp_u64(*n as u64, other.u64()),
            Value::F64(n) => cmp_f64(*n, other.f64()),
            Value::F32(n) => cmp_f64(*n as f64, other.f64()),
            Value::String(s) => Some(s.as_str().cmp(other.str())),
            _ => None,
        }
    }
}

impl PartialOrd<Value> for &Value {
    fn op_partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match self {
            Value::Null => Some(Ordering::Equal),
            Value::Bool(b) => cmp_bool(*b, other.bool()),
            Value::I32(n) => cmp_f64(*n as f64, other.f64()),
            Value::I64(n) => cmp_f64(*n as f64, other.f64()),
            Value::U32(n) => cmp_u64(*n as u64, other.u64()),
            Value::U64(n) => cmp_u64(*n as u64, other.u64()),
            Value::F64(n) => cmp_f64(*n, other.f64()),
            Value::F32(n) => cmp_f64(*n as f64, other.f64()),
            Value::String(s) => Some(s.as_str().cmp(other.str())),
            _ => None,
        }
    }
}

impl PartialOrd<&Value> for &Value {
    fn op_partial_cmp(&self, other: &&Value) -> Option<Ordering> {
        match self {
            Value::Null => Some(Ordering::Equal),
            Value::Bool(b) => cmp_bool(*b, other.bool()),
            Value::I32(n) => cmp_f64(*n as f64, other.f64()),
            Value::I64(n) => cmp_f64(*n as f64, other.f64()),
            Value::U32(n) => cmp_u64(*n as u64, other.u64()),
            Value::U64(n) => cmp_u64(*n as u64, other.u64()),
            Value::F64(n) => cmp_f64(*n, other.f64()),
            Value::F32(n) => cmp_f64(*n as f64, other.f64()),
            Value::String(s) => Some(s.as_str().cmp(other.str())),
            _ => None,
        }
    }
}

impl PartialOrd<&&Value> for &Value {
    fn op_partial_cmp(&self, other: &&&Value) -> Option<Ordering> {
        match self {
            Value::Null => Some(Ordering::Equal),
            Value::Bool(b) => cmp_bool(*b, other.bool()),
            Value::I32(n) => cmp_f64(*n as f64, other.f64()),
            Value::I64(n) => cmp_f64(*n as f64, other.f64()),
            Value::U32(n) => cmp_u64(*n as u64, other.u64()),
            Value::U64(n) => cmp_u64(*n as u64, other.u64()),
            Value::F64(n) => cmp_f64(*n, other.f64()),
            Value::F32(n) => cmp_f64(*n as f64, other.f64()),
            Value::String(s) => Some(s.as_str().cmp(other.str())),
            _ => None,
        }
    }
}

impl PartialOrd<&Value> for Value {
    fn op_partial_cmp(&self, other: &&Value) -> Option<Ordering> {
        match self {
            Value::Null => Some(Ordering::Equal),
            Value::Bool(b) => cmp_bool(*b, other.bool()),
            Value::I32(n) => cmp_f64(*n as f64, other.f64()),
            Value::I64(n) => cmp_f64(*n as f64, other.f64()),
            Value::U32(n) => cmp_u64(*n as u64, other.u64()),
            Value::U64(n) => cmp_u64(*n as u64, other.u64()),
            Value::F64(n) => cmp_f64(*n, other.f64()),
            Value::F32(n) => cmp_f64(*n as f64, other.f64()),
            Value::String(s) => Some(s.as_str().cmp(other.str())),
            _ => None,
        }
    }
}

impl PartialOrd<&&Value> for Value {
    fn op_partial_cmp(&self, other: &&&Value) -> Option<Ordering> {
        match self {
            Value::Null => Some(Ordering::Equal),
            Value::Bool(b) => cmp_bool(*b, other.bool()),
            Value::I32(n) => cmp_f64(*n as f64, other.f64()),
            Value::I64(n) => cmp_f64(*n as f64, other.f64()),
            Value::U32(n) => cmp_u64(*n as u64, other.u64()),
            Value::U64(n) => cmp_u64(*n as u64, other.u64()),
            Value::F64(n) => cmp_f64(*n, other.f64()),
            Value::F32(n) => cmp_f64(*n as f64, other.f64()),
            Value::String(s) => Some(s.as_str().cmp(other.str())),
            _ => None,
        }
    }
}

macro_rules! impl_numeric_cmp {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialOrd<$ty> for Value {
                fn op_partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    $eq(self, *other as _)
                }
            }

            impl PartialOrd<Value> for $ty {
                fn op_partial_cmp(&self, other: &Value) -> Option<Ordering> {
                    $eq(other, *self as _)
                }
            }

            impl PartialOrd<&Value> for $ty {
                fn op_partial_cmp(&self, other: &&Value)  -> Option<Ordering> {
                    $eq(*other, *self as _)
                }
            }

            impl PartialOrd<&&Value> for $ty {
                fn op_partial_cmp(&self, other: &&&Value)  -> Option<Ordering> {
                    $eq(**other, *self as _)
                }
            }

            impl<'a> PartialOrd<$ty> for &'a Value {
                fn op_partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    $eq(*self, *other as _)
                }
            }
        )*)*
    }
}

impl_numeric_cmp! {
    eq_u64[u8 u16 u32 u64]
    eq_i64[i8 i16 i32 i64 isize]
    eq_f64[f32 f64]
    eq_bool[bool]
    eq_str[&str]
}

macro_rules! cmp_self {
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

cmp_self!(cmp_u64[u8 u16 u32 u64]);
cmp_self!(cmp_i64[i8 i16 i32 i64 isize]);
cmp_self!(cmp_f64[f32 f64]);

impl PartialOrd<&str> for &str {
    fn op_partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.partial_cmp(other)
    }
}

impl PartialOrd<&str> for String {
    fn op_partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl PartialOrd<String> for String {
    fn op_partial_cmp(&self, other: &String) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<&str> for &String {
    fn op_partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl PartialOrd<String> for &String {
    fn op_partial_cmp(&self, other: &String) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}
