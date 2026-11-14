use crate::ops::Str;
use rbs::Value;

impl Str for Value {
    fn contains_str(&self, value: Value) -> bool {
        self.as_str()
            .unwrap_or_default()
            .contains(value.as_str().unwrap_or_default())
    }

    fn starts_with(&self, value: Value) -> bool {
        self.as_str()
            .unwrap_or_default()
            .starts_with(value.as_str().unwrap_or_default())
    }

    fn ends_with(&self, value: Value) -> bool {
        self.as_str()
            .unwrap_or_default()
            .ends_with(value.as_str().unwrap_or_default())
    }
}

#[cfg(test)]
mod test {
    use crate::ops::Str;
    use rbs::Value;

    #[test]
    fn test_str_methods() {
        let value = Value::String("hello world".to_string());
        assert!(value.contains_str(Value::String("world".to_string())));
        assert!(value.starts_with(Value::String("hello".to_string())));
        assert!(value.ends_with(Value::String("world".to_string())));
        assert!(!Value::Null.contains_str(Value::String("world".to_string())));

        let other = Value::String("world".to_string());
        assert!(value.contains_str(other));
    }
}
