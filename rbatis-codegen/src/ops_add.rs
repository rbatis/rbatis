use crate::ops::{Add, AsProxy};
use rbs::Value;

fn op_add_u64(value: &Value, other: u64) -> u64 {
    value.u64() + other
}

fn op_add_i64(value: &Value, other: i64) -> i64 {
    value.i64() + other
}

fn op_add_f64(value: &Value, other: f64) -> f64 {
    value.f64() + other
}

macro_rules! impl_numeric_add {
    ($($eq:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Add<$ty> for Value {
                type Output = $return_ty;
                fn op_add(self, other: $ty) -> Self::Output {
                    $eq(&self, other as _)
                }
            }

            impl Add<&$ty> for Value {
                type Output = $return_ty;
                fn op_add(self, other: &$ty) -> Self::Output {
                    $eq(&self, *other as _)
                }
            }

            impl<'a> Add<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_add(self, other: $ty) -> Self::Output {
                    $eq(self, other as _)
                }
            }

            impl<'a> Add<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_add(self, other: &$ty) -> Self::Output {
                    $eq(self, *other as _)
                }
            }

            impl Add<Value> for $ty {
                type Output = $return_ty;
                fn op_add(self, other: Value) -> Self::Output {
                    $eq(&other, self as _)
                }
            }

            impl Add<&Value> for $ty {
                type Output = $return_ty;
                fn op_add(self, other: &Value) -> Self::Output {
                    $eq(other, self as _)
                }
            }

            impl Add<Value> for &$ty {
                type Output = $return_ty;
                fn op_add(self, other: Value) -> Self::Output {
                    $eq(&other, *self as _)
                }
            }

            impl Add<&Value> for &$ty {
                type Output = $return_ty;
                fn op_add(self, other: &Value) -> Self::Output {
                    $eq(other, *self as _)
                }
            }

            // for unary
            impl Add<&&Value> for $ty {
                type Output = $return_ty;
                fn op_add(self, other: &&Value) -> Self::Output {
                    $eq(other, self as _)
                }
            }
        )*)*
    }
}

impl_numeric_add! {
    op_add_u64[u8 u16 u32 u64] -> u64
    op_add_i64[i8 i16 i32 i64 isize usize] -> i64
    op_add_f64[f32 f64] -> f64
}

fn op_add_value(left: Value, rhs: &Value) -> Value {
    match left {
        Value::String(s) => Value::String(s + rhs.as_str().unwrap_or("")),
        Value::I32(s) => Value::I32(s + rhs.i32()),
        Value::I64(s) => Value::I64(s + rhs.i64()),
        Value::U32(s) => Value::U32(s + rhs.u32()),
        Value::U64(s) => Value::U64(s + rhs.u64()),
        Value::F64(s) => Value::F64(s + rhs.f64()),
        Value::F32(v) => Value::F64(v as f64 + rhs.f64()),
        Value::Ext(_, e) => op_add_value(*e, rhs),
        _ => Value::Null,
    }
}

//value
impl Add<&Value> for Value {
    type Output = Value;
    fn op_add(self, rhs: &Value) -> Self::Output {
        op_add_value(self, rhs)
    }
}

impl Add<&&Value> for Value {
    type Output = Value;
    fn op_add(self, rhs: &&Value) -> Self::Output {
        op_add_value(self, rhs)
    }
}

impl Add<Value> for Value {
    type Output = Value;
    fn op_add(self, rhs: Value) -> Self::Output {
        op_add_value(self, &rhs)
    }
}

impl Add<&Value> for &Value {
    type Output = Value;
    fn op_add(self, rhs: &Value) -> Self::Output {
        op_add_value(self.to_owned(), rhs)
    }
}

impl Add<&&Value> for &Value {
    type Output = Value;
    fn op_add(self, rhs: &&Value) -> Self::Output {
        op_add_value(self.to_owned(), rhs)
    }
}

impl Add<Value> for &Value {
    type Output = Value;
    fn op_add(self, rhs: Value) -> Self::Output {
        op_add_value(self.to_owned(), &rhs)
    }
}

//str
impl Add<Value> for &str {
    type Output = String;
    fn op_add(self, rhs: Value) -> Self::Output {
        self.to_string() + &rhs.string()
    }
}

impl Add<&Value> for &str {
    type Output = String;
    fn op_add(self, rhs: &Value) -> Self::Output {
        self.to_string() + rhs.clone().string().as_str()
    }
}

impl Add<&str> for Value {
    type Output = Value;
    fn op_add(self, rhs: &str) -> Self::Output {
        Value::String(self.string() + rhs)
    }
}

impl Add<&str> for &Value {
    type Output = Value;
    fn op_add(self, rhs: &str) -> Self::Output {
        Value::String(self.to_owned().string() + rhs)
    }
}

impl Add<&&str> for Value {
    type Output = Value;
    fn op_add(self, rhs: &&str) -> Self::Output {
        Value::String(self.string() + rhs)
    }
}

impl Add<&&str> for &Value {
    type Output = Value;
    fn op_add(self, rhs: &&str) -> Self::Output {
        Value::String(self.to_owned().string() + rhs)
    }
}

impl Add<String> for Value {
    type Output = Value;
    fn op_add(self, rhs: String) -> Self::Output {
        Value::String(self.string() + rhs.as_str())
    }
}

impl Add<String> for &Value {
    type Output = Value;
    fn op_add(self, rhs: String) -> Self::Output {
        Value::String(self.to_owned().string() + rhs.as_str())
    }
}

impl Add<Value> for &String {
    type Output = String;
    fn op_add(self, rhs: Value) -> Self::Output {
        self.to_string() + &rhs.string()
    }
}

impl Add<&Value> for &String {
    type Output = String;
    fn op_add(self, rhs: &Value) -> Self::Output {
        self.to_string() + &rhs.clone().string()
    }
}

impl Add<&String> for Value {
    type Output = Value;
    fn op_add(self, rhs: &String) -> Self::Output {
        Value::String(self.string() + rhs.as_str())
    }
}

impl Add<&String> for &Value {
    type Output = Value;
    fn op_add(self, rhs: &String) -> Self::Output {
        Value::String(self.to_owned().string() + rhs.as_str())
    }
}

impl Add<Value> for String {
    type Output = String;
    fn op_add(self, rhs: Value) -> Self::Output {
        self.to_string() + &rhs.string()
    }
}

impl Add<&Value> for String {
    type Output = String;
    fn op_add(self, rhs: &Value) -> Self::Output {
        self.to_string() + &rhs.clone().string()
    }
}

impl Add<&&Value> for String {
    type Output = String;

    fn op_add(self, rhs: &&Value) -> Self::Output {
        self + &rhs.string()
    }
}

macro_rules! self_add {
    ([$($ty:ty)*]) => {
        $(
    impl Add<$ty> for $ty{
         type Output = $ty;
      fn op_add(self, rhs: $ty) -> Self::Output {
        self+rhs
      }
    }
    impl Add<&$ty> for $ty{
         type Output = $ty;
      fn op_add(self, rhs: &$ty) -> Self::Output {
        self+*rhs
      }
    }
    impl Add<$ty> for &$ty{
         type Output = $ty;
      fn op_add(self, rhs: $ty) -> Self::Output {
        *self+rhs
      }
    }
    impl Add<&$ty> for &$ty{
         type Output = $ty;
      fn op_add(self, rhs: &$ty) -> Self::Output {
        *self+*rhs
      }
    }
        )*
    };
}
self_add!([u8 u16 u32 u64]);
self_add!([i8 i16 i32 i64 isize usize]);
self_add!([f32 f64]);

impl Add<String> for String {
    type Output = String;

    fn op_add(self, rhs: String) -> Self::Output {
        self + &rhs
    }
}

impl Add<&str> for String {
    type Output = String;

    fn op_add(self, rhs: &str) -> Self::Output {
        self + rhs
    }
}

impl Add<&&str> for String {
    type Output = String;

    fn op_add(self, rhs: &&str) -> Self::Output {
        self + *rhs
    }
}

impl Add<String> for &str {
    type Output = String;

    fn op_add(self, rhs: String) -> Self::Output {
        self.to_string() + &rhs
    }
}

impl Add<&String> for &str {
    type Output = String;

    fn op_add(self, rhs: &String) -> Self::Output {
        self.to_string() + rhs.as_str()
    }
}

impl Add<&&String> for &str {
    type Output = String;

    fn op_add(self, rhs: &&String) -> Self::Output {
        self.to_string() + rhs.as_str()
    }
}

#[cfg(test)]
mod test {
    use crate::ops::Add;
    use rbs::{to_value, Value};

    #[test]
    fn test_add() {
        let i: i64 = 1;
        let v = to_value!(1);
        let r = Value::from(v.op_add(&i));
        assert_eq!(r, Value::from(2));
    }
}
