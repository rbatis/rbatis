use crate::ops::AsProxy;
use crate::ops::Mul;
use rbs::Value;

impl Mul<&Value> for Value {
    type Output = Value;
    fn op_mul(self, rhs: &Value) -> Self::Output {
        match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                Value::I32(s * rhs)
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                Value::I64(s * rhs)
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                Value::U32(s * rhs)
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                Value::U64(s * rhs)
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                Value::F64(s * rhs)
            }
            _ => Value::Null,
        }
    }
}

impl Mul<&&Value> for Value {
    type Output = Value;
    fn op_mul(self, rhs: &&Value) -> Self::Output {
        match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                Value::I32(s * rhs)
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                Value::I64(s * rhs)
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                Value::U32(s * rhs)
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                Value::U64(s * rhs)
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                Value::F64(s * rhs)
            }
            _ => Value::Null,
        }
    }
}

impl Mul<Value> for Value {
    type Output = Value;
    fn op_mul(self, rhs: Value) -> Self::Output {
        match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                Value::I32(s * rhs)
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                Value::I64(s * rhs)
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                Value::U32(s * rhs)
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                Value::U64(s * rhs)
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                Value::F64(s * rhs)
            }
            _ => Value::Null,
        }
    }
}

impl Mul<&Value> for &Value {
    type Output = Value;
    fn op_mul(self, rhs: &Value) -> Self::Output {
        match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                Value::I32(s * rhs)
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                Value::I64(s * rhs)
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                Value::U32(s * rhs)
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                Value::U64(s * rhs)
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                Value::F64(s * rhs)
            }
            _ => Value::Null,
        }
    }
}

impl Mul<&&Value> for &Value {
    type Output = Value;
    fn op_mul(self, rhs: &&Value) -> Self::Output {
        match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                Value::I32(s * rhs)
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                Value::I64(s * rhs)
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                Value::U32(s * rhs)
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                Value::U64(s * rhs)
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                Value::F64(s * rhs)
            }
            _ => Value::Null,
        }
    }
}

impl Mul<Value> for &Value {
    type Output = Value;
    fn op_mul(self, rhs: Value) -> Self::Output {
        match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                Value::I32(s * rhs)
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                Value::I64(s * rhs)
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                Value::U32(s * rhs)
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                Value::U64(s * rhs)
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                Value::F64(s * rhs)
            }
            _ => Value::Null,
        }
    }
}

fn op_mul_u32(value: &Value, other: u32) -> u32 {
    value.u32() * other
}

fn op_mul_u64(value: &Value, other: u64) -> u64 {
    value.u64() * other
}

fn op_mul_i32(value: &Value, other: i32) -> i32 {
    value.i32() * other
}

fn op_mul_i64(value: &Value, other: i64) -> i64 {
    value.i64() * other
}

fn op_mul_f64(value: &Value, other: f64) -> f64 {
    value.f64() * other
}

macro_rules! impl_numeric_mul {
    ($($mul:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Mul<$ty> for Value {
                type Output = $return_ty;
                fn op_mul(self, other: $ty) -> Self::Output {
                    $mul(&self, other as _)
                }
            }

            impl Mul<Value> for $ty {
                type Output = $return_ty;
                fn op_mul(self, other: Value) -> Self::Output {
                    $mul(&other, self as _)
                }
            }

            impl Mul<&Value> for $ty {
                type Output = $return_ty;
                fn op_mul(self, other: &Value) -> Self::Output {
                    $mul(other, self as _)
                }
            }

            impl Mul<&&Value> for $ty {
                type Output = $return_ty;
                fn op_mul(self, other: &&Value) -> Self::Output {
                    $mul(*other, self as _)
                }
            }

            impl<'a> Mul<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_mul(self, other: $ty) -> Self::Output {
                    $mul(self, other as _)
                }
            }

            impl<'a> Mul<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_mul(self, other: &$ty) -> Self::Output {
                    $mul(self, *other as _)
                }
            }
        )*)*
    }
}

impl_numeric_mul! {
    op_mul_u64[u8 u16 u32 u64] -> u64
    op_mul_i64[i8 i16 i32 i64 isize] -> i64
    op_mul_f64[f32 f64] -> f64
}

macro_rules! mul_self {
    ([$($ty:ty)*]) => {
        $(
impl Mul<$ty> for $ty{
         type Output = $ty;
      fn op_mul(self, rhs: $ty) -> Self::Output {
        self * rhs
      }
    }
impl Mul<&$ty> for $ty{
         type Output = $ty;
      fn op_mul(self, rhs: &$ty) -> Self::Output {
        self * *rhs
      }
    }
impl Mul<$ty> for &$ty{
         type Output = $ty;
      fn op_mul(self, rhs: $ty) -> Self::Output {
        *self * rhs
      }
    }
impl Mul<&$ty> for &$ty{
         type Output = $ty;
      fn op_mul(self, rhs: &$ty) -> Self::Output {
        *self * *rhs
      }
    }
        )*
    };
}
mul_self!([u8 u16 u32 u64]);
mul_self!([i8 i16 i32 i64 isize]);
mul_self!([f32 f64]);
