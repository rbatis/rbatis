use rbson::Bson;
use crate::ops::Div;

use crate::ops::Value;
use crate::ops::AsProxy;

fn op_div_u64(value: &Value, other: u64) -> u64 {
    if other == 0 {
        return 0;
    }
    (value.u64() / other)
}

fn op_div_i64(value: &Value, other: i64) -> i64 {
    if other == 0 {
        return 0;
    }
    (value.i64() / other)
}


fn op_div_f64(value: &Value, other: f64) -> f64 {
    if other == 0.0 {
        return 0.0;
    }
    value.f64() / other
}


fn op_div_i64_value(value: &Value, other: i64) -> i64 {
    let v = value.i64();
    if v == 0 {
        return 0;
    }
    (other / v)
}

fn op_div_u64_value(value: &Value, other: u64) ->u64 {
    let v = value.u64();
    if v == 0 {
        return 0;
    }
    (other / v)
}


fn op_div_f64_value(value: &Value, other: f64) -> f64 {
    let v = value.f64();
    if v == 0.0 {
        return 0.0;
    }
    (other / v)
}

macro_rules! impl_numeric_div {
    ($($div:ident,$div_value:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Div<$ty> for Value {
                type Output = $return_ty;
                fn op_div(self, other: $ty) -> Self::Output {
                    $div(&self, other as _)
                }
            }

            impl Div<&$ty> for Value {
                type Output = $return_ty;
                fn op_div(self, other: &$ty) -> Self::Output {
                    $div(&self, *other as _)
                }
            }

            impl Div<&&$ty> for Value {
                type Output = $return_ty;
                fn op_div(self, other: &&$ty) -> Self::Output {
                    $div(&self, **other as _)
                }
            }

            impl Div<Value> for $ty {
                type Output = $return_ty;
                fn op_div(self, other: Value) -> Self::Output {
                    $div_value(&other, self as _)
                }
            }

            impl Div<&Value> for $ty {
                type Output = $return_ty;
                fn op_div(self, other: &Value) -> Self::Output {
                    $div_value(other, self as _)
                }
            }

            impl Div<&&Value> for $ty {
                type Output = $return_ty;
                fn op_div(self, other: &&Value) -> Self::Output {
                    $div_value(*other, self as _)
                }
            }

            impl<'a> Div<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_div(self, other: $ty) -> Self::Output {
                    $div(self, other as _)
                }
            }

            impl<'a> Div<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_div(self, other: &$ty) -> Self::Output {
                    $div(self, *other as _)
                }
            }
            impl<'a> Div<&&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_div(self, other: &&$ty) -> Self::Output {
                    $div(self, **other as _)
                }
            }
        )*)*
    }
}


impl_numeric_div! {
    op_div_u64,op_div_u64_value[u8 u16 u32 u64] -> u64
    op_div_i64,op_div_i64_value[i8 i16 i32 i64 isize] -> i64
    op_div_f64,op_div_f64_value[f32 f64] -> f64
}

//value

impl Div<&Value> for Value {
    type Output = Value;
    fn op_div(self, rhs: &Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int32(0);
                }
                return Value::Double(s as f64 / rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int64(0);
                }
                return Value::Double(s as f64 / rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt32(0);
                }
                return Value::Double(s as f64 / rhs as f64);
            }
            Value::UInt64(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt64(0);
                }
                return Value::Double(s as f64 / rhs as f64);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Double(0.0);
                }
                return Value::Double(s / rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Div<&&Value> for Value {
    type Output = Value;
    fn op_div(self, rhs: &&Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int32(0);
                }
                return Value::Double(s as f64 / rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int64(0);
                }
                return Value::Double(s as f64 / rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt32(0);
                }
                return Value::Double(s as f64 / rhs as f64);
            }
            Value::UInt64(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt64(0);
                }
                return Value::Double(s as f64 / rhs as f64);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Double(0.0);
                }
                return Value::Double(s / rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Div<Value> for Value {
    type Output = Value;
    fn op_div(self, rhs: Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int32(0);
                }
                return Value::Double(s as f64 / rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int64(0);
                }
                return Value::Double(s as f64 / rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt32(0);
                }
                return Value::Double(s as f64 / rhs as f64);
            }
            Value::UInt64(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt64(0);
                }
                return Value::Double(s as f64 / rhs as f64);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Double(0.0);
                }
                return Value::Double(s / rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Div<Value> for &Value {
    type Output = Value;
    fn op_div(self, rhs: Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int32(0);
                }
                return Value::Double(*s as f64 / rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int64(0);
                }
                return Value::Double(*s as f64 / rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt32(0);
                }
                return Value::Double(*s as f64 / rhs as f64);
            }
            Value::UInt64(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt64(0);
                }
                return Value::Double(*s as f64 / rhs as f64);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Double(0.0);
                }
                return Value::Double(s / rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Div<&Value> for &Value {
    type Output = Value;
    fn op_div(self, rhs: &Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int32(0);
                }
                return Value::Double(*s as f64 / rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int64(0);
                }
                return Value::Double(*s as f64 / rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt32(0);
                }
                return Value::Double(*s as f64 / rhs as f64);
            }
            Value::UInt64(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt64(0);
                }
                return Value::Double(*s as f64 / rhs as f64);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Double(0.0);
                }
                return Value::Double(s / rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Div<&&Value> for &Value {
    type Output = Value;
    fn op_div(self, rhs: &&Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int32(0);
                }
                return Value::Double(*s as f64 / rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Int64(0);
                }
                return Value::Double(*s as f64 / rhs);
            }
            Value::UInt32(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt32(0);
                }
                return Value::Double(*s as f64 / rhs as f64);
            }
            Value::UInt64(s) => {
                let rhs = rhs.as_u64().unwrap_or_default();
                if rhs == 0 {
                    return Value::UInt64(0);
                }
                return Value::Double(*s as f64 / rhs as f64);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                if rhs == 0.0 {
                    return Value::Double(0.0);
                }
                return Value::Double(*s as f64 / rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}


macro_rules! div_self {
    ([$($ty:ty)*]) => {
        $(
impl Div<$ty> for $ty{
         type Output = $ty;
      fn op_div(self, rhs: $ty) -> Self::Output {
        self / rhs
      }
    }
impl Div<&$ty> for $ty{
         type Output = $ty;
      fn op_div(self, rhs: &$ty) -> Self::Output {
        self / *rhs
      }
    }
impl Div<$ty> for &$ty{
         type Output = $ty;
      fn op_div(self, rhs: $ty) -> Self::Output {
        *self / rhs
      }
    }
impl Div<&$ty> for &$ty{
         type Output = $ty;
      fn op_div(self, rhs: &$ty) -> Self::Output {
        *self / *rhs
      }
    }
        )*
    };
}
div_self!([u8 u16 u32 u64]);
div_self!([i8 i16 i32 i64 isize]);
div_self!([f32 f64]);