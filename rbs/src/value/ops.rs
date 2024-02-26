use crate::Value;

impl std::ops::Not for &Value {
    type Output = bool;
    fn not(self) -> Self::Output {
        match self {
            Value::Null => { true }
            Value::Bool(v) => { !v }
            Value::I32(v) => { !(*v == 1) }
            Value::I64(v) => { !(*v == 1) }
            Value::U32(v) => { !(*v == 1) }
            Value::U64(v) => { !(*v == 1) }
            Value::F32(v) => { !(*v == 1.0) }
            Value::F64(v) => { !(*v == 1.0) }
            Value::String(v) => { !(v == "true") }
            Value::Binary(_v) => { true }
            Value::Array(_v) => { true }
            Value::Map(_v) => { true }
            Value::Ext(_, v) => {
                std::ops::Not::not(v.as_ref())
            }
        }
    }
}

impl std::ops::Not for Value {
    type Output = bool;

    fn not(self) -> Self::Output {
        std::ops::Not::not(&self)
    }
}

#[cfg(test)]
mod test {
    use crate::Value;

    #[test]
    fn test_ops_not() {
        let v = Value::Null;
        assert_eq!(!v,true);
    }
}