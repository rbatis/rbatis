#![allow(clippy::needless_borrow)]

use rbatis_codegen::ops::Str;
use rbs::Value;

#[test]
fn test_contains_str_value() {
    let v = Value::String("hello world".to_string());
    let result = v.contains_str(Value::String("world".to_string()));
    assert!(result);

    let v = Value::String("hello world".to_string());
    let result = v.contains_str(Value::String("test".to_string()));
    assert!(!result);
}

#[test]
fn test_contains_str_value_ref() {
    let v = Value::String("hello world".to_string());
    let result = (&v).contains_str(Value::String("world".to_string()));
    assert!(result);

    let v = Value::String("hello world".to_string());
    let result = (&v).contains_str(Value::String("test".to_string()));
    assert!(!result);
}

#[test]
fn test_contains_str_value_double_ref() {
    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.contains_str(Value::String("world".to_string()));
    assert!(result);

    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.contains_str(Value::String("test".to_string()));
    assert!(!result);
}

#[test]
fn test_value_argument() {
    let v = Value::String("hello world".to_string());
    let needle = Value::String("world".to_string());

    assert!(v.contains_str(needle));
    assert!(v.starts_with(Value::String("hello".to_string())));
    assert!(v.ends_with(Value::String("world".to_string())));
}

#[test]
fn test_starts_with_value() {
    let v = Value::String("hello world".to_string());
    let result = v.starts_with(Value::String("hello".to_string()));
    assert!(result);

    let v = Value::String("hello world".to_string());
    let result = v.starts_with(Value::String("world".to_string()));
    assert!(!result);
}

#[test]
fn test_starts_with_value_ref() {
    let v = Value::String("hello world".to_string());
    let result = (&v).starts_with(Value::String("hello".to_string()));
    assert!(result);

    let v = Value::String("hello world".to_string());
    let result = (&v).starts_with(Value::String("world".to_string()));
    assert!(!result);
}

#[test]
fn test_starts_with_value_double_ref() {
    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.starts_with(Value::String("hello".to_string()));
    assert!(result);

    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.starts_with(Value::String("world".to_string()));
    assert!(!result);
}

#[test]
fn test_ends_with_value() {
    let v = Value::String("hello world".to_string());
    let result = v.ends_with(Value::String("world".to_string()));
    assert!(result);

    let v = Value::String("hello world".to_string());
    let result = v.ends_with(Value::String("hello".to_string()));
    assert!(!result);
}

#[test]
fn test_ends_with_value_ref() {
    let v = Value::String("hello world".to_string());
    let result = (&v).ends_with(Value::String("world".to_string()));
    assert!(result);

    let v = Value::String("hello world".to_string());
    let result = (&v).ends_with(Value::String("hello".to_string()));
    assert!(!result);
}

#[test]
fn test_ends_with_value_double_ref() {
    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.ends_with(Value::String("world".to_string()));
    assert!(result);

    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.ends_with(Value::String("hello".to_string()));
    assert!(!result);
}

#[test]
fn test_string_methods_with_non_string_values() {
    // Test with non-String values that should return default behavior
    let v = Value::I32(42);
    let result = v.contains_str(Value::String("test".to_string()));
    assert!(!result);

    let v = Value::I32(42);
    let result = v.starts_with(Value::String("test".to_string()));
    assert!(!result);

    let v = Value::I32(42);
    let result = v.ends_with(Value::String("test".to_string()));
    assert!(!result);

    let v = Value::Null;
    let result = v.contains_str(Value::String("test".to_string()));
    assert!(!result);

    let v = Value::Null;
    let result = v.starts_with(Value::String("test".to_string()));
    assert!(!result);

    let v = Value::Null;
    let result = v.ends_with(Value::String("test".to_string()));
    assert!(!result);
}

#[test]
fn test_string_methods_with_empty_strings() {
    let v = Value::String("".to_string());
    let result = v.contains_str(Value::String("test".to_string()));
    assert!(!result);

    let v = Value::String("".to_string());
    let result = v.starts_with(Value::String("test".to_string()));
    assert!(!result);

    let v = Value::String("".to_string());
    let result = v.ends_with(Value::String("test".to_string()));
    assert!(!result);

    // Test with empty pattern
    let v = Value::String("hello".to_string());
    let result = v.contains_str(Value::String("".to_string()));
    assert!(result);

    let v = Value::String("hello".to_string());
    let result = v.starts_with(Value::String("".to_string()));
    assert!(result);

    let v = Value::String("hello".to_string());
    let result = v.ends_with(Value::String("".to_string()));
    assert!(result);
}
