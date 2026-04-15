use crate::ops::AsProxy;
use crate::ops::BitShl;
use rbs::Value;

fn op_shl_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(s) => Value::I32(s << rhs.i32()),
        Value::I64(s) => Value::I64(s << rhs.i64()),
        Value::U32(s) => Value::U32(s << rhs.u32()),
        Value::U64(s) => Value::U64(s << rhs.u64()),
        Value::Ext(_, e) => op_shl_value(*e, rhs),
        _ => Value::Null,
    }
}

impl BitShl<Value> for Value {
    type Output = Value;
    fn op_shl(self, rhs: Value) -> Self::Output {
        op_shl_value(self, rhs)
    }
}

impl BitShl<&Value> for Value {
    type Output = Value;
    fn op_shl(self, rhs: &Value) -> Self::Output {
        op_shl_value(self, rhs.to_owned())
    }
}

impl BitShl<&&Value> for Value {
    type Output = Value;
    fn op_shl(self, rhs: &&Value) -> Self::Output {
        op_shl_value(self, (*rhs).to_owned())
    }
}

impl BitShl<Value> for &Value {
    type Output = Value;
    fn op_shl(self, rhs: Value) -> Self::Output {
        op_shl_value(self.to_owned(), rhs)
    }
}

impl BitShl<&Value> for &Value {
    type Output = Value;
    fn op_shl(self, rhs: &Value) -> Self::Output {
        op_shl_value(self.to_owned(), rhs.to_owned())
    }
}

impl BitShl<&&Value> for &Value {
    type Output = Value;
    fn op_shl(self, rhs: &&Value) -> Self::Output {
        op_shl_value(self.to_owned(), (*rhs).to_owned())
    }
}

fn op_shl_u64(value: &Value, other: u64) -> u64 {
    value.u64() << other
}

fn op_shl_i64(value: &Value, other: i64) -> i64 {
    value.i64() << other
}

fn op_shl_u64_value(value: &Value, other: u64) -> u64 {
    other << value.u64()
}

fn op_shl_i64_value(value: &Value, other: i64) -> i64 {
    other << value.i64()
}

macro_rules! impl_numeric_shl {
    ($($shl:ident,$shl_value:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl BitShl<$ty> for Value {
                type Output = $return_ty;
                fn op_shl(self, other: $ty) -> Self::Output {
                    $shl(&self, other as _)
                }
            }

            impl BitShl<&$ty> for Value {
                type Output = $return_ty;
                fn op_shl(self, other: &$ty) -> Self::Output {
                    $shl(&self, *other as _)
                }
            }

            impl<'a> BitShl<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_shl(self, other: $ty) -> Self::Output {
                    $shl(self, other as _)
                }
            }

            impl<'a> BitShl<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_shl(self, other: &$ty) -> Self::Output {
                    $shl(self, *other as _)
                }
            }

            impl BitShl<Value> for $ty {
                type Output = $return_ty;
                fn op_shl(self, other: Value) -> Self::Output {
                    $shl_value(&other, self as _)
                }
            }

            impl BitShl<&Value> for $ty {
                type Output = $return_ty;
                fn op_shl(self, other: &Value) -> Self::Output {
                    $shl_value(other, self as _)
                }
            }

            impl BitShl<Value> for &$ty {
                type Output = $return_ty;
                fn op_shl(self, other: Value) -> Self::Output {
                    $shl_value(&other, *self as _)
                }
            }

            impl BitShl<&Value> for &$ty {
                type Output = $return_ty;
                fn op_shl(self, other: &Value) -> Self::Output {
                    $shl_value(other, *self as _)
                }
            }

            impl BitShl<&&Value> for $ty {
                type Output = $return_ty;
                fn op_shl(self, other: &&Value) -> Self::Output {
                    $shl_value(*other, self as _)
                }
            }
        )*)*
    }
}

impl_numeric_shl! {
    op_shl_u64,op_shl_u64_value[u8 u16 u32 u64] -> u64
    op_shl_i64,op_shl_i64_value[i8 i16 i32 i64 isize usize] -> i64
}

macro_rules! self_shl {
    ([$($ty:ty)*]) => {
        $(
impl BitShl<$ty> for $ty{
      type Output = $ty;
      fn op_shl(self, rhs: $ty) -> Self::Output {
        self << rhs
      }
}
impl BitShl<&$ty> for $ty{
      type Output = $ty;
      fn op_shl(self, rhs: &$ty) -> Self::Output {
        self << *rhs
      }
}
impl BitShl<$ty> for &$ty{
      type Output = $ty;
      fn op_shl(self, rhs: $ty) -> Self::Output {
        *self << rhs
      }
}
impl BitShl<&$ty> for &$ty{
      type Output = $ty;
      fn op_shl(self, rhs: &$ty) -> Self::Output {
        *self << *rhs
      }
}
        )*
    };
}

self_shl!([u8 u16 u32 u64]);
self_shl!([i8 i16 i32 i64 isize usize]);
