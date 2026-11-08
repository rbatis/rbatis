use crate::ops::Neg;
use crate::ops_macros::impl_value_unary_op;
use rbs::Value;

fn op_neg(left: Value) -> Value {
    match left {
        Value::I32(value) => Value::I32(-value),
        Value::I64(value) => Value::I64(-value),
        Value::F32(value) => Value::F32(-value),
        Value::F64(value) => Value::F64(-value),
        Value::Ext(_, value) => op_neg(*value),
        _ => Value::Null,
    }
}

impl_value_unary_op!(Neg, neg, op_neg);

#[cfg(test)]
mod test {
    use crate::ops::Neg;
    use rbs::Value;

    #[test]
    fn test_neg() {
        assert_eq!(Value::I32(2).neg(), Value::I32(-2));
        assert_eq!((&Value::F64(1.5)).neg(), Value::F64(-1.5));
        assert_eq!((&&Value::I64(3)).neg(), Value::I64(-3));
    }
}
