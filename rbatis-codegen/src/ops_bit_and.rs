use crate::ops::{AsProxy, BitAnd};
use crate::ops_macros::{
    impl_bool_scalar_binary_op, impl_bool_value_binary_op, impl_numeric_binary_op,
    impl_same_type_binary_op,
};
use rbs::Value;

fn op_bit_and_u64(value: &Value, other: u64) -> u64 {
    value.u64() & other
}

fn op_bit_and_i64(value: &Value, other: i64) -> i64 {
    value.i64() & other
}

impl_bool_value_binary_op!(BitAnd, op_bitand, &);
impl_bool_scalar_binary_op!(BitAnd, op_bitand, &);
impl_numeric_binary_op!(
    BitAnd,
    op_bitand,
    op_bit_and_u64,
    op_bit_and_u64,
    [u8, u16, u32, u64],
    u64
);
impl_numeric_binary_op!(
    BitAnd,
    op_bitand,
    op_bit_and_i64,
    op_bit_and_i64,
    [i8, i16, i32, i64, isize, usize],
    i64
);
impl_same_type_binary_op!(
    BitAnd,
    op_bitand,
    &,
    [u8, u16, u32, u64, i8, i16, i32, i64, isize, usize, bool]
);

#[cfg(test)]
mod test {
    use crate::ops::BitAnd;
    use rbs::Value;

    #[test]
    fn test_bitand() {
        assert_eq!(Value::I32(5).op_bitand(3i32), 1i64);
        assert_eq!((&Value::I32(5)).op_bitand(&3i32), 1i64);
        assert!(Value::Bool(true).op_bitand(&&Value::Bool(true)));
        assert!(!true.op_bitand(&false));
    }
}
