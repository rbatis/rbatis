use crate::ops::Not;

use rbs::Value;

impl Not for Value {
    type Output = bool;

    fn op_not(self) -> Self::Output {
        match self {
            Value::Bool(b) => !b,
            _ => true,
        }
    }
}

impl Not for &Value {
    type Output = bool;
    fn op_not(self) -> Self::Output {
        match self {
            Value::Bool(b) => !*b,
            _ => true,
        }
    }
}

impl Not for &mut Value {
    type Output = bool;
    fn op_not(self) -> Self::Output {
        match self {
            Value::Bool(b) => !*b,
            _ => true,
        }
    }
}
