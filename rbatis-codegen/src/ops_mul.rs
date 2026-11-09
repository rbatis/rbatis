use crate::ops::{AsProxy, Mul};
use crate::ops_macros::{impl_numeric_binary_op, impl_same_type_binary_op, impl_value_binary_op};
use rbs::Value;

// A `*` of two integers panics in a debug build and wraps around in a release build when
// the result does not fit the type. Widen to the next larger Value integer when one
// exists (the product of two I32/u32 always fits an I64/U64), saturate when no Value
// integer can hold it: never a panic, never a wrapped number.
fn op_mul_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(s) => match s.checked_mul(rhs.i32()) {
            Some(v) => Value::I32(v),
            None => Value::I64(s as i64 * rhs.i32() as i64),
        },
        Value::I64(s) => Value::I64(s.saturating_mul(rhs.i64())),
        Value::U32(s) => match s.checked_mul(rhs.u32()) {
            Some(v) => Value::U32(v),
            None => Value::U64(s as u64 * rhs.u32() as u64),
        },
        Value::U64(s) => Value::U64(s.saturating_mul(rhs.u64())),
        Value::F32(s) => Value::F32(s * rhs.f64() as f32),
        Value::F64(s) => Value::F64(s * rhs.f64()),
        Value::Ext(_, e) => op_mul_value(*e, rhs),
        _ => Value::Null,
    }
}

fn op_mul_u64(value: &Value, other: u64) -> u64 {
    value.u64().saturating_mul(other)
}

fn op_mul_i64(value: &Value, other: i64) -> i64 {
    value.i64().saturating_mul(other)
}

fn op_mul_f64(value: &Value, other: f64) -> f64 {
    value.f64() * other
}

impl_value_binary_op!(Mul, op_mul, op_mul_value);
impl_numeric_binary_op!(
    Mul,
    op_mul,
    op_mul_u64,
    op_mul_u64,
    [u8, u16, u32, u64],
    u64
);
impl_numeric_binary_op!(
    Mul,
    op_mul,
    op_mul_i64,
    op_mul_i64,
    [i8, i16, i32, i64, isize, usize],
    i64
);
impl_numeric_binary_op!(Mul, op_mul, op_mul_f64, op_mul_f64, [f32, f64], f64);
impl_same_type_binary_op!(
    Mul,
    op_mul,
    *,
    [
        u8, u16, u32, u64, i8, i16, i32, i64, isize, usize, f32, f64
    ]
);

#[cfg(test)]
mod test {
    use crate::ops::Mul;
    use rbs::Value;

    #[test]
    fn test_mul() {
        let value = Value::I32(3);
        assert_eq!((&value).op_mul(&4i64), 12i64);
        assert_eq!(value.op_mul(4i64), 12i64);
        assert_eq!(Value::I32(3).op_mul(&&Value::I32(4)), Value::I32(12));
    }
}
