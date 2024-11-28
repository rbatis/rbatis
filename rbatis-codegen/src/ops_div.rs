use crate::ops::AsProxy;
use crate::ops::Div;
use rbs::Value;

fn op_div_u64(value: &Value, other: u64) -> u64 {
    if other == 0 {
        return 0;
    }
    value.u64() / other
}

fn op_div_i64(value: &Value, other: i64) -> i64 {
    if other == 0 {
        return 0;
    }
    value.i64() / other
}

fn op_div_f64(value: &Value, other: f64) -> f64 {
    if other == 0.0 {
        0.0
    } else {
        value.f64() / other
    }
}

fn op_div_i64_value(value: &Value, other: i64) -> i64 {
    let v = value.i64();
    if v == 0 {
        return 0;
    }
    other / v
}

fn op_div_u64_value(value: &Value, other: u64) -> u64 {
    let v = value.u64();
    if v == 0 {
        return 0;
    }
    other / v
}

fn op_div_f64_value(value: &Value, other: f64) -> f64 {
    let v = value.f64();
    if v == 0.0 {
        0.0
    } else {
        other / v
    }
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

            impl Div<Value> for &$ty {
                type Output = $return_ty;
                fn op_div(self, other: Value) -> Self::Output {
                    $div_value(&other, *self as _)
                }
            }

            impl Div<&Value> for &$ty {
                type Output = $return_ty;
                fn op_div(self, other: &Value) -> Self::Output {
                    $div_value(other, *self as _)
                }
            }

            // for unary
            impl Div<&&Value> for $ty {
                type Output = $return_ty;
                fn op_div(self, other: &&Value) -> Self::Output {
                    $div_value(other, self as _)
                }
            }
        )*)*
    }
}

impl_numeric_div! {
    op_div_u64,op_div_u64_value[u8 u16 u32 u64] -> u64
    op_div_i64,op_div_i64_value[i8 i16 i32 i64 isize usize] -> i64
    op_div_f64,op_div_f64_value[f32 f64] -> f64
}

fn op_div_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(s) => {
            let rhs = rhs.f64();
            if rhs == 0.0 {
                return Value::I32(0);
            }
            Value::I32((s as f64 / rhs) as i32)
        }
        Value::I64(s) => {
            let rhs = rhs.f64();
            if rhs == 0.0 {
                return Value::I64(0);
            }
            Value::I64((s as f64 / rhs) as i64)
        }
        Value::U32(s) => {
            let rhs = rhs.u64();
            if rhs == 0 {
                return Value::U32(0);
            }
            Value::U32((s as f64 / rhs as f64) as u32)
        }
        Value::U64(s) => {
            let rhs = rhs.u64();
            if rhs == 0 {
                return Value::U64(0);
            }
            Value::U64((s as f64 / rhs as f64) as u64)
        }
        Value::F32(s) => {
            let rhs = rhs.f64() as f32;
            if rhs == 0.0 {
                return Value::F32(0.0);
            }
            Value::F32(s / rhs)
        }
        Value::F64(s) => {
            let rhs = rhs.f64();
            if rhs == 0.0 {
                return Value::F64(0.0);
            }
            Value::F64(s / rhs)
        }
        Value::Ext(_, e) => op_div_value(*e, rhs),
        _ => Value::Null,
    }
}

//value
impl Div<Value> for Value {
    type Output = Value;
    fn op_div(self, rhs: Value) -> Self::Output {
        op_div_value(self, rhs)
    }
}

impl Div<&Value> for Value {
    type Output = Value;
    fn op_div(self, rhs: &Value) -> Self::Output {
        op_div_value(self, rhs.to_owned())
    }
}

impl Div<&&Value> for Value {
    type Output = Value;
    fn op_div(self, rhs: &&Value) -> Self::Output {
        op_div_value(self, (*rhs).to_owned())
    }
}

impl Div<Value> for &Value {
    type Output = Value;
    fn op_div(self, rhs: Value) -> Self::Output {
        op_div_value(self.to_owned(), rhs.to_owned())
    }
}

impl Div<&Value> for &Value {
    type Output = Value;
    fn op_div(self, rhs: &Value) -> Self::Output {
        op_div_value(self.to_owned(), rhs.to_owned())
    }
}

impl Div<&&Value> for &Value {
    type Output = Value;
    fn op_div(self, rhs: &&Value) -> Self::Output {
        op_div_value(self.to_owned(), (*rhs).to_owned())
    }
}

impl Div<&&Value> for &&Value {
    type Output = Value;
    fn op_div(self, rhs: &&Value) -> Self::Output {
        op_div_value((*self).to_owned(), (*rhs).to_owned())
    }
}

macro_rules! self_div {
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
self_div!([u8 u16 u32 u64]);
self_div!([i8 i16 i32 i64 isize usize]);
self_div!([f32 f64]);
