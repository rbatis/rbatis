use rbatis_codegen::ops::PartialEq;
use rbs::Value;

#[test]
fn test_value_eq_value() {
    // 测试Value与Value的相等比较
    let v1 = Value::I32(10);
    let v2 = Value::I32(10);
    let v3 = Value::I32(20);
    let v4 = Value::String("10".to_string());
    
    assert!(v1.op_eq(&v2));
    assert!(!v1.op_eq(&v3));
    assert!(!v1.op_eq(&v4));
}

#[test]
fn test_reference_variants() {
    // 测试各种引用形式的相等比较
    let v1 = Value::I32(10);
    let v2 = Value::I32(10);
    let v1_ref = &v1;
    let v2_ref = &v2;
    let v1_ref_ref = &v1_ref;
    
    assert!(v1_ref.op_eq(&v2));
    assert!(v1_ref.op_eq(&v2_ref));
    assert!(v1_ref.op_eq(&v1_ref_ref));
    assert!(v1_ref_ref.op_eq(&v1_ref_ref));
    assert!(v1_ref_ref.op_eq(&v2));
    assert!(v1.op_eq(&v2_ref));
    assert!(v1.op_eq(&v1_ref_ref));
}

#[test]
fn test_primitive_eq_value() {
    // 测试原始类型与Value的相等比较
    let v_i32 = Value::I32(10);
    let v_f64 = Value::F64(10.0);
    let v_bool = Value::Bool(true);
    let v_str = Value::String("hello".to_string());
    
    // 整数比较
    assert!(10i32.op_eq(&v_i32));
    assert!(10i64.op_eq(&v_i32));
    assert!(10u32.op_eq(&v_i32));
    assert!(10u64.op_eq(&v_i32));
    assert!(!20i32.op_eq(&v_i32));
    
    // 浮点数比较
    assert!(10.0f64.op_eq(&v_f64));
    assert!(10.0f32.op_eq(&v_f64));
    assert!(!11.0f64.op_eq(&v_f64));
    
    // 布尔值比较
    assert!(true.op_eq(&v_bool));
    assert!(!false.op_eq(&v_bool));
    
    // 字符串比较
    assert!("hello".op_eq(&v_str));
    assert!(!"world".op_eq(&v_str));
    let hello_string = "hello".to_string();
    assert!(hello_string.op_eq(&v_str));
}

#[test]
fn test_value_eq_primitive() {
    // 测试Value与原始类型的相等比较
    let v_i32 = Value::I32(10);
    let v_f64 = Value::F64(10.0);
    let v_bool = Value::Bool(true);
    let v_str = Value::String("hello".to_string());
    
    // 整数比较
    assert!(v_i32.op_eq(&10i32));
    assert!(v_i32.op_eq(&10i64));
    assert!(v_i32.op_eq(&10u32));
    assert!(v_i32.op_eq(&10u64));
    assert!(!v_i32.op_eq(&20i32));
    
    // 浮点数比较
    assert!(v_f64.op_eq(&10.0f64));
    assert!(v_f64.op_eq(&10.0f32));
    assert!(!v_f64.op_eq(&11.0f64));
    
    // 布尔值比较
    assert!(v_bool.op_eq(&true));
    assert!(!v_bool.op_eq(&false));
    
    // 字符串比较
    assert!(v_str.op_eq(&"hello"));
    assert!(!v_str.op_eq(&"world"));
    let hello_string = "hello".to_string();
    assert!(v_str.op_eq(&hello_string));
}

#[test]
fn test_string_eq_number() {
    // 测试字符串与数字的相等比较
    let v_str_10 = Value::String("10".to_string());
    let v_str_10_0 = Value::String("10.0".to_string());
    let v_str_true = Value::String("true".to_string());
    
    // 通过打印日志看一下具体结果
    println!("\"10\".op_eq(&10i32): {}", v_str_10.op_eq(&10i32));
    println!("\"10.0\".op_eq(&10.0f64): {}", v_str_10_0.op_eq(&10.0f64));
    println!("\"true\".op_eq(&true): {}", v_str_true.op_eq(&true));
    
    // 10作为数字和字符串"10"可能相等也可能不相等，具体看实现
    // 这里先不做断言，只观察行为
}

#[test]
fn test_cross_type_eq() {
    // 测试跨类型的相等比较
    let v_i32 = Value::I32(10);
    let v_f64 = Value::F64(10.0);
    let v_bool = Value::Bool(true);
    let v_str = Value::String("hello".to_string());
    
    // 跨类型比较
    println!("i32(10).op_eq(&f64(10.0)): {}", v_i32.op_eq(&v_f64));
    println!("i32(10).op_eq(&bool(true)): {}", v_i32.op_eq(&v_bool));
    println!("i32(10).op_eq(&str(\"hello\")): {}", v_i32.op_eq(&v_str));
    println!("f64(10.0).op_eq(&bool(true)): {}", v_f64.op_eq(&v_bool));
    println!("f64(10.0).op_eq(&str(\"hello\")): {}", v_f64.op_eq(&v_str));
    println!("bool(true).op_eq(&str(\"hello\")): {}", v_bool.op_eq(&v_str));
    
    // 这里也先不做断言，只观察行为
} 