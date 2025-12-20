use rbatis_codegen::ops::StrMethods;
use rbs::Value;

#[test]
fn test_contains_str_value() {
    let v = Value::String("hello world".to_string());
    let result = v.contains_str("world");
    assert_eq!(result, true);
    
    let v = Value::String("hello world".to_string());
    let result = v.contains_str("test");
    assert_eq!(result, false);
}

#[test]
fn test_contains_str_value_ref() {
    let v = Value::String("hello world".to_string());
    let result = (&v).contains_str("world");
    assert_eq!(result, true);
    
    let v = Value::String("hello world".to_string());
    let result = (&v).contains_str("test");
    assert_eq!(result, false);
}

#[test]
fn test_contains_str_value_double_ref() {
    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.contains_str("world");
    assert_eq!(result, true);
    
    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.contains_str("test");
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_value() {
    let v = Value::String("hello world".to_string());
    let result = v.starts_with("hello");
    assert_eq!(result, true);
    
    let v = Value::String("hello world".to_string());
    let result = v.starts_with("world");
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_value_ref() {
    let v = Value::String("hello world".to_string());
    let result = (&v).starts_with("hello");
    assert_eq!(result, true);
    
    let v = Value::String("hello world".to_string());
    let result = (&v).starts_with("world");
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_value_double_ref() {
    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.starts_with("hello");
    assert_eq!(result, true);
    
    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.starts_with("world");
    assert_eq!(result, false);
}

#[test]
fn test_ends_with_value() {
    let v = Value::String("hello world".to_string());
    let result = v.ends_with("world");
    assert_eq!(result, true);
    
    let v = Value::String("hello world".to_string());
    let result = v.ends_with("hello");
    assert_eq!(result, false);
}

#[test]
fn test_ends_with_value_ref() {
    let v = Value::String("hello world".to_string());
    let result = (&v).ends_with("world");
    assert_eq!(result, true);
    
    let v = Value::String("hello world".to_string());
    let result = (&v).ends_with("hello");
    assert_eq!(result, false);
}

#[test]
fn test_ends_with_value_double_ref() {
    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.ends_with("world");
    assert_eq!(result, true);
    
    let v = Value::String("hello world".to_string());
    let v_ref = &v;
    let result = v_ref.ends_with("hello");
    assert_eq!(result, false);
}

#[test]
fn test_string_methods_with_non_string_values() {
    // Test with non-String values that should return default behavior
    let v = Value::I32(42);
    let result = v.contains_str("test");
    assert_eq!(result, false);
    
    let v = Value::I32(42);
    let result = v.starts_with("test");
    assert_eq!(result, false);
    
    let v = Value::I32(42);
    let result = v.ends_with("test");
    assert_eq!(result, false);
    
    let v = Value::Null;
    let result = v.contains_str("test");
    assert_eq!(result, false);
    
    let v = Value::Null;
    let result = v.starts_with("test");
    assert_eq!(result, false);
    
    let v = Value::Null;
    let result = v.ends_with("test");
    assert_eq!(result, false);
}

#[test]
fn test_string_methods_with_empty_strings() {
    let v = Value::String("".to_string());
    let result = v.contains_str("test");
    assert_eq!(result, false);
    
    let v = Value::String("".to_string());
    let result = v.starts_with("test");
    assert_eq!(result, false);
    
    let v = Value::String("".to_string());
    let result = v.ends_with("test");
    assert_eq!(result, false);
    
    // Test with empty pattern
    let v = Value::String("hello".to_string());
    let result = v.contains_str("");
    assert_eq!(result, true);
    
    let v = Value::String("hello".to_string());
    let result = v.starts_with("");
    assert_eq!(result, true);
    
    let v = Value::String("hello".to_string());
    let result = v.ends_with("");
    assert_eq!(result, true);
}