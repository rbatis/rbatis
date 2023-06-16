use crate::ops::Not;
use rbs::Value;

fn op_not(left: Value) -> Value {
    use std::ops::Not;
    match left {
        Value::I32(b) => Value::I32(b.not()),
        Value::I64(b) => Value::I64(b.not()),
        Value::U32(b) => Value::U32(b.not()),
        Value::U64(b) => Value::U64(b.not()),
        Value::Bool(b) => Value::Bool(b.not()),
        Value::Ext(_, e) => op_not(*e),
        _ => Value::Null,
    }
}

impl Not for Value {
    type Output = Value;

    fn op_not(self) -> Self::Output {
        op_not(self)
    }
}

impl Not for &Value {
    type Output = Value;
    fn op_not(self) -> Self::Output {
        op_not(self.to_owned())
    }
}

impl Not for &mut Value {
    type Output = Value;
    fn op_not(self) -> Self::Output {
        op_not(self.to_owned())
    }
}
