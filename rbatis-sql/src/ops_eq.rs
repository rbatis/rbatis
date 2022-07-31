use crate::ops::{AsProxy, Value};
use crate::ops::PartialEq;
use std::cmp::PartialEq as PE;


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

impl PartialEq<&Value> for Value{
   fn op_eq(&self, other: &&Value) -> bool {
        self.eq(&**other)
    }
}
impl PartialEq<&&Value> for Value{
    fn op_eq(&self, other: &&&Value) -> bool {
        self.eq(&***other)
    }
}

impl PartialEq<Value> for Value{
    fn op_eq(&self, other: &Value) -> bool {
        self.eq(other)
    }
}

/**
eq base
**/
fn eq_u64(value: &Value, other: u64) -> bool {
    value.u64().eq(&other)
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
    value.as_str().unwrap_or_default().eq(other)
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

impl PartialEq<Value> for String {
   fn op_eq(&self, other: &Value) -> bool {
        eq_str(other, self.as_str())
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

            impl PartialEq<&&Value> for $ty {
               fn op_eq(&self, other: &&&Value)  -> bool {
                    $eq(**other, *self as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a Value {
               fn op_eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }
        )*)*
    }
}

impl_numeric_eq! {
    eq_i64[u8 u16 u32 u64]
    eq_i64[i8 i16 i32 i64 isize]
    eq_f64[f32 f64]
    eq_bool[bool]
}


macro_rules! eq_self {
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

eq_self!([u8 u16 u32 u64]);
eq_self!([i8 i16 i32 i64 isize]);
eq_self!([f32 f64]);
eq_self!([String &str]);