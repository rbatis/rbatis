use crate::ops::Not;
use crate::ops_macros::impl_value_unary_op;
use rbs::Value;

fn op_not(left: Value) -> Value {
    match left {
        Value::I32(value) => Value::I32(!value),
        Value::I64(value) => Value::I64(!value),
        Value::U32(value) => Value::U32(!value),
        Value::U64(value) => Value::U64(!value),
        Value::Bool(value) => Value::Bool(!value),
        Value::Ext(_, value) => op_not(*value),
        _ => Value::Null,
    }
}

impl_value_unary_op!(Not, op_not, op_not);

#[cfg(test)]
mod test {
    use crate::ops::Not;
    use rbs::Value;

    #[test]
    fn test_not() {
        assert_eq!(Value::Bool(true).op_not(), Value::Bool(false));
        assert_eq!((&Value::I64(0)).op_not(), Value::I64(-1));
        assert_eq!((&&Value::U32(0)).op_not(), Value::U32(u32::MAX));
    }
}
