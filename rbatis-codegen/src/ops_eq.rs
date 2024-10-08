use crate::ops::AsProxy;
use crate::ops::PartialEq;
use rbs::Value;
use std::borrow::Cow;
use std::cmp::PartialEq as PE;
use std::ops::Deref;

impl PartialEq<Value> for &'_ Value {
    fn op_eq(&self, other: &Value) -> bool {
        self.eq(&other)
    }
}

impl PartialEq<&Value> for &'_ Value {
    fn op_eq(&self, other: &&Value) -> bool {
        self.eq(&*other)
    }
}

impl PartialEq<&&Value> for &'_ Value {
    fn op_eq(&self, other: &&&Value) -> bool {
        self.eq(&**other)
    }
}

impl PartialEq<Value> for &&'_ Value {
    fn op_eq(&self, other: &Value) -> bool {
        (*self).eq(&other)
    }
}

impl PartialEq<&Value> for Value {
    fn op_eq(&self, other: &&Value) -> bool {
        self.eq(&**other)
    }
}

impl PartialEq<&&Value> for Value {
    fn op_eq(&self, other: &&&Value) -> bool {
        self.eq(&***other)
    }
}

impl PartialEq<Value> for Value {
    fn op_eq(&self, other: &Value) -> bool {
        self.eq(other)
    }
}

fn eq_i64(value: &Value, other: i64) -> bool {
    value.i64().eq(&other)
}

fn eq_f64(value: &Value, other: f64) -> bool {
    value.f64().eq(&other)
}

fn eq_bool(value: &Value, other: bool) -> bool {
    value.as_bool().unwrap_or_default().eq(&other)
}

fn eq_str(value: &Value, other: &str) -> bool {
    match value {
        Value::String(v) => v.eq(other),
        Value::Ext(_, ext) => eq_str(ext, other),
        _ => value.to_string().eq(other),
    }
}

impl PartialEq<str> for Value {
    fn op_eq(&self, other: &str) -> bool {
        eq_str(self, other)
    }
}

impl<'a> PartialEq<&'a str> for Value {
    fn op_eq(&self, other: &&str) -> bool {
        eq_str(self, *other)
    }
}

impl PartialEq<Value> for str {
    fn op_eq(&self, other: &Value) -> bool {
        eq_str(other, self)
    }
}

impl<'a> PartialEq<Value> for &'a str {
    fn op_eq(&self, other: &Value) -> bool {
        eq_str(other, *self)
    }
}

impl PartialEq<&str> for str {
    fn op_eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<String> for Value {
    fn op_eq(&self, other: &String) -> bool {
        eq_str(self, other.as_str())
    }
}

impl PartialEq<String> for &Value {
    fn op_eq(&self, other: &String) -> bool {
        eq_str(self, other.as_str())
    }
}

impl PartialEq<&str> for &Value {
    fn op_eq(&self, other: &&str) -> bool {
        eq_str(self, *other)
    }
}

impl PartialEq<Value> for String {
    fn op_eq(&self, other: &Value) -> bool {
        eq_str(other, self.as_str())
    }
}

impl PartialEq<Cow<'_, Value>> for Value {
    fn op_eq(&self, other: &Cow<'_, Value>) -> bool {
        self.eq(other.deref())
    }
}

macro_rules! impl_numeric_eq {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialEq<$ty> for Value {
               fn op_eq(&self, other: &$ty) -> bool {
                    $eq(self, *other as _)
                }
            }

            impl PartialEq<&$ty> for Value {
               fn op_eq(&self, other: &&$ty) -> bool {
                    $eq(self, **other as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a Value {
               fn op_eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }

            impl<'a> PartialEq<&$ty> for &'a Value {
               fn op_eq(&self, other: &&$ty) -> bool {
                    $eq(*self, **other as _)
                }
            }

            impl PartialEq<Value> for $ty {
               fn op_eq(&self, other: &Value) -> bool {
                    $eq(other, *self as _)
                }
            }

            impl PartialEq<&Value> for $ty {
               fn op_eq(&self, other: &&Value)  -> bool {
                    $eq(*other, *self as _)
                }
            }

            impl PartialEq<Value> for &$ty {
               fn op_eq(&self, other: &Value) -> bool {
                    $eq(other, **self as _)
                }
            }

            impl PartialEq<&Value> for &$ty {
               fn op_eq(&self, other: &&Value)  -> bool {
                    $eq(*other, **self as _)
                }
            }
            // for unary
            impl PartialEq<&&Value> for $ty {
               fn op_eq(&self, other: &&&Value)  -> bool {
                    $eq(*other, *self as _)
                }
            }
        )*)*
    }
}

impl_numeric_eq! {
    eq_i64[u8 u16 u32 u64 usize]
    eq_i64[i8 i16 i32 i64 isize]
    eq_f64[f32 f64]
    eq_bool[bool]
}

macro_rules! self_eq {
    ([$($ty:ty)*]) => {
        $(
impl PartialEq<$ty> for $ty{
      fn op_eq(&self, rhs: &$ty) -> bool {
          self.eq(rhs)
      }
    }
impl PartialEq<&$ty> for $ty{
      fn op_eq(&self, rhs: &&$ty) -> bool {
         self.eq(*rhs)
      }
    }
impl PartialEq<$ty> for &$ty{
      fn op_eq(&self, rhs: &$ty) -> bool {
          self.eq(&rhs)
      }
    }
impl PartialEq<&$ty> for &$ty{
      fn op_eq(&self, rhs: &&$ty) -> bool {
          self.eq(rhs)
      }
    }
        )*
    };
}

self_eq!([u8 u16 u32 u64 usize]);
self_eq!([i8 i16 i32 i64 isize]);
self_eq!([f32 f64]);
self_eq!([String & str]);

macro_rules! impl_str_eq {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialEq<$ty> for &str {
               fn op_eq(&self, other: &$ty) -> bool {
                    $eq(self, *other as _)
                }
            }

            impl PartialEq<&str> for $ty {
               fn op_eq(&self, other: &&str) -> bool {
                    $eq(other, *self as _)
                }
            }

            impl PartialEq<&&str> for $ty {
               fn op_eq(&self, other: &&&str)  -> bool {
                    $eq(*other, *self as _)
                }
            }

            impl PartialEq<&&&str> for $ty {
               fn op_eq(&self, other: &&&&str)  -> bool {
                    $eq(**other, *self as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a &str {
               fn op_eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }
        )*)*
    }
}

fn eq_str_i64(value: &str, other: i64) -> bool {
    value.eq(&other.to_string())
}

fn eq_str_f64(value: &str, other: f64) -> bool {
    value.eq(&other.to_string())
}

fn eq_str_bool(value: &str, other: bool) -> bool {
    match value {
        "true" => {
            if other {
                true
            } else {
                false
            }
        }
        "false" => {
            if !other {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
impl_str_eq! {
    eq_str_i64[u8 u16 u32 u64 usize]
    eq_str_i64[i8 i16 i32 i64 isize]
    eq_str_f64[f32 f64]
    eq_str_bool[bool]
}

#[cfg(test)]
mod test {
    use crate::ops::{Add, PartialEq};
    use rbs::{to_value, Value};

    #[test]
    fn test_eq() {
        let i: i64 = 1;
        let v = to_value!(1);
        let r = Value::from(v.op_add(&i));
        if r == Value::from(2) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_eq_str() {
        let v = to_value!(1);
        let r = v.op_ne("");
        assert_eq!(r, true);
    }
}
