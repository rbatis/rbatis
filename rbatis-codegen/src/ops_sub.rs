use crate::ops::{AsProxy, Sub};
use crate::ops_macros::{impl_numeric_binary_op, impl_same_type_binary_op, impl_value_binary_op};
use rbs::Value;

fn op_sub_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(s) => Value::I32(s - rhs.i32()),
        Value::I64(s) => Value::I64(s - rhs.i64()),
        Value::U32(s) => Value::U32(s - rhs.u32()),
        Value::U64(s) => Value::U64(s - rhs.u64()),
        Value::F32(s) => Value::F32(s - rhs.f64() as f32),
        Value::F64(s) => Value::F64(s - rhs.f64()),
        Value::Ext(_, e) => op_sub_value(*e, rhs),
        _ => Value::Null,
    }
}

fn op_sub_u64(value: &Value, other: u64) -> u64 {
    value.u64() - other
}

fn op_sub_i64(value: &Value, other: i64) -> i64 {
    value.i64() - other
}

fn op_sub_f64(value: &Value, other: f64) -> f64 {
    value.f64() - other
}

fn op_sub_u64_value(value: &Value, other: u64) -> u64 {
    other - value.u64()
}

fn op_sub_i64_value(value: &Value, other: i64) -> i64 {
    other - value.i64()
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
