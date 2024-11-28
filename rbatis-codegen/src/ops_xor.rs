use crate::ops::AsProxy;
use crate::ops::BitXor;
use rbs::Value;

fn op_bitxor_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(s) => {
            let rhs = rhs.i32();
            Value::I32(s ^ rhs)
        }
        Value::I64(s) => {
            let rhs = rhs.i64();
            Value::I64(s ^ rhs)
        }
        Value::U32(s) => {
            let rhs = rhs.u32();
            Value::U32(s ^ rhs)
        }
        Value::U64(s) => {
            let rhs = rhs.u64();
            Value::U64(s ^ rhs)
        }
        Value::Ext(_, e) => op_bitxor_value(*e, rhs),
        _ => Value::Null,
    }
}

impl BitXor<Value> for Value {
    type Output = Value;
    fn op_bitxor(self, rhs: Value) -> Self::Output {
        op_bitxor_value(self, rhs)
    }
}

//value
impl BitXor<&Value> for Value {
    type Output = Value;
    fn op_bitxor(self, rhs: &Value) -> Self::Output {
        op_bitxor_value(self, rhs.to_owned())
    }
}

impl BitXor<&&Value> for Value {
    type Output = Value;
    fn op_bitxor(self, rhs: &&Value) -> Self::Output {
        op_bitxor_value(self, (*rhs).to_owned())
    }
}

impl BitXor<Value> for &Value {
    type Output = Value;
    fn op_bitxor(self, rhs: Value) -> Self::Output {
        op_bitxor_value(self.to_owned(), rhs)
    }
}

impl BitXor<&Value> for &Value {
    type Output = Value;
    fn op_bitxor(self, rhs: &Value) -> Self::Output {
        op_bitxor_value(self.to_owned(), rhs.to_owned())
    }
}

impl BitXor<&&Value> for &Value {
    type Output = Value;
    fn op_bitxor(self, rhs: &&Value) -> Self::Output {
        op_bitxor_value(self.to_owned(), (*rhs).to_owned())
    }
}

fn op_bitxor_i64(value: &Value, other: i64) -> i64 {
    value.i64() ^ other
}

fn op_bitxor_f64(_: &Value, _: f64) -> f64 {
    0.0
}

fn op_bitxor_i64_value(value: &Value, other: i64) -> i64 {
    other ^ value.i64()
}

fn op_bitxor_f64_value(_: &Value, _: f64) -> f64 {
    0.0
}

macro_rules! impl_numeric_sub {
    ($($sub:ident,$sub_value:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl BitXor<$ty> for Value {
                type Output = $return_ty;
                fn op_bitxor(self, other: $ty) -> Self::Output {
                    $sub(&self, other as _)
                }
            }

            impl BitXor<&$ty> for Value {
                type Output = $return_ty;
                fn op_bitxor(self, other: &$ty) -> Self::Output {
                    $sub(&self, *other as _)
                }
            }

            impl<'a> BitXor<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_bitxor(self, other: $ty) -> Self::Output {
                    $sub(self, other as _)
                }
            }

            impl<'a> BitXor<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_bitxor(self, other: &$ty) -> Self::Output {
                    $sub(self, *other as _)
                }
            }

            impl BitXor<Value> for $ty {
                type Output = $return_ty;
                fn op_bitxor(self, other: Value) -> Self::Output {
                    $sub_value(&other, self as _)
                }
            }

            impl BitXor<&Value> for $ty {
                type Output = $return_ty;
                fn op_bitxor(self, other: &Value) -> Self::Output {
                    $sub_value(other, self as _)
                }
            }

            impl BitXor<Value> for &$ty {
                type Output = $return_ty;
                fn op_bitxor(self, other: Value) -> Self::Output {
                    $sub_value(&other, *self as _)
                }
            }

            impl BitXor<&Value> for &$ty {
                type Output = $return_ty;
                fn op_bitxor(self, other: &Value) -> Self::Output {
                    $sub_value(other, *self as _)
                }
            }
            // for unary
            impl BitXor<&&Value> for $ty {
                type Output = $return_ty;
                fn op_bitxor(self, other: &&Value) -> Self::Output {
                    $sub_value(other, self as _)
                }
            }
        )*)*
    }
}

impl_numeric_sub! {
    op_bitxor_i64,op_bitxor_i64_value[i8 i16 i32 i64 isize usize] -> i64
    op_bitxor_f64,op_bitxor_f64_value[f32 f64] -> f64
}

macro_rules! xor_self {
([$($ty:ty)*]) => {
$(
impl BitXor< $ ty> for $ ty{
  type Output = $ty;
  fn op_bitxor(self, rhs: $ty) -> Self::Output {
      self ^ rhs
  }
}
impl BitXor<& $ ty> for $ ty{
  type Output = $ty;
  fn op_bitxor(self, rhs: & $ ty) -> Self::Output {
    self ^ *rhs
  }
}
impl BitXor< $ ty> for & $ ty{
  type Output = $ty;
  fn op_bitxor(self, rhs: $ty) -> Self::Output {
    *self ^ rhs
  }
}
impl BitXor<& $ ty> for & $ ty{
  type Output = $ty;
  fn op_bitxor(self, rhs: & $ ty) -> Self::Output {
    *self ^ *rhs
  }
}
)*
};
}

xor_self!([u8 u16 u32 u64]);
xor_self!([i8 i16 i32 i64 isize usize]);
// unsupported! xor_self!([f32 f64]);
