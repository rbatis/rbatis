use crate::ops::Str;
use rbs::Value;

impl Str for Value {
    fn contains_str(&self, value: Value) -> bool {
        self.contains(value)
    }

    fn contains(&self, value: Value) -> bool {
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

    fn eq_ignore_ascii_case(&self, value: Value) -> bool {
        self.as_str()
            .unwrap_or_default()
            .eq_ignore_ascii_case(value.as_str().unwrap_or_default())
    }

    fn replace(&self, from: Value, to: Value) -> Value {
        Value::String(self.as_str().unwrap_or_default().replace(
            from.as_str().unwrap_or_default(),
            to.as_str().unwrap_or_default(),
        ))
    }

    fn trim(&self) -> Value {
        Value::String(self.as_str().unwrap_or_default().trim().to_string())
    }

    fn trim_start(&self) -> Value {
        Value::String(self.as_str().unwrap_or_default().trim_start().to_string())
    }

    fn trim_end(&self) -> Value {
        Value::String(self.as_str().unwrap_or_default().trim_end().to_string())
    }

    fn to_lowercase(&self) -> Value {
        Value::String(self.as_str().unwrap_or_default().to_lowercase())
    }

    fn to_uppercase(&self) -> Value {
        Value::String(self.as_str().unwrap_or_default().to_uppercase())
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
        assert!(value.contains(Value::String("world".to_string())));
        assert!(value.starts_with(Value::String("hello".to_string())));
        assert!(value.ends_with(Value::String("world".to_string())));
        assert!(value.eq_ignore_ascii_case(Value::String("HELLO WORLD".to_string())));
        assert_eq!(value.trim(), Value::String("hello world".to_string()));
        assert_eq!(
            Value::String("  hello  ".to_string()).trim_start(),
            Value::String("hello  ".to_string())
        );
        assert_eq!(
            Value::String("  hello  ".to_string()).trim_end(),
            Value::String("  hello".to_string())
        );
        assert_eq!(
            Value::String("HELLO".to_string()).to_lowercase(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            Value::String("hello".to_string()).to_uppercase(),
            Value::String("HELLO".to_string())
        );
        assert_eq!(
            value.replace(
                Value::String("world".to_string()),
                Value::String("rust".to_string())
            ),
            Value::String("hello rust".to_string())
        );
        assert!(!Value::Null.contains_str(Value::String("world".to_string())));

        let other = Value::String("world".to_string());
        assert!(value.contains_str(other));
    }
}
