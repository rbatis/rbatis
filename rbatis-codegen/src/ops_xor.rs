use crate::ops::{AsProxy, BitXor};
use crate::ops_macros::{
    impl_bool_scalar_binary_op, impl_numeric_binary_op, impl_same_type_binary_op,
    impl_value_binary_op,
};
use rbs::Value;

fn op_bitxor_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::Bool(s) => Value::Bool(s ^ rhs.bool()),
        Value::I32(s) => Value::I32(s ^ rhs.i32()),
        Value::I64(s) => Value::I64(s ^ rhs.i64()),
        Value::U32(s) => Value::U32(s ^ rhs.u32()),
        Value::U64(s) => Value::U64(s ^ rhs.u64()),
        Value::Ext(_, e) => op_bitxor_value(*e, rhs),
        _ => Value::Null,
    }
}

fn op_bitxor_u64(value: &Value, other: u64) -> u64 {
    value.u64() ^ other
}

fn op_bitxor_i64(value: &Value, other: i64) -> i64 {
    value.i64() ^ other
}

fn op_bitxor_f64(value: &Value, other: f64) -> f64 {
    ((value.f64() as i64) ^ (other as i64)) as f64
}

impl_value_binary_op!(BitXor, op_bitxor, op_bitxor_value);
impl_bool_scalar_binary_op!(BitXor, op_bitxor, ^);
impl_numeric_binary_op!(
    BitXor,
    op_bitxor,
    op_bitxor_u64,
    op_bitxor_u64,
    [u8, u16, u32, u64],
    u64
);
impl_numeric_binary_op!(
    BitXor,
    op_bitxor,
    op_bitxor_i64,
    op_bitxor_i64,
    [i8, i16, i32, i64, isize, usize],
    i64
);
impl_numeric_binary_op!(
    BitXor,
    op_bitxor,
    op_bitxor_f64,
    op_bitxor_f64,
    [f32, f64],
    f64
);
impl_same_type_binary_op!(
    BitXor,
    op_bitxor,
    ^,
    [u8, u16, u32, u64, i8, i16, i32, i64, isize, usize, bool]
);

#[cfg(test)]
mod test {
    use crate::ops::BitXor;
    use rbs::Value;

    #[test]
    fn test_bitxor() {
        assert_eq!(Value::I32(5).op_bitxor(3i32), 6i64);
        assert_eq!((&Value::I32(5)).op_bitxor(&3i32), 6i64);
        assert_eq!(Value::I32(5).op_bitxor(&&Value::I32(3)), Value::I32(6));
        assert!(true.op_bitxor(&false));
        assert!(Value::Bool(true).op_bitxor(false));
    }
}
