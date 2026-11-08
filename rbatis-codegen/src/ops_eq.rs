use crate::ops::PartialEq;
use crate::ops_compare::{value_eq, value_eq_bool, value_eq_f64, value_eq_i64, value_eq_u64};
use crate::ops_macros::{impl_compare_matrix, impl_value_scalar_compare_op};
use rbs::Value;
use std::borrow::Cow;
use std::ops::Deref;

#[inline]
fn eq_i128(value: i128, rhs: i128) -> bool {
    value == rhs
}

#[inline]
fn eq_f64(value: f64, rhs: f64) -> bool {
    value == rhs
}

#[inline]
fn eq_str(value: &str, rhs: &str) -> bool {
    value == rhs
}

impl PartialEq<Value> for Value {
    fn op_eq(&self, other: &Value) -> bool {
        value_eq(self, other)
    }
}

impl PartialEq<&Value> for Value {
    fn op_eq(&self, other: &&Value) -> bool {
        value_eq(self, other)
    }
}

impl PartialEq<&&Value> for Value {
    fn op_eq(&self, other: &&&Value) -> bool {
        value_eq(self, other)
    }
}

impl PartialEq<Value> for &Value {
    fn op_eq(&self, other: &Value) -> bool {
        value_eq(self, other)
    }
}

impl PartialEq<&Value> for &Value {
    fn op_eq(&self, other: &&Value) -> bool {
        value_eq(self, other)
    }
}

impl PartialEq<&&Value> for &Value {
    fn op_eq(&self, other: &&&Value) -> bool {
        value_eq(self, other)
    }
}

impl PartialEq<Value> for &&Value {
    fn op_eq(&self, other: &Value) -> bool {
        value_eq(self, other)
    }
}

impl PartialEq<&Value> for &&Value {
    fn op_eq(&self, other: &&Value) -> bool {
        value_eq(self, other)
    }
}

impl PartialEq<&&Value> for &&Value {
    fn op_eq(&self, other: &&&Value) -> bool {
        value_eq(self, other)
    }
}

impl PartialEq<str> for Value {
    fn op_eq(&self, other: &str) -> bool {
        eq_str(self.as_str().unwrap_or_default(), other)
    }
}

impl PartialEq<&str> for Value {
    fn op_eq(&self, other: &&str) -> bool {
        eq_str(self.as_str().unwrap_or_default(), other)
    }
}

impl PartialEq<Value> for str {
    fn op_eq(&self, other: &Value) -> bool {
        eq_str(other.as_str().unwrap_or_default(), self)
    }
}

impl PartialEq<Value> for &str {
    fn op_eq(&self, other: &Value) -> bool {
        eq_str(other.as_str().unwrap_or_default(), self)
    }
}

impl PartialEq<&str> for str {
    fn op_eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for Value {
    fn op_eq(&self, other: &String) -> bool {
        eq_str(self.as_str().unwrap_or_default(), other.as_str())
    }
}

impl PartialEq<&String> for Value {
    fn op_eq(&self, other: &&String) -> bool {
        eq_str(self.as_str().unwrap_or_default(), other.as_str())
    }
}

impl PartialEq<String> for &Value {
    fn op_eq(&self, other: &String) -> bool {
        eq_str(self.as_str().unwrap_or_default(), other.as_str())
    }
}

impl PartialEq<&String> for &Value {
    fn op_eq(&self, other: &&String) -> bool {
        eq_str(self.as_str().unwrap_or_default(), other.as_str())
    }
}

impl PartialEq<&str> for &Value {
    fn op_eq(&self, other: &&str) -> bool {
        eq_str(self.as_str().unwrap_or_default(), other)
    }
}

impl PartialEq<Value> for String {
    fn op_eq(&self, other: &Value) -> bool {
        eq_str(other.as_str().unwrap_or_default(), self.as_str())
    }
}

impl PartialEq<Value> for &String {
    fn op_eq(&self, other: &Value) -> bool {
        eq_str(other.as_str().unwrap_or_default(), self.as_str())
    }
}

impl PartialEq<Cow<'_, Value>> for Value {
    fn op_eq(&self, other: &Cow<'_, Value>) -> bool {
        value_eq(self, other.deref())
    }
}

impl_value_scalar_compare_op!(
    PartialEq,
    op_eq,
    value_eq_u64,
    value_eq_u64,
    [u8, u16, u32, u64],
    bool
);
impl_value_scalar_compare_op!(
    PartialEq,
    op_eq,
    value_eq_i64,
    value_eq_i64,
    [i8, i16, i32, i64, isize, usize],
    bool
);
impl_value_scalar_compare_op!(
    PartialEq,
    op_eq,
    value_eq_f64,
    value_eq_f64,
    [f32, f64],
    bool
);
impl_value_scalar_compare_op!(PartialEq, op_eq, value_eq_bool, value_eq_bool, [bool], bool);

macro_rules! impl_same_type_eq {
    ([$($ty:ty)*]) => {
        $(
            impl PartialEq<$ty> for $ty {
                fn op_eq(&self, rhs: &$ty) -> bool {
                    self == rhs
                }
            }

            impl PartialEq<&$ty> for $ty {
                fn op_eq(&self, rhs: &&$ty) -> bool {
                    self == *rhs
                }
            }

            impl PartialEq<$ty> for &$ty {
                fn op_eq(&self, rhs: &$ty) -> bool {
                    *self == rhs
                }
            }

            impl PartialEq<&$ty> for &$ty {
                fn op_eq(&self, rhs: &&$ty) -> bool {
                    *self == *rhs
                }
            }
        )*
    };
}

impl_same_type_eq! {
    [bool]
}
impl_same_type_eq! {
    [String]
}
impl_same_type_eq! {
    [&str]
}

macro_rules! impl_str_eq {
    ($($compare:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialEq<$ty> for &str {
                fn op_eq(&self, other: &$ty) -> bool {
                    $compare(self, *other as _)
                }
            }

            impl PartialEq<&str> for $ty {
                fn op_eq(&self, other: &&str) -> bool {
                    $compare(other, *self as _)
                }
            }

            impl PartialEq<&&str> for $ty {
                fn op_eq(&self, other: &&&str) -> bool {
                    $compare(*other, *self as _)
                }
            }

            impl PartialEq<&&&str> for $ty {
                fn op_eq(&self, other: &&&&str) -> bool {
                    $compare(**other, *self as _)
                }
            }

            impl PartialEq<$ty> for &&str {
                fn op_eq(&self, other: &$ty) -> bool {
                    $compare(*self, *other as _)
                }
            }
        )*)*
    };
}

fn eq_str_u64(value: &str, other: u64) -> bool {
    value == other.to_string()
}

fn eq_str_i64(value: &str, other: i64) -> bool {
    value == other.to_string()
}

fn eq_str_f64(value: &str, other: f64) -> bool {
    value == other.to_string()
}

fn eq_str_bool(value: &str, other: bool) -> bool {
    match value {
        "true" => other,
        "false" => !other,
        _ => false,
    }
}

impl_str_eq! {
    eq_str_u64 [u8 u16 u32 u64]
    eq_str_i64 [i8 i16 i32 i64 isize usize]
    eq_str_f64 [f32 f64]
    eq_str_bool [bool]
}

impl_compare_matrix!(
    PartialEq,
    op_eq,
    eq_i128,
    i128,
    bool;
    u8 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    u16 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    u32 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    u64 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    usize => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    i8 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    i16 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    i32 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    i64 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    isize => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize]
);
impl_compare_matrix!(
    PartialEq,
    op_eq,
    eq_f64,
    f64,
    bool;
    u8 => [f32, f64],
    u16 => [f32, f64],
    u32 => [f32, f64],
    u64 => [f32, f64],
    usize => [f32, f64],
    i8 => [f32, f64],
    i16 => [f32, f64],
    i32 => [f32, f64],
    i64 => [f32, f64],
    isize => [f32, f64],
);
impl_compare_matrix!(
    PartialEq,
    op_eq,
    eq_f64,
    f64,
    bool;
    f32 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    f64 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
);
impl_compare_matrix!(
    PartialEq,
    op_eq,
    eq_f64,
    f64,
    bool;
    f32 => [f32, f64],
    f64 => [f32, f64]
);

#[cfg(test)]
mod test {
    use crate::ops::PartialEq;
    use rbs::{value, Value};

    #[test]
    fn test_eq() {
        let i: i64 = 1;
        let v = value!(1);
        assert!(v.op_eq(&i));
        assert!((&&v).op_eq(&&Value::I32(1)));
    }

    #[test]
    fn test_eq_cross_type() {
        assert!(1u64.op_eq(&Value::U64(1)));
        assert!(1i32.op_eq(&1u64));
        assert!("1".op_eq(&Value::String("1".to_string())));
        assert!(1isize.op_eq(&1u8));
        assert!(1.0f32.op_eq(&1i64));
        assert!(true.op_eq(&true));
        assert!(!u64::MAX.op_eq(&-1i64));
    }
}
