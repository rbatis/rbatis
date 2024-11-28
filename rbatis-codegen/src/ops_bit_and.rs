use crate::ops::BitAnd;
use rbs::Value;

impl BitAnd for Value {
    type Output = bool;

    fn op_bitand(self, rhs: Self) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<Value> for bool {
    type Output = bool;

    fn op_bitand(self, rhs: Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}

//ref value
impl BitAnd<Value> for &Value {
    type Output = bool;

    fn op_bitand(self, rhs: Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&Value> for &Value {
    type Output = bool;

    fn op_bitand(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&&Value> for &Value {
    type Output = bool;

    fn op_bitand(self, rhs: &&Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<bool> for &Value {
    type Output = bool;

    fn op_bitand(self, rhs: bool) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs
    }
}

//rhs ref
impl BitAnd<&Value> for Value {
    type Output = bool;

    fn op_bitand(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&Value> for bool {
    type Output = bool;

    fn op_bitand(self, rhs: &Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&&Value> for Value {
    type Output = bool;

    fn op_bitand(self, rhs: &&Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&&Value> for bool {
    type Output = bool;

    fn op_bitand(self, rhs: &&Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}

fn op_bit_and_u64(v: &Value, other: u64) -> u64 {
    std::ops::BitAnd::bitand(v.as_u64().unwrap_or_default(), other)
}

fn op_bit_and_i64(v: &Value, other: i64) -> i64 {
    std::ops::BitAnd::bitand(v.as_i64().unwrap_or_default(), other)
}

macro_rules! impl_numeric_bitand {
    ($($eq:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl BitAnd<$ty> for Value {
                type Output = $return_ty;
                fn op_bitand(self, other: $ty) -> Self::Output {
                    $eq(&self, other as _)
                }
            }

            impl BitAnd<&$ty> for Value {
                type Output = $return_ty;
                fn op_bitand(self, other: &$ty) -> Self::Output {
                    $eq(&self, *other as _)
                }
            }

            impl<'a> BitAnd<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_bitand(self, other: $ty) -> Self::Output {
                    $eq(self, other as _)
                }
            }

            impl<'a> BitAnd<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_bitand(self, other: &$ty) -> Self::Output {
                    $eq(self, *other as _)
                }
            }

            impl BitAnd<Value> for $ty {
                type Output = $return_ty;
                fn op_bitand(self, other: Value) -> Self::Output {
                    $eq(&other, self as _)
                }
            }

            impl BitAnd<&Value> for $ty {
                type Output = $return_ty;
                fn op_bitand(self, other: &Value) -> Self::Output {
                    $eq(other, self as _)
                }
            }

            impl BitAnd<Value> for &$ty {
                type Output = $return_ty;
                fn op_bitand(self, other: Value) -> Self::Output {
                    $eq(&other, *self as _)
                }
            }

            impl BitAnd<&Value> for &$ty {
                type Output = $return_ty;
                fn op_bitand(self, other: &Value) -> Self::Output {
                    $eq(other, *self as _)
                }
            }

            // for unary
            impl BitAnd<&&Value> for $ty {
                type Output = $return_ty;
                fn op_bitand(self, other: &&Value) -> Self::Output {
                    $eq(other, self as _)
                }
            }
        )*)*
    }
}

impl_numeric_bitand! {
    op_bit_and_u64[u8 u16 u32 u64] -> u64
    op_bit_and_i64[i8 i16 i32 i64 isize usize] -> i64
}

macro_rules! self_bitand {
    ([$($ty:ty)*]) => {
        $(
impl BitAnd<$ty> for $ty{
      type Output = $ty;
      fn op_bitand(self, rhs: $ty) -> Self::Output {
        self & rhs
      }
}
impl BitAnd<&$ty> for $ty{
      type Output = $ty;
      fn op_bitand(self, rhs: &$ty) -> Self::Output {
        self  &  *rhs
      }
}
impl BitAnd<$ty> for &$ty{
      type Output = $ty;
      fn op_bitand(self, rhs: $ty) -> Self::Output {
        *self  &  rhs
      }
}
impl BitAnd<&$ty> for &$ty{
      type Output = $ty;
      fn op_bitand(self, rhs: &$ty) -> Self::Output {
        *self & *rhs
      }
}
        )*
    };
}
self_bitand!([u8 u16 u32 u64]);
self_bitand!([i8 i16 i32 i64 isize usize]);
