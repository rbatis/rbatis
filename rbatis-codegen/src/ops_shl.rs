use crate::ops::{AsProxy, BitShl};
use crate::ops_macros::{impl_numeric_binary_op, impl_same_type_shift_op, impl_value_binary_op};
use rbs::Value;

fn op_shl_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(s) => {
            let rhs = rhs.i64();
            if !(0..i32::BITS as i64).contains(&rhs) {
                return Value::I32(0);
            }
            Value::I32(s << rhs)
        }
        Value::I64(s) => {
            let rhs = rhs.i64();
            if !(0..i64::BITS as i64).contains(&rhs) {
                return Value::I64(0);
            }
            Value::I64(s << rhs)
        }
        Value::U32(s) => {
            let rhs = rhs.u64();
            if rhs >= u32::BITS as u64 {
                return Value::U32(0);
            }
            Value::U32(s << rhs)
        }
        Value::U64(s) => {
            let rhs = rhs.u64();
            if rhs >= u64::BITS as u64 {
                return Value::U64(0);
            }
            Value::U64(s << rhs)
        }
        Value::Ext(_, e) => op_shl_value(*e, rhs),
        _ => Value::Null,
    }
}

fn op_shl_u64(value: &Value, other: u64) -> u64 {
    if other >= u64::BITS as u64 {
        return 0;
    }
    value.u64() << other
}

fn op_shl_i64(value: &Value, other: i64) -> i64 {
    if !(0..i64::BITS as i64).contains(&other) {
        return 0;
    }
    value.i64() << other
}

fn op_shl_u64_value(value: &Value, other: u64) -> u64 {
    let rhs = value.u64();
    if rhs >= u64::BITS as u64 {
        return 0;
    }
    other << rhs
}

fn op_shl_i64_value(value: &Value, other: i64) -> i64 {
    let rhs = value.i64();
    if !(0..i64::BITS as i64).contains(&rhs) {
        return 0;
    }
    other << rhs
}

impl_value_binary_op!(BitShl, op_shl, op_shl_value);
impl_numeric_binary_op!(
    BitShl,
    op_shl,
    op_shl_u64,
    op_shl_u64_value,
    [u8, u16, u32, u64],
    u64
);
impl_numeric_binary_op!(
    BitShl,
    op_shl,
    op_shl_i64,
    op_shl_i64_value,
    [i8, i16, i32, i64, isize, usize],
    i64
);
impl_same_type_shift_op!(
    BitShl,
    op_shl,
    checked_shl,
    [u8, u16, u32, u64, i8, i16, i32, i64, isize, usize]
);

#[cfg(test)]
mod test {
    use crate::ops::BitShl;
    use rbs::Value;

    #[test]
    fn test_shl() {
        assert_eq!(Value::I32(1).op_shl(2i64), 4i64);
        assert_eq!((&Value::I32(1)).op_shl(&2i64), 4i64);
        assert_eq!(Value::I32(1).op_shl(&&Value::I32(2)), Value::I32(4));
        assert_eq!(1i64.op_shl(&Value::I32(2)), 4i64);
    }

    #[test]
    fn test_shl_out_of_range() {
        assert_eq!(Value::I32(1).op_shl(64i64), 0i64);
        assert_eq!(Value::I32(1).op_shl(&&Value::I32(32)), Value::I32(0));
    }
}
