use crate::ops::AsProxy;
use crate::ops::Sub;
use rbs::Value;

fn op_sub_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(s) => Value::I32(s - rhs.i32()),
        Value::I64(s) => Value::I64(s - rhs.i64()),
        Value::U32(s) => Value::U32(s - rhs.u32()),
        Value::U64(s) => Value::U64(s - rhs.u64()),
        Value::F32(s) => Value::F32(s - rhs.f64() as f32),
        Value::F64(s) => Value::F64(s - rhs.f64()),
        Value::Ext(_, e) => op_sub_value(*e, rhs),
        _ => Value::Null,
    }
}

impl Sub<Value> for Value {
    type Output = Value;
    fn op_sub(self, rhs: Value) -> Self::Output {
        op_sub_value(self, rhs)
    }
}

//value
impl Sub<&Value> for Value {
    type Output = Value;
    fn op_sub(self, rhs: &Value) -> Self::Output {
        op_sub_value(self, rhs.to_owned())
    }
}

impl Sub<&&Value> for Value {
    type Output = Value;
    fn op_sub(self, rhs: &&Value) -> Self::Output {
        op_sub_value(self, (*rhs).to_owned())
    }
}

impl Sub<Value> for &Value {
    type Output = Value;
    fn op_sub(self, rhs: Value) -> Self::Output {
        op_sub_value(self.to_owned(), rhs)
    }
}

impl Sub<&Value> for &Value {
    type Output = Value;
    fn op_sub(self, rhs: &Value) -> Self::Output {
        op_sub_value(self.to_owned(), rhs.to_owned())
    }
}

impl Sub<&&Value> for &Value {
    type Output = Value;
    fn op_sub(self, rhs: &&Value) -> Self::Output {
        op_sub_value(self.to_owned(), (*rhs).to_owned())
    }
}

fn op_sub_u64(value: &Value, other: u64) -> u64 {
    (value.u64() - other) as u64
}

fn op_sub_i64(value: &Value, other: i64) -> i64 {
    value.i64() - other
}

fn op_sub_f64(value: &Value, other: f64) -> f64 {
    value.f64() - other
}

fn op_sub_u64_value(value: &Value, other: u64) -> u64 {
    (other - value.u64()) as u64
}

fn op_sub_i64_value(value: &Value, other: i64) -> i64 {
    other - value.i64()
}

fn op_sub_f64_value(value: &Value, other: f64) -> f64 {
    other - value.f64()
}

macro_rules! impl_numeric_sub {
    ($($sub:ident,$sub_value:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Sub<$ty> for Value {
                type Output = $return_ty;
                fn op_sub(self, other: $ty) -> Self::Output {
                    $sub(&self, other as _)
                }
            }

            impl Sub<&$ty> for Value {
                type Output = $return_ty;
                fn op_sub(self, other: &$ty) -> Self::Output {
                    $sub(&self, *other as _)
                }
            }

            impl<'a> Sub<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_sub(self, other: $ty) -> Self::Output {
                    $sub(self, other as _)
                }
            }

            impl<'a> Sub<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_sub(self, other: &$ty) -> Self::Output {
                    $sub(self, *other as _)
                }
            }

            impl Sub<Value> for $ty {
                type Output = $return_ty;
                fn op_sub(self, other: Value) -> Self::Output {
                    $sub_value(&other, self as _)
                }
            }

            impl Sub<&Value> for $ty {
                type Output = $return_ty;
                fn op_sub(self, other: &Value) -> Self::Output {
                    $sub_value(other, self as _)
                }
            }

            impl Sub<Value> for &$ty {
                type Output = $return_ty;
                fn op_sub(self, other: Value) -> Self::Output {
                    $sub_value(&other, *self as _)
                }
            }

            impl Sub<&Value> for &$ty {
                type Output = $return_ty;
                fn op_sub(self, other: &Value) -> Self::Output {
                    $sub_value(other, *self as _)
                }
            }
            // for unary
            impl Sub<&&Value> for $ty {
                type Output = $return_ty;
                fn op_sub(self, other: &&Value) -> Self::Output {
                    $sub_value(*other, self as _)
                }
            }
        )*)*
    }
}

impl_numeric_sub! {
    op_sub_u64,op_sub_u64_value[u8 u16 u32 u64] -> u64
    op_sub_i64,op_sub_i64_value[i8 i16 i32 i64 isize usize] -> i64
    op_sub_f64,op_sub_f64_value[f32 f64] -> f64
}

macro_rules! self_sub {
    ([$($ty:ty)*]) => {
        $(
impl Sub<$ty> for $ty{
      type Output = $ty;
      fn op_sub(self, rhs: $ty) -> Self::Output {
        self-rhs
      }
}
impl Sub<&$ty> for $ty{
      type Output = $ty;
      fn op_sub(self, rhs: &$ty) -> Self::Output {
        self-*rhs
      }
}
impl Sub<$ty> for &$ty{
      type Output = $ty;
      fn op_sub(self, rhs: $ty) -> Self::Output {
        *self-rhs
      }
}
impl Sub<&$ty> for &$ty{
      type Output = $ty;
      fn op_sub(self, rhs: &$ty) -> Self::Output {
        *self-*rhs
      }
}
        )*
    };
}

self_sub!([u8 u16 u32 u64]);
self_sub!([i8 i16 i32 i64 isize usize]);
self_sub!([f32 f64]);

#[cfg(test)]
mod test {
    use crate::ops::Sub;
    use rbs::Value;

    #[test]
    fn test_sub() {
        let v = 0.op_sub(&&Value::I32(-1));
        println!("{}", v);
        assert_eq!(v, 1i64);
    }
}
