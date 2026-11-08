use crate::ops::{AsProxy, BitOr};
use crate::ops_macros::{
    impl_bool_scalar_binary_op, impl_bool_value_binary_op, impl_numeric_binary_op,
    impl_same_type_binary_op,
};
use rbs::Value;

fn op_bit_or_u64(value: &Value, other: u64) -> u64 {
    value.u64() | other
}

fn op_bit_or_i64(value: &Value, other: i64) -> i64 {
    value.i64() | other
}

impl_bool_value_binary_op!(BitOr, op_bitor, |);
impl_bool_scalar_binary_op!(BitOr, op_bitor, |);
impl_numeric_binary_op!(
    BitOr,
    op_bitor,
    op_bit_or_u64,
    op_bit_or_u64,
    [u8, u16, u32, u64],
    u64
);
impl_numeric_binary_op!(
    BitOr,
    op_bitor,
    op_bit_or_i64,
    op_bit_or_i64,
    [i8, i16, i32, i64, isize, usize],
    i64
);
impl_same_type_binary_op!(
    BitOr,
    op_bitor,
    |,
    [u8, u16, u32, u64, i8, i16, i32, i64, isize, usize, bool]
);

#[cfg(test)]
mod test {
    use crate::ops::BitOr;
    use rbs::Value;

    #[test]
    fn test_bitor() {
        assert_eq!(Value::I32(5).op_bitor(3i32), 7i64);
        assert_eq!((&Value::I32(5)).op_bitor(&3i32), 7i64);
        assert!(Value::Bool(false).op_bitor(&&Value::Bool(true)));
        assert!(false.op_bitor(&true));
    }
}
