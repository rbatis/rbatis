use crate::ops::{AsProxy, Rem};
use crate::ops_compare::integer_value;
use crate::ops_macros::{impl_numeric_binary_op, impl_same_type_binary_op, impl_value_binary_op};
use rbs::Value;

fn op_rem_u64(value: &Value, other: u64) -> u64 {
    if other == 0 {
        return 0;
    }
    value.u64() % other
}

fn op_rem_i64(value: &Value, other: i64) -> i64 {
    if other == 0 {
        return 0;
    }
    value.i64() % other
}

fn op_rem_f64(value: &Value, other: f64) -> f64 {
    if other == 0.0 {
        0.0
    } else {
        value.f64() % other
    }
}

fn op_rem_u64_value(value: &Value, other: u64) -> u64 {
    let rhs = value.u64();
    if rhs == 0 {
        return 0;
    }
    other % rhs
}

fn op_rem_i64_value(value: &Value, other: i64) -> i64 {
    let rhs = value.i64();
    if rhs == 0 {
        return 0;
    }
    other % rhs
}

fn op_rem_f64_value(value: &Value, other: f64) -> f64 {
    let rhs = value.f64();
    if rhs == 0.0 {
        0.0
    } else {
        other % rhs
    }
}

fn integer_rem(left: i128, rhs: &Value) -> Option<i128> {
    let rhs = integer_value(rhs)?;
    if rhs == 0 {
        Some(0)
    } else {
        Some(left % rhs)
    }
}

fn op_rem_value(left: Value, rhs: Value) -> Value {
    match left {
        Value::I32(value) => {
            if let Some(result) = integer_rem(value as i128, &rhs) {
                return i32::try_from(result)
                    .map(Value::I32)
                    .unwrap_or(Value::I32(0));
            }
            let rhs = rhs.f64();
            if rhs == 0.0 {
                return Value::I32(0);
            }
            Value::I32((value as f64 % rhs) as i32)
        }
        Value::I64(value) => {
            if let Some(result) = integer_rem(value as i128, &rhs) {
                return i64::try_from(result)
                    .map(Value::I64)
                    .unwrap_or(Value::I64(0));
            }
            let rhs = rhs.f64();
            if rhs == 0.0 {
                return Value::I64(0);
            }
            Value::I64((value as f64 % rhs) as i64)
        }
        Value::U32(value) => {
            if let Some(result) = integer_rem(value as i128, &rhs) {
                return u32::try_from(result)
                    .map(Value::U32)
                    .unwrap_or(Value::U32(0));
            }
            let rhs = rhs.f64();
            if rhs == 0.0 {
                return Value::U32(0);
            }
            Value::U32((value as f64 % rhs) as u32)
        }
        Value::U64(value) => {
            if let Some(result) = integer_rem(value as i128, &rhs) {
                return u64::try_from(result)
                    .map(Value::U64)
                    .unwrap_or(Value::U64(0));
            }
            let rhs = rhs.f64();
            if rhs == 0.0 {
                return Value::U64(0);
            }
            Value::U64((value as f64 % rhs) as u64)
        }
        Value::F32(value) => {
            let rhs = rhs.f64() as f32;
            if rhs == 0.0 {
                return Value::F32(0.0);
            }
            Value::F32(value % rhs)
        }
        Value::F64(value) => {
            let rhs = rhs.f64();
            if rhs == 0.0 {
                return Value::F64(0.0);
            }
            Value::F64(value % rhs)
        }
        Value::Ext(_, value) => op_rem_value(*value, rhs),
        _ => Value::Null,
    }
}

impl_value_binary_op!(Rem, op_rem, op_rem_value);
impl_numeric_binary_op!(
    Rem,
    op_rem,
    op_rem_u64,
    op_rem_u64_value,
    [u8, u16, u32, u64],
    u64
);
impl_numeric_binary_op!(
    Rem,
    op_rem,
    op_rem_i64,
    op_rem_i64_value,
    [i8, i16, i32, i64, isize, usize],
    i64
);
impl_numeric_binary_op!(Rem, op_rem, op_rem_f64, op_rem_f64_value, [f32, f64], f64);
impl_same_type_binary_op!(
    Rem,
    op_rem,
    %,
    [
        u8, u16, u32, u64, i8, i16, i32, i64, isize, usize, f32, f64
    ]
);

#[cfg(test)]
mod test {
    use crate::ops::Rem;
    use rbs::Value;

    #[test]
    fn test_rem() {
        let value = Value::I32(13);
        assert_eq!((&value).op_rem(&5i64), 3i64);
        assert_eq!(value.op_rem(5i64), 3i64);
        assert_eq!(Value::I32(13).op_rem(&&Value::I32(5)), Value::I32(3));
    }

    #[test]
    fn test_rem_by_zero() {
        assert_eq!(Value::I32(13).op_rem(0i64), 0i64);
        assert_eq!((&Value::I32(13)).op_rem(&&Value::I32(0)), Value::I32(0));
    }

    #[test]
    fn test_integer_rem_keeps_precision() {
        assert_eq!(
            Value::I64(9_007_199_254_740_993).op_rem(&Value::I64(2)),
            Value::I64(1)
        );
        assert_eq!(Value::I64(-5).op_rem(&Value::U64(2)), Value::I64(-1));
    }
}
