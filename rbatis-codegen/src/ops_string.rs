use crate::ops::StrMethods;
use rbs::Value;

macro_rules! impl_str_methods {
    ($($ty:ty),* $(,)?) => {
        $(
            impl StrMethods for $ty {
                fn contains_str(self, value: &str) -> bool {
                    self.as_str().unwrap_or_default().contains(value)
                }

                fn starts_with(self, value: &str) -> bool {
                    self.as_str().unwrap_or_default().starts_with(value)
                }

                fn ends_with(self, value: &str) -> bool {
                    self.as_str().unwrap_or_default().ends_with(value)
                }
            }
        )*
    };
}

impl_str_methods!(Value, &Value, &&Value);

#[cfg(test)]
mod test {
    use crate::ops::StrMethods;
    use rbs::Value;

    #[test]
    fn test_str_methods() {
        let value = Value::String("hello world".to_string());
        assert!((&value).contains_str("world"));
        assert!((&value).starts_with("hello"));
        assert!((&&value).ends_with("world"));
        assert!(!Value::Null.contains_str("world"));
    }
}
