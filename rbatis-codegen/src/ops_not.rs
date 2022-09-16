use crate::ops::Not;

use rbs::Value;

fn op_not(left: Value) -> Value {
    match left {
        Value::I32(b) => Value::I32(!b),
        Value::Bool(b) => Value::Bool(!b),
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
