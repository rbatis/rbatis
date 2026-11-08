use crate::ops::{AsProxy, PartialOrd};
use crate::ops_compare::{
    value_cmp_bool, value_cmp_bool_reverse, value_cmp_f64, value_cmp_f64_reverse, value_cmp_i64,
    value_cmp_i64_reverse, value_cmp_u64, value_cmp_u64_reverse, value_partial_cmp,
};
use crate::ops_macros::{impl_compare_matrix, impl_value_scalar_compare_op};
use rbs::Value;
use std::cmp::{Ordering, PartialOrd as StdPartialOrd};

#[inline]
fn cmp_i128(value: i128, rhs: i128) -> Option<Ordering> {
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

#[inline]
fn cmp_str(value: &str, rhs: &str) -> Option<Ordering> {
    Some(value.cmp(rhs))
}

impl PartialOrd<Value> for Value {
    fn op_partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
        value_partial_cmp(self, rhs)
    }
}

impl PartialOrd<&Value> for Value {
    fn op_partial_cmp(&self, rhs: &&Value) -> Option<Ordering> {
        value_partial_cmp(self, rhs)
    }
}

impl PartialOrd<&&Value> for Value {
    fn op_partial_cmp(&self, rhs: &&&Value) -> Option<Ordering> {
        value_partial_cmp(self, rhs)
    }
}

impl PartialOrd<Value> for &Value {
    fn op_partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
        value_partial_cmp(self, rhs)
    }
}

impl PartialOrd<&Value> for &Value {
    fn op_partial_cmp(&self, rhs: &&Value) -> Option<Ordering> {
        value_partial_cmp(self, rhs)
    }
}

impl PartialOrd<&&Value> for &Value {
    fn op_partial_cmp(&self, rhs: &&&Value) -> Option<Ordering> {
        value_partial_cmp(self, rhs)
    }
}

impl PartialOrd<Value> for &&Value {
    fn op_partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
        value_partial_cmp(self, rhs)
    }
}

impl PartialOrd<&Value> for &&Value {
    fn op_partial_cmp(&self, rhs: &&Value) -> Option<Ordering> {
        value_partial_cmp(self, rhs)
    }
}

impl PartialOrd<&&Value> for &&Value {
    fn op_partial_cmp(&self, rhs: &&&Value) -> Option<Ordering> {
        value_partial_cmp(self, rhs)
    }
}

impl_value_scalar_compare_op!(
    PartialOrd,
    op_partial_cmp,
    value_cmp_u64,
    value_cmp_u64_reverse,
    [u8, u16, u32, u64],
    Option<Ordering>
);
impl_value_scalar_compare_op!(
    PartialOrd,
    op_partial_cmp,
    value_cmp_i64,
    value_cmp_i64_reverse,
    [i8, i16, i32, i64, isize, usize],
    Option<Ordering>
);
impl_value_scalar_compare_op!(
    PartialOrd,
    op_partial_cmp,
    value_cmp_f64,
    value_cmp_f64_reverse,
    [f32, f64],
    Option<Ordering>
);
impl_value_scalar_compare_op!(
    PartialOrd,
    op_partial_cmp,
    value_cmp_bool,
    value_cmp_bool_reverse,
    [bool],
    Option<Ordering>
);

macro_rules! impl_same_type_cmp {
    ($compare:ident [$($ty:ty)*]) => {
        $(
            impl PartialOrd<$ty> for $ty {
                fn op_partial_cmp(&self, rhs: &$ty) -> Option<Ordering> {
                    $compare(*self as _, *rhs as _)
                }
            }

            impl PartialOrd<&$ty> for $ty {
                fn op_partial_cmp(&self, rhs: &&$ty) -> Option<Ordering> {
                    $compare(*self as _, **rhs as _)
                }
            }

            impl PartialOrd<$ty> for &$ty {
                fn op_partial_cmp(&self, rhs: &$ty) -> Option<Ordering> {
                    $compare(**self as _, *rhs as _)
                }
            }

            impl PartialOrd<&$ty> for &$ty {
                fn op_partial_cmp(&self, rhs: &&$ty) -> Option<Ordering> {
                    $compare(**self as _, **rhs as _)
                }
            }
        )*
    };
}

impl_same_type_cmp! {
    cmp_bool [bool]
}

impl PartialOrd<&str> for &str {
    fn op_partial_cmp(&self, rhs: &&str) -> Option<Ordering> {
        (*self).partial_cmp(*rhs)
    }
}

impl PartialOrd<&str> for String {
    fn op_partial_cmp(&self, rhs: &&str) -> Option<Ordering> {
        self.as_str().partial_cmp(*rhs)
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
        self.as_str().partial_cmp(*rhs)
    }
}

impl PartialOrd<&&str> for &String {
    fn op_partial_cmp(&self, rhs: &&&str) -> Option<Ordering> {
        self.as_str().partial_cmp(**rhs)
    }
}

impl PartialOrd<&&&str> for &String {
    fn op_partial_cmp(&self, rhs: &&&&str) -> Option<Ordering> {
        self.as_str().partial_cmp(***rhs)
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

impl PartialOrd<&str> for Value {
    fn op_partial_cmp(&self, rhs: &&str) -> Option<Ordering> {
        cmp_str(self.string().as_str(), rhs)
    }
}

impl PartialOrd<&&str> for Value {
    fn op_partial_cmp(&self, rhs: &&&str) -> Option<Ordering> {
        cmp_str(self.string().as_str(), rhs)
    }
}

impl PartialOrd<&str> for &Value {
    fn op_partial_cmp(&self, rhs: &&str) -> Option<Ordering> {
        cmp_str(self.string().as_str(), rhs)
    }
}

impl PartialOrd<&&str> for &Value {
    fn op_partial_cmp(&self, rhs: &&&str) -> Option<Ordering> {
        cmp_str(self.string().as_str(), rhs)
    }
}

impl PartialOrd<&str> for &&Value {
    fn op_partial_cmp(&self, rhs: &&str) -> Option<Ordering> {
        cmp_str(self.string().as_str(), rhs)
    }
}

impl PartialOrd<&&str> for &&Value {
    fn op_partial_cmp(&self, rhs: &&&str) -> Option<Ordering> {
        cmp_str(self.string().as_str(), rhs)
    }
}

impl PartialOrd<Value> for &str {
    fn op_partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
        cmp_str(self, rhs.string().as_str())
    }
}

impl PartialOrd<&Value> for &str {
    fn op_partial_cmp(&self, rhs: &&Value) -> Option<Ordering> {
        cmp_str(self, rhs.string().as_str())
    }
}

impl PartialOrd<&&Value> for &str {
    fn op_partial_cmp(&self, rhs: &&&Value) -> Option<Ordering> {
        cmp_str(self, rhs.string().as_str())
    }
}

impl PartialOrd<Value> for &&str {
    fn op_partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
        cmp_str(self, rhs.string().as_str())
    }
}

impl PartialOrd<&Value> for &&str {
    fn op_partial_cmp(&self, rhs: &&Value) -> Option<Ordering> {
        cmp_str(self, rhs.string().as_str())
    }
}

impl PartialOrd<&&Value> for &&str {
    fn op_partial_cmp(&self, rhs: &&&Value) -> Option<Ordering> {
        cmp_str(self, rhs.string().as_str())
    }
}

impl_compare_matrix!(
    PartialOrd,
    op_partial_cmp,
    cmp_i128,
    i128,
    Option<Ordering>;
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
    PartialOrd,
    op_partial_cmp,
    cmp_f64,
    f64,
    Option<Ordering>;
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
    PartialOrd,
    op_partial_cmp,
    cmp_f64,
    f64,
    Option<Ordering>;
    f32 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
    f64 => [u8, u16, u32, u64, usize, i8, i16, i32, i64, isize],
);
impl_compare_matrix!(
    PartialOrd,
    op_partial_cmp,
    cmp_f64,
    f64,
    Option<Ordering>;
    f32 => [f32, f64],
    f64 => [f32, f64]
);

#[cfg(test)]
mod test {
    use crate::ops::PartialOrd;
    use rbs::Value;
    use std::cmp::Ordering;

    #[test]
    fn test_cmp() {
        assert_eq!(
            Value::I32(2).op_partial_cmp(&Value::I32(1)),
            Some(Ordering::Greater)
        );
        assert_eq!(
            (&&Value::I32(2)).op_partial_cmp(&Value::I32(1)),
            Some(Ordering::Greater)
        );
        assert_eq!(1isize.op_partial_cmp(&1u8), Some(Ordering::Equal));
        assert_eq!(1.0f32.op_partial_cmp(&1i64), Some(Ordering::Equal));
        assert_eq!(true.op_partial_cmp(&false), Some(Ordering::Greater));
    }
}
