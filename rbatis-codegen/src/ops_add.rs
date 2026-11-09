use crate::ops::{Add, AsProxy};
use crate::ops_macros::{impl_numeric_binary_op, impl_same_type_binary_op, impl_value_binary_op};
use rbs::Value;

// A `+` of two integers panics in a debug build and wraps around in a release build
// when the result does not fit the type. A number that came out of a database column
// (Value::U32/Value::U64) is added to a literal all the time, so a template would run
// differently in a debug and in a release build. Widen to the next larger Value integer
// when one exists, saturate when it does not: never a panic, never a wrapped number.
fn op_add_u64(value: &Value, other: u64) -> u64 {
    value.u64().saturating_add(other)
}

fn op_add_i64(value: &Value, other: i64) -> i64 {
    value.i64().saturating_add(other)
}

fn op_add_f64(value: &Value, other: f64) -> f64 {
    value.f64() + other
}

fn op_add_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::String(s) => Value::String(s + rhs.as_str().unwrap_or("")),
        Value::I32(s) => match s.checked_add(rhs.i32()) {
            Some(v) => Value::I32(v),
            None => Value::I64(s as i64 + rhs.i32() as i64),
        },
        Value::I64(s) => Value::I64(s.saturating_add(rhs.i64())),
        Value::U32(s) => match s.checked_add(rhs.u32()) {
            Some(v) => Value::U32(v),
            None => Value::U64(s as u64 + rhs.u32() as u64),
        },
        Value::U64(s) => Value::U64(s.saturating_add(rhs.u64())),
        Value::F64(s) => Value::F64(s + rhs.f64()),
        Value::F32(v) => Value::F32(v + rhs.f64() as f32),
        Value::Ext(_, e) => op_add_value(*e, rhs),
        _ => Value::Null,
    }
}

impl_value_binary_op!(Add, op_add, op_add_value);
impl_numeric_binary_op!(
    Add,
    op_add,
    op_add_u64,
    op_add_u64,
    [u8, u16, u32, u64],
    u64
);
impl_numeric_binary_op!(
    Add,
    op_add,
    op_add_i64,
    op_add_i64,
    [i8, i16, i32, i64, isize, usize],
    i64
);
impl_numeric_binary_op!(Add, op_add, op_add_f64, op_add_f64, [f32, f64], f64);
impl_same_type_binary_op!(
    Add,
    op_add,
    +,
    [
        u8, u16, u32, u64, i8, i16, i32, i64, isize, usize, f32, f64
    ]
);

macro_rules! impl_value_string_add {
    ($($rhs:ty),* $(,)?) => {
        $(
            impl Add<$rhs> for Value {
                type Output = Value;

                fn op_add(self, rhs: $rhs) -> Self::Output {
                    Value::String(self.string() + &rhs.to_string())
                }
            }

            impl Add<$rhs> for &Value {
                type Output = Value;

                fn op_add(self, rhs: $rhs) -> Self::Output {
                    Value::String(self.string() + &rhs.to_string())
                }
            }

            impl Add<$rhs> for &&Value {
                type Output = Value;

                fn op_add(self, rhs: $rhs) -> Self::Output {
                    Value::String(self.string() + &rhs.to_string())
                }
            }
        )*
    };
}

impl_value_string_add!(&str, &&str, String, &String, &&String);

macro_rules! impl_string_value_add {
    ($($lhs:ty),* $(,)?) => {
        $(
            impl Add<Value> for $lhs {
                type Output = String;

                fn op_add(self, rhs: Value) -> Self::Output {
                    self.to_string() + &rhs.string()
                }
            }

            impl Add<&Value> for $lhs {
                type Output = String;

                fn op_add(self, rhs: &Value) -> Self::Output {
                    self.to_string() + &rhs.string()
                }
            }

            impl Add<&&Value> for $lhs {
                type Output = String;

                fn op_add(self, rhs: &&Value) -> Self::Output {
                    self.to_string() + &rhs.string()
                }
            }
        )*
    };
}

impl_string_value_add!(&str, &&str, String, &String, &&String);

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

impl Add<String> for String {
    type Output = String;

    fn op_add(self, rhs: String) -> Self::Output {
        self + &rhs
    }
}

impl Add<&String> for String {
    type Output = String;

    fn op_add(self, rhs: &String) -> Self::Output {
        self + rhs.as_str()
    }
}

impl Add<&&String> for String {
    type Output = String;

    fn op_add(self, rhs: &&String) -> Self::Output {
        self + rhs.as_str()
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

impl Add<&str> for &str {
    type Output = String;

    fn op_add(self, rhs: &str) -> Self::Output {
        self.to_string() + rhs
    }
}

impl Add<&&str> for &str {
    type Output = String;

    fn op_add(self, rhs: &&str) -> Self::Output {
        self.to_string() + rhs
    }
}

impl Add<&str> for &&str {
    type Output = String;

    fn op_add(self, rhs: &str) -> Self::Output {
        self.to_string() + rhs
    }
}

impl Add<&&str> for &&str {
    type Output = String;

    fn op_add(self, rhs: &&str) -> Self::Output {
        self.to_string() + rhs
    }
}

#[cfg(test)]
mod test {
    use crate::ops::Add;
    use rbs::{value, Value};

    #[test]
    fn test_add() {
        let i: i64 = 1;
        let v = value!(1);
        assert_eq!((&v).op_add(&i), 2i64);
        assert_eq!(v.op_add(&i), 2i64);
        assert_eq!(Value::I32(1).op_add(&&Value::I32(2)), Value::I32(3));
    }

    #[test]
    fn test_string_reference_matrix() {
        let value = Value::String("left".to_string());
        assert_eq!(
            value.op_add("right"),
            Value::String("leftright".to_string())
        );
        assert_eq!(
            (&Value::String("left".to_string())).op_add("right"),
            Value::String("leftright".to_string())
        );
        assert_eq!(
            "left".op_add(&Value::String("right".to_string())),
            "leftright"
        );
    }
}
