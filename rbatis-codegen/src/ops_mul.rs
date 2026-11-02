use crate::ops::{AsProxy, Mul};
use crate::ops_macros::{impl_numeric_binary_op, impl_same_type_binary_op, impl_value_binary_op};
use rbs::Value;

fn op_mul_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(s) => Value::I32(s * rhs.i32()),
        Value::I64(s) => Value::I64(s * rhs.i64()),
        Value::U32(s) => Value::U32(s * rhs.u32()),
        Value::U64(s) => Value::U64(s * rhs.u64()),
        Value::F32(s) => Value::F32(s * rhs.f64() as f32),
        Value::F64(s) => Value::F64(s * rhs.f64()),
        Value::Ext(_, e) => op_mul_value(*e, rhs),
        _ => Value::Null,
    }
}

fn op_mul_u64(value: &Value, other: u64) -> u64 {
    value.u64() * other
}

fn op_mul_i64(value: &Value, other: i64) -> i64 {
    value.i64() * other
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
