use crate::ops::{AsProxy, Sub};
use crate::ops_macros::{impl_numeric_binary_op, impl_same_type_binary_op, impl_value_binary_op};
use rbs::Value;

// A `-` of two integers panics in a debug build and wraps around in a release build when
// the result does not fit the type. A number that came out of a database column
// (Value::U32/Value::U64) is subtracted by a literal all the time (`#{count} - 1` of a
// count of 0), so a template would run differently in a debug and in a release build.
// Keep the exact result in the next larger Value integer when one exists (a U32 that
// went negative becomes a signed I64), saturate when no Value integer can hold it:
// never a panic, never a wrapped number.
fn op_sub_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(s) => match s.checked_sub(rhs.i32()) {
            Some(v) => Value::I32(v),
            None => Value::I64(s as i64 - rhs.i32() as i64),
        },
        Value::I64(s) => Value::I64(s.saturating_sub(rhs.i64())),
        // a U32 that went negative is a signed number. I64 and not I32: a u32 cast to an
        // i32 wraps, and the difference of two u32 always fits an i64
        Value::U32(s) => match s.checked_sub(rhs.u32()) {
            Some(v) => Value::U32(v),
            None => Value::I64(s as i64 - rhs.u32() as i64),
        },
        Value::U64(s) => Value::U64(s.saturating_sub(rhs.u64())),
        Value::F32(s) => Value::F32(s - rhs.f64() as f32),
        Value::F64(s) => Value::F64(s - rhs.f64()),
        Value::Ext(_, e) => op_sub_value(*e, rhs),
        _ => Value::Null,
    }
}

fn op_sub_u64(value: &Value, other: u64) -> u64 {
    value.u64().saturating_sub(other)
}

fn op_sub_i64(value: &Value, other: i64) -> i64 {
    value.i64().saturating_sub(other)
}

fn op_sub_f64(value: &Value, other: f64) -> f64 {
    value.f64() - other
}

fn op_sub_u64_value(value: &Value, other: u64) -> u64 {
    other.saturating_sub(value.u64())
}

fn op_sub_i64_value(value: &Value, other: i64) -> i64 {
    other.saturating_sub(value.i64())
}

fn op_sub_f64_value(value: &Value, other: f64) -> f64 {
    other - value.f64()
}

impl_value_binary_op!(Sub, op_sub, op_sub_value);
impl_numeric_binary_op!(
    Sub,
    op_sub,
    op_sub_u64,
    op_sub_u64_value,
    [u8, u16, u32, u64],
    u64
);
impl_numeric_binary_op!(
    Sub,
    op_sub,
    op_sub_i64,
    op_sub_i64_value,
    [i8, i16, i32, i64, isize, usize],
    i64
);
impl_numeric_binary_op!(Sub, op_sub, op_sub_f64, op_sub_f64_value, [f32, f64], f64);
impl_same_type_binary_op!(
    Sub,
    op_sub,
    -,
    [
        u8, u16, u32, u64, i8, i16, i32, i64, isize, usize, f32, f64
    ]
);

#[cfg(test)]
mod test {
    use crate::ops::Sub;
    use rbs::{value, Value};

    #[test]
    fn test_sub() {
        let i: i64 = 1;
        let v = value!(2);
        assert_eq!((&v).op_sub(&i), 1i64);
        assert_eq!(v.op_sub(&i), 1i64);
        assert_eq!(Value::I32(2).op_sub(&&Value::I32(1)), Value::I32(1));
    }
}
