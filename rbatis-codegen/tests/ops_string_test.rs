use rbatis_codegen::ops::StrMethods;
use rbs::Value;

#[test]
fn test_contains_str_value() {
    let value = Value::String("hello world".to_string());
    assert!(value.clone().contains_str("world"));
    assert!(value.clone().contains_str("hello"));
    assert!(!value.contains_str("foo"));
}

#[test]
fn test_contains_str_ref_value() {
    let value = Value::String("hello world".to_string());
    assert!((&value).contains_str("world"));
    assert!((&value).contains_str("hello"));
    assert!(!(&value).contains_str("foo"));
}

#[test]
fn test_contains_str_ref_ref_value() {
    let value = Value::String("hello world".to_string());
    let ref_value = &value;
    assert!((&ref_value).contains_str("world"));
    assert!((&ref_value).contains_str("hello"));
    assert!(!(&ref_value).contains_str("foo"));
}

#[test]
fn test_starts_with_value() {
    let value = Value::String("hello world".to_string());
    assert!(value.clone().starts_with("hello"));
    assert!(!value.clone().starts_with("world"));
    assert!(!value.starts_with("foo"));
}

#[test]
fn test_starts_with_ref_value() {
    let value = Value::String("hello world".to_string());
    assert!((&value).starts_with("hello"));
    assert!(!(&value).starts_with("world"));
    assert!(!(&value).starts_with("foo"));
}

#[test]
fn test_starts_with_ref_ref_value() {
    let value = Value::String("hello world".to_string());
    let ref_value = &value;
    assert!((&ref_value).starts_with("hello"));
    assert!(!(&ref_value).starts_with("world"));
    assert!(!(&ref_value).starts_with("foo"));
}

#[test]
fn test_ends_with_value() {
    let value = Value::String("hello world".to_string());
    assert!(value.clone().ends_with("world"));
    assert!(!value.clone().ends_with("hello"));
    assert!(!value.ends_with("foo"));
}

#[test]
fn test_ends_with_ref_value() {
    let value = Value::String("hello world".to_string());
    assert!((&value).ends_with("world"));
    assert!(!(&value).ends_with("hello"));
    assert!(!(&value).ends_with("foo"));
}

#[test]
fn test_ends_with_ref_ref_value() {
    let value = Value::String("hello world".to_string());
    let ref_value = &value;
    assert!((&ref_value).ends_with("world"));
    assert!(!(&ref_value).ends_with("hello"));
    assert!(!(&ref_value).ends_with("foo"));
}

#[test]
fn test_str_methods_with_null() {
    let null_value = Value::Null;
    assert!(!null_value.clone().contains_str("anything"));
    assert!(!null_value.clone().starts_with("anything"));
    assert!(!null_value.ends_with("anything"));
}

#[test]
fn test_str_methods_with_number() {
    let number_value = Value::I32(123);
    assert!(!number_value.clone().contains_str("123"));
    assert!(!number_value.clone().starts_with("1"));
    assert!(!number_value.ends_with("3"));
}

#[test]
fn test_str_methods_with_empty_string() {
    let empty_value = Value::String("".to_string());
    assert!(!empty_value.clone().contains_str("anything"));
    assert!(!empty_value.clone().starts_with("anything"));
    assert!(!empty_value.clone().ends_with("anything"));
    assert!(empty_value.clone().contains_str(""));
    assert!(empty_value.clone().starts_with(""));
    assert!(empty_value.ends_with(""));
}
