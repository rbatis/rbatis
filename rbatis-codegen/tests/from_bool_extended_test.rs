use rbatis_codegen::ops::From;
use rbs::Value;

#[test]
fn test_from_bool() {
    // Test From<bool> for bool
    let b: bool = From::op_from(true);
    assert_eq!(b, true);
    
    let b: bool = From::op_from(false);
    assert_eq!(b, false);
}

#[test]
fn test_from_bool_ref() {
    // Test From<&bool> for bool
    let bool_val = true;
    let b: bool = From::op_from(&bool_val);
    assert_eq!(b, true);
    
    let bool_val = false;
    let b: bool = From::op_from(&bool_val);
    assert_eq!(b, false);
}

#[test]
fn test_from_bool_double_ref() {
    // Test From<&&bool> for bool
    let bool_val = true;
    let bool_ref = &bool_val;
    let b: bool = From::op_from(&bool_ref);
    assert_eq!(b, true);
    
    let bool_val = false;
    let bool_ref = &bool_val;
    let b: bool = From::op_from(&bool_ref);
    assert_eq!(b, false);
}

#[test]
fn test_from_value_to_bool() {
    // Test From<Value> for bool
    let v = Value::Bool(true);
    let b: bool = From::op_from(v);
    assert_eq!(b, true);
    
    let v = Value::Bool(false);
    let b: bool = From::op_from(v);
    assert_eq!(b, false);
    
    // Test other Value types
    let v = Value::String("true".to_string());
    let b: bool = From::op_from(v);
    // Should return false for non-bool values
    assert_eq!(b, false);
    
    let v = Value::I32(1);
    let b: bool = From::op_from(v);
    // Should return false for non-bool values
    assert_eq!(b, false);
}

#[test]
fn test_from_value_ref_to_bool() {
    // Test From<&Value> for bool
    let v = Value::Bool(true);
    let b: bool = From::op_from(&v);
    assert_eq!(b, true);
    
    let v = Value::Bool(false);
    let b: bool = From::op_from(&v);
    assert_eq!(b, false);
    
    // Test other Value types
    let v = Value::String("true".to_string());
    let b: bool = From::op_from(&v);
    // Should return false for non-bool values
    assert_eq!(b, false);
}

#[test]
fn test_from_value_double_ref_to_bool() {
    // Test From<&&Value> for bool
    let v = Value::Bool(true);
    let v_ref = &v;
    let b: bool = From::op_from(&v_ref);
    assert_eq!(b, true);
    
    let v = Value::Bool(false);
    let v_ref = &v;
    let b: bool = From::op_from(&v_ref);
    assert_eq!(b, false);
    
    // Test other Value types
    let v = Value::String("true".to_string());
    let v_ref = &v;
    let b: bool = From::op_from(&v_ref);
    // Should return false for non-bool values
    assert_eq!(b, false);
}