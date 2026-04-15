use crate::ops::AsProxy;
use crate::ops::BitShr;
use rbs::Value;

fn op_shr_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(s) => Value::I32(s >> rhs.i32()),
        Value::I64(s) => Value::I64(s >> rhs.i64()),
        Value::U32(s) => Value::U32(s >> rhs.u32()),
        Value::U64(s) => Value::U64(s >> rhs.u64()),
        Value::Ext(_, e) => op_shr_value(*e, rhs),
        _ => Value::Null,
    }
}

impl BitShr<Value> for Value {
    type Output = Value;
    fn op_shr(self, rhs: Value) -> Self::Output {
        op_shr_value(self, rhs)
    }
}

impl BitShr<&Value> for Value {
    type Output = Value;
    fn op_shr(self, rhs: &Value) -> Self::Output {
        op_shr_value(self, rhs.to_owned())
    }
}

impl BitShr<&&Value> for Value {
    type Output = Value;
    fn op_shr(self, rhs: &&Value) -> Self::Output {
        op_shr_value(self, (*rhs).to_owned())
    }
}

impl BitShr<Value> for &Value {
    type Output = Value;
    fn op_shr(self, rhs: Value) -> Self::Output {
        op_shr_value(self.to_owned(), rhs)
    }
}

impl BitShr<&Value> for &Value {
    type Output = Value;
    fn op_shr(self, rhs: &Value) -> Self::Output {
        op_shr_value(self.to_owned(), rhs.to_owned())
    }
}

impl BitShr<&&Value> for &Value {
    type Output = Value;
    fn op_shr(self, rhs: &&Value) -> Self::Output {
        op_shr_value(self.to_owned(), (*rhs).to_owned())
    }
}

fn op_shr_u64(value: &Value, other: u64) -> u64 {
    value.u64() >> other
}

fn op_shr_i64(value: &Value, other: i64) -> i64 {
    value.i64() >> other
}

fn op_shr_u64_value(value: &Value, other: u64) -> u64 {
    other >> value.u64()
}

fn op_shr_i64_value(value: &Value, other: i64) -> i64 {
    other >> value.i64()
}

macro_rules! impl_numeric_shr {
    ($($shr:ident,$shr_value:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl BitShr<$ty> for Value {
                type Output = $return_ty;
                fn op_shr(self, other: $ty) -> Self::Output {
                    $shr(&self, other as _)
                }
            }

            impl BitShr<&$ty> for Value {
                type Output = $return_ty;
                fn op_shr(self, other: &$ty) -> Self::Output {
                    $shr(&self, *other as _)
                }
            }

            impl<'a> BitShr<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_shr(self, other: $ty) -> Self::Output {
                    $shr(self, other as _)
                }
            }

            impl<'a> BitShr<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_shr(self, other: &$ty) -> Self::Output {
                    $shr(self, *other as _)
                }
            }

            impl BitShr<Value> for $ty {
                type Output = $return_ty;
                fn op_shr(self, other: Value) -> Self::Output {
                    $shr_value(&other, self as _)
                }
            }

            impl BitShr<&Value> for $ty {
                type Output = $return_ty;
                fn op_shr(self, other: &Value) -> Self::Output {
                    $shr_value(other, self as _)
                }
            }

            impl BitShr<Value> for &$ty {
                type Output = $return_ty;
                fn op_shr(self, other: Value) -> Self::Output {
                    $shr_value(&other, *self as _)
                }
            }

            impl BitShr<&Value> for &$ty {
                type Output = $return_ty;
                fn op_shr(self, other: &Value) -> Self::Output {
                    $shr_value(other, *self as _)
                }
            }

            impl BitShr<&&Value> for $ty {
                type Output = $return_ty;
                fn op_shr(self, other: &&Value) -> Self::Output {
                    $shr_value(*other, self as _)
                }
            }
        )*)*
    }
}

impl_numeric_shr! {
    op_shr_u64,op_shr_u64_value[u8 u16 u32 u64] -> u64
    op_shr_i64,op_shr_i64_value[i8 i16 i32 i64 isize usize] -> i64
}

macro_rules! self_shr {
    ([$($ty:ty)*]) => {
        $(
impl BitShr<$ty> for $ty{
      type Output = $ty;
      fn op_shr(self, rhs: $ty) -> Self::Output {
        self >> rhs
      }
}
impl BitShr<&$ty> for $ty{
      type Output = $ty;
      fn op_shr(self, rhs: &$ty) -> Self::Output {
        self >> *rhs
      }
}
impl BitShr<$ty> for &$ty{
      type Output = $ty;
      fn op_shr(self, rhs: $ty) -> Self::Output {
        *self >> rhs
      }
}
impl BitShr<&$ty> for &$ty{
      type Output = $ty;
      fn op_shr(self, rhs: &$ty) -> Self::Output {
        *self >> *rhs
      }
}
        )*
    };
}

self_shr!([u8 u16 u32 u64]);
self_shr!([i8 i16 i32 i64 isize usize]);
