use crate::ops::Value;
use crate::ops::BitXor;
use crate::ops::AsProxy;



//value
impl BitXor<&Value> for Value {
    type Output = Value;
    fn op_bitxor(self, rhs: &Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s ^ rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s ^ rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.u32();
                return Value::UInt32(s ^ rhs);
            }
            Value::UInt64(s) => {
                let rhs = rhs.u64();
                return Value::UInt64(s ^ rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Null;
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl BitXor<&&Value> for Value {
    type Output = Value;
    fn op_bitxor(self, rhs: &&Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s ^ rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s ^ rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.u32();
                return Value::UInt32(s ^ rhs);
            }
            Value::UInt64(s) => {
                let rhs = rhs.u64();
                return Value::UInt64(s ^ rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Null;
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl BitXor<Value> for Value {
    type Output = Value;
    fn op_bitxor(self, rhs: Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s ^ rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s ^ rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.u32();
                return Value::UInt32(s ^ rhs);
            }
            Value::UInt64(s) => {
                let rhs = rhs.u64();
                return Value::UInt64(s ^ rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Null;
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl BitXor<&Value> for &Value {
    type Output = Value;
    fn op_bitxor(self, rhs: &Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s ^ rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s ^ rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.u32();
                return Value::UInt32(s ^ rhs);
            }
            Value::UInt64(s) => {
                let rhs = rhs.u64();
                return Value::UInt64(s ^ rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Null;
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl BitXor<&&Value> for &Value {
    type Output = Value;
    fn op_bitxor(self, rhs: &&Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s ^ rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s ^ rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.u32();
                return Value::UInt32(s ^ rhs);
            }
            Value::UInt64(s) => {
                let rhs = rhs.u64();
                return Value::UInt64(s ^ rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Null;
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl BitXor<Value> for &Value {
    type Output = Value;
    fn op_bitxor(self, rhs: Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s ^ rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s ^ rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.u32();
                return Value::UInt32(s ^ rhs);
            }
            Value::UInt64(s) => {
                let rhs = rhs.u64();
                return Value::UInt64(s ^ rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Null;
            }
            _ => {
                return Value::Null;
            }
        };
    }
}


fn op_bitxor_u64(value: &Value, other: u64) -> u64 {
    value.u64() ^ other
}

fn op_bitxor_i64(value: &Value, other: i64) -> i64 {
    value.i64() ^ other
}

fn op_bitxor_f64(value: &Value, other: f64) -> f64 {
    0.0
}


fn op_bitxor_u64_value(value: &Value, other: u64) -> u64 {
    other ^ value.u64()
}

fn op_bitxor_i64_value(value: &Value, other: i64) -> i64 {
    other ^ value.i64()
}

fn op_bitxor_f64_value(value: &Value, other: f64) -> f64 {
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
            impl BitXor<&&Value> for $ty {
                type Output = $return_ty;
                fn op_bitxor(self, other: &&Value) -> Self::Output {
                    $sub_value(*other, self as _)
                }
            }

            impl<'a> BitXor<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_bitxor(self, other: $ty) -> Self::Output {
                    $sub(self, other as _)
                }
            }
        )*)*
    }
}


impl_numeric_sub! {
    op_bitxor_i64,op_bitxor_i64_value[i8 i16 i32 i64 isize] -> i64
    op_bitxor_f64,op_bitxor_f64_value[f32 f64] -> f64
}



macro_rules! xor_self {
    ([$($ty:ty)*]) => {
        $(
impl BitXor<$ty> for $ty{
         type Output = $ty;
      fn op_bitxor(self, rhs: $ty) -> Self::Output {
        self ^ rhs
      }
    }
impl BitXor<&$ty> for $ty{
         type Output = $ty;
      fn op_bitxor(self, rhs: &$ty) -> Self::Output {
        self ^ *rhs
      }
    }
impl BitXor<$ty> for &$ty{
         type Output = $ty;
      fn op_bitxor(self, rhs: $ty) -> Self::Output {
        *self ^ rhs
      }
    }
impl BitXor<&$ty> for &$ty{
         type Output = $ty;
      fn op_bitxor(self, rhs: &$ty) -> Self::Output {
        *self ^ *rhs
      }
    }
        )*
    };
}

xor_self!([i8 i16 i32 i64 isize]);
// unsupport! xor_self!([f32 f64]);