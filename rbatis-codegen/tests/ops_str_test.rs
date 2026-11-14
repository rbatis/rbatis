#![allow(clippy::needless_borrow)]

use rbatis_codegen::ops::Str;
use rbs::Value;

#[test]
fn test_contains_str_value() {
    let value = Value::String("hello world".to_string());
    assert!(value.clone().contains_str(Value::String("world".to_string())));
    assert!(value.clone().contains_str(Value::String("hello".to_string())));
    assert!(!value.contains_str(Value::String("foo".to_string())));
}

#[test]
fn test_contains_str_ref_value() {
    let value = Value::String("hello world".to_string());
    assert!((&value).contains_str(Value::String("world".to_string())));
    assert!((&value).contains_str(Value::String("hello".to_string())));
    assert!(!(&value).contains_str(Value::String("foo".to_string())));
}

#[test]
fn test_contains_str_ref_ref_value() {
    let value = Value::String("hello world".to_string());
    let ref_value = &value;
    assert!((&ref_value).contains_str(Value::String("world".to_string())));
    assert!((&ref_value).contains_str(Value::String("hello".to_string())));
    assert!(!(&ref_value).contains_str(Value::String("foo".to_string())));
}

#[test]
fn test_starts_with_value() {
    let value = Value::String("hello world".to_string());
    assert!(value.clone().starts_with(Value::String("hello".to_string())));
    assert!(!value.clone().starts_with(Value::String("world".to_string())));
    assert!(!value.starts_with(Value::String("foo".to_string())));
}

#[test]
fn test_starts_with_ref_value() {
    let value = Value::String("hello world".to_string());
    assert!((&value).starts_with(Value::String("hello".to_string())));
    assert!(!(&value).starts_with(Value::String("world".to_string())));
    assert!(!(&value).starts_with(Value::String("foo".to_string())));
}

#[test]
fn test_starts_with_ref_ref_value() {
    let value = Value::String("hello world".to_string());
    let ref_value = &value;
    assert!((&ref_value).starts_with(Value::String("hello".to_string())));
    assert!(!(&ref_value).starts_with(Value::String("world".to_string())));
    assert!(!(&ref_value).starts_with(Value::String("foo".to_string())));
}

#[test]
fn test_ends_with_value() {
    let value = Value::String("hello world".to_string());
    assert!(value.clone().ends_with(Value::String("world".to_string())));
    assert!(!value.clone().ends_with(Value::String("hello".to_string())));
    assert!(!value.ends_with(Value::String("foo".to_string())));
}

#[test]
fn test_ends_with_ref_value() {
    let value = Value::String("hello world".to_string());
    assert!((&value).ends_with(Value::String("world".to_string())));
    assert!(!(&value).ends_with(Value::String("hello".to_string())));
    assert!(!(&value).ends_with(Value::String("foo".to_string())));
}

#[test]
fn test_ends_with_ref_ref_value() {
    let value = Value::String("hello world".to_string());
    let ref_value = &value;
    assert!((&ref_value).ends_with(Value::String("world".to_string())));
    assert!(!(&ref_value).ends_with(Value::String("hello".to_string())));
    assert!(!(&ref_value).ends_with(Value::String("foo".to_string())));
}

#[test]
fn test_str_methods_with_null() {
    let null_value = Value::Null;
    assert!(!null_value.clone().contains_str(Value::String("anything".to_string())));
    assert!(!null_value.clone().starts_with(Value::String("anything".to_string())));
    assert!(!null_value.ends_with(Value::String("anything".to_string())));
}

#[test]
fn test_str_methods_with_number() {
    let number_value = Value::I32(123);
    assert!(!number_value.clone().contains_str(Value::String("123".to_string())));
    assert!(!number_value.clone().starts_with(Value::String("1".to_string())));
    assert!(!number_value.ends_with(Value::String("3".to_string())));
}

#[test]
fn test_str_methods_with_empty_string() {
    let empty_value = Value::String("".to_string());
    assert!(!empty_value.clone().contains_str(Value::String("anything".to_string())));
    assert!(!empty_value.clone().starts_with(Value::String("anything".to_string())));
    assert!(!empty_value.clone().ends_with(Value::String("anything".to_string())));
    assert!(empty_value.clone().contains_str(Value::String("".to_string())));
    assert!(empty_value.clone().starts_with(Value::String("".to_string())));
    assert!(empty_value.ends_with(Value::String("".to_string())));
}
