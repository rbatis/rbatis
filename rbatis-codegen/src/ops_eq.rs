use crate::ops::PartialEq;
use rbs::Value;
use std::borrow::Cow;
use std::cmp::PartialEq as PE;
use std::ops::Deref;

#[inline]
fn eq_i64(value: i64, rhs: i64) -> bool {
    value == rhs
}

#[inline]
fn eq_f64(value: f64, rhs: f64) -> bool {
    value == rhs
}

#[inline]
fn eq_bool(value: bool, rhs: bool) -> bool {
    value == rhs
}

#[inline]
fn eq_str(value: &str, rhs: &str) -> bool {
    value == rhs
}

#[inline]
fn into_i64(value: &Value) -> i64 {
    value.as_i64().unwrap_or_default()
}

#[inline]
fn into_f64(value: &Value) -> f64 {
    value.as_f64().unwrap_or_default()
}

#[inline]
fn into_bool(value: &Value) -> bool {
    value.as_bool().unwrap_or_default()
}

impl PartialEq<Value> for &Value {
    fn op_eq(&self, other: &Value) -> bool {
        self.eq(&other)
    }
}

impl PartialEq<&Value> for &Value {
    fn op_eq(&self, other: &&Value) -> bool {
        self.eq(&*other)
    }
}

impl PartialEq<&&Value> for &Value {
    fn op_eq(&self, other: &&&Value) -> bool {
        self.eq(&**other)
    }
}

impl PartialEq<&&Value> for &&Value {
    fn op_eq(&self, other: &&&Value) -> bool {
        self.eq(other)
    }
}

impl PartialEq<Value> for &&Value {
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

impl PartialEq<str> for Value {
    fn op_eq(&self, other: &str) -> bool {
        eq_str(self.as_str().unwrap_or_default(), other)
    }
}

impl<'a> PartialEq<&'a str> for Value {
    fn op_eq(&self, other: &&str) -> bool {
        eq_str(self.as_str().unwrap_or_default(), *other)
    }
}

impl PartialEq<Value> for str {
    fn op_eq(&self, other: &Value) -> bool {
        eq_str(other.as_str().unwrap_or_default(), self)
    }
}

impl<'a> PartialEq<Value> for &'a str {
    fn op_eq(&self, other: &Value) -> bool {
        eq_str(other.as_str().unwrap_or_default(), *self)
    }
}

impl PartialEq<&str> for str {
    fn op_eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<String> for Value {
    fn op_eq(&self, other: &String) -> bool {
        eq_str(self.as_str().unwrap_or_default(), other.as_str())
    }
}

impl PartialEq<String> for &Value {
    fn op_eq(&self, other: &String) -> bool {
        eq_str(self.as_str().unwrap_or_default(), other.as_str())
    }
}

impl PartialEq<&str> for &Value {
    fn op_eq(&self, other: &&str) -> bool {
        eq_str(self.as_str().unwrap_or_default(), *other)
    }
}

impl PartialEq<Value> for String {
    fn op_eq(&self, other: &Value) -> bool {
        eq_str(other.as_str().unwrap_or_default(), self.as_str())
    }
}

impl PartialEq<Cow<'_, Value>> for Value {
    fn op_eq(&self, other: &Cow<'_, Value>) -> bool {
        self.eq(other.deref())
    }
}

macro_rules! impl_numeric_eq {
    ($($eq:ident,$into:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialEq<$ty> for Value {
               fn op_eq(&self, other: &$ty) -> bool {
                    $eq($into(self), *other as _)
                }
            }

            impl PartialEq<&$ty> for Value {
               fn op_eq(&self, other: &&$ty) -> bool {
                    $eq($into(self), **other as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a Value {
               fn op_eq(&self, other: &$ty) -> bool {
                    $eq($into(*self), *other as _)
                }
            }

            impl<'a> PartialEq<&$ty> for &'a Value {
               fn op_eq(&self, other: &&$ty) -> bool {
                    $eq($into(*self), **other as _)
                }
            }

            impl PartialEq<Value> for $ty {
               fn op_eq(&self, other: &Value) -> bool {
                    $eq($into(other), *self as _)
                }
            }

            impl PartialEq<&Value> for $ty {
               fn op_eq(&self, other: &&Value)  -> bool {
                    $eq($into(*other), *self as _)
                }
            }

            impl PartialEq<Value> for &$ty {
               fn op_eq(&self, other: &Value) -> bool {
                    $eq($into(other), **self as _)
                }
            }

            impl PartialEq<&Value> for &$ty {
               fn op_eq(&self, other: &&Value)  -> bool {
                    $eq($into(*other), **self as _)
                }
            }
            // for unary
            impl PartialEq<&&Value> for $ty {
               fn op_eq(&self, other: &&&Value)  -> bool {
                    $eq($into(**other), *self as _)
                }
            }
        )*)*
    }
}

impl_numeric_eq! {
    eq_i64,into_i64[u8 u16 u32 u64]
    eq_i64,into_i64[i8 i16 i32 i64 isize usize]
    eq_f64,into_f64[f32 f64]
    eq_bool,into_bool[bool]
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

self_eq!([u8 u16 u32 u64]);
self_eq!([i8 i16 i32 i64 isize usize]);
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
    eq_str_i64[u8 u16 u32 u64]
    eq_str_i64[i8 i16 i32 i64 isize usize]
    eq_str_f64[f32 f64]
    eq_str_bool[bool]
}

macro_rules! eq_diff {
   ($eq:ident[$(($ty1:ty,$ty2:ty),)*]) => {
        $(
        impl PartialEq<$ty1> for $ty2{
          fn op_eq(&self, rhs: &$ty1) -> bool {
              rhs.eq(&(*self as $ty1))
          }
        }
        impl PartialEq<&$ty1> for $ty2{
          fn op_eq(&self, rhs: &&$ty1) -> bool {
             (*rhs).eq(&(*self as $ty1))
          }
        }
        impl PartialEq<$ty1> for &$ty2{
          fn op_eq(&self, rhs: &$ty1) -> bool {
             rhs.eq(&(**self as $ty1))
          }
        }
        impl PartialEq<&$ty1> for &$ty2{
          fn op_eq(&self, rhs: &&$ty1) -> bool {
             (*rhs).eq(&(**self as $ty1))
          }
        }
        )*
    };
}

eq_diff!(eq_i64[(i64,i8),(i64,i16),(i64,i32),]);
eq_diff!(eq_i64[(i64,u8),(i64,u16),(i64,u32),(i64,u64),(i64,usize),]);
eq_diff!(eq_f64[(i64,f32),(i64,f64),]);

eq_diff!(eq_i64[(u64,i8),(u64,i16),(u64,i32),(u64,i64),]);
eq_diff!(eq_u64[(u64,u8),(u64,u16),(u64,u32),(u64,usize),]);
eq_diff!(eq_f64[(u64,f32),(u64,f64),]);

eq_diff!(eq_f64[(f64,u8),(f64,u16),(f64,u32),(f64,u64),(f64,usize),]);
eq_diff!(eq_f64[(f64,i8),(f64,i16),(f64,i32),(f64,i64),]);
eq_diff!(eq_f64[(f64,f32),]);

#[cfg(test)]
mod test {
    use crate::ops::{Add};
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
}
