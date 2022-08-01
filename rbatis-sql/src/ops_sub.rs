use rbs::Value;
use crate::ops::Sub;
use crate::ops::AsProxy;


//value
impl Sub<&Value> for Value {
    type Output = Value;
    fn op_sub(self, rhs: &Value) -> Self::Output {
        return match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                return Value::I64((s - rhs) as i64);
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                return Value::I64(s - rhs);
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                return Value::U64(((s - rhs) as u64) );
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                return Value::U64((s - rhs));
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::F64(s - rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Sub<&&Value> for Value {
    type Output = Value;
    fn op_sub(self, rhs: &&Value) -> Self::Output {
        return match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                return Value::I64((s - rhs) as i64);
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                return Value::I64(s - rhs);
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                return Value::U64(((s - rhs) as u64) );
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                return Value::U64((s - rhs));
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::F64(s - rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Sub<Value> for Value {
    type Output = Value;
    fn op_sub(self, rhs: Value) -> Self::Output {
        return match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                return Value::I64((s - rhs) as i64);
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                return Value::I64(s - rhs);
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                return Value::U64(((s - rhs) as u64) );
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                return Value::U64((s - rhs));
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::F64(s - rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Sub<&Value> for &Value {
    type Output = Value;
    fn op_sub(self, rhs: &Value) -> Self::Output {
        return match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                return Value::I64((s - rhs) as i64);
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                return Value::I64(s - rhs);
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                return Value::U64(((s - rhs) as u64) );
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                return Value::U64((s - rhs));
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::F64(s - rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Sub<&&Value> for &Value {
    type Output = Value;
    fn op_sub(self, rhs: &&Value) -> Self::Output {
        return match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                return Value::I64((s - rhs) as i64);
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                return Value::I64(s - rhs);
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                return Value::U64(((s - rhs) as u64) );
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                return Value::U64((s - rhs));
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::F64(s - rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Sub<Value> for &Value {
    type Output = Value;
    fn op_sub(self, rhs: Value) -> Self::Output {
        return match self {
            Value::I32(s) => {
                let rhs = rhs.i32();
                return Value::I64((s - rhs) as i64);
            }
            Value::I64(s) => {
                let rhs = rhs.i64();
                return Value::I64(s - rhs);
            }
            Value::U32(s) => {
                let rhs = rhs.u32();
                return Value::U64(((s - rhs) as u64) );
            }
            Value::U64(s) => {
                let rhs = rhs.u64();
                return Value::U64((s - rhs));
            }
            Value::F64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::F64(s - rhs);
            }
            _ => {
                return Value::Null;
            }
        };
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
            impl Sub<&&Value> for $ty {
                type Output = $return_ty;
                fn op_sub(self, other: &&Value) -> Self::Output {
                    $sub_value(*other, self as _)
                }
            }

            impl<'a> Sub<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_sub(self, other: $ty) -> Self::Output {
                    $sub(self, other as _)
                }
            }
        )*)*
    }
}


impl_numeric_sub! {
    op_sub_u64,op_sub_u64_value[u8 u16 u32 u64] -> u64
    op_sub_i64,op_sub_i64_value[i8 i16 i32 i64 isize] -> i64
    op_sub_f64,op_sub_f64_value[f32 f64] -> f64
}



macro_rules! sub_self {
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

sub_self!([i8 i16 i32 i64 isize]);
sub_self!([f32 f64]);