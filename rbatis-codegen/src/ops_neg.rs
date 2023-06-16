use crate::ops::Neg;

use rbs::Value;

fn op_neg(left: Value) -> Value {
    use std::ops::Neg;
    match left {
        Value::I32(b) => Value::I32(b.neg()),
        Value::I64(b) => Value::I64(b.neg()),
        Value::F32(b) => Value::F32(b.neg()),
        Value::F64(b) => Value::F64(b.neg()),
        Value::Ext(_, e) => op_neg(*e),
        _ => Value::Null,
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        op_neg(self)
    }
}

impl Neg for &Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        op_neg(self.to_owned())
    }
}

impl Neg for &mut Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        op_neg(self.to_owned())
    }
}
