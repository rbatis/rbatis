use rbs::Value;
use rbatis_codegen::ops::Rem;

#[test]
fn test_value_rem_value() {
    // Value % Value
    let result1 = Value::I32(10).op_rem(&Value::I32(3));
    let result2 = Value::I64(11).op_rem(&Value::I64(4));
    let result3 = Value::U32(12).op_rem(&Value::U32(5));
    let result4 = Value::U64(13).op_rem(&Value::U64(5));
    let result5 = Value::F32(10.5).op_rem(&Value::F32(3.0));
    let result6 = Value::F64(11.5).op_rem(&Value::F64(4.0));
    
    assert!(matches!(result1, Value::I32(1)));
    assert!(matches!(result2, Value::I64(3)));
    assert!(matches!(result3, Value::U32(2)));
    assert!(matches!(result4, Value::U64(3)));
    assert!(matches!(result5, Value::F32(1.5)));
    assert!(matches!(result6, Value::F64(3.5)));
}

#[test]
fn test_value_rem_primitive() {
    // Value % 原始类型
    assert_eq!(Value::I32(10).op_rem(&3i32), 1i64);
    assert_eq!(Value::I64(11).op_rem(&4i64), 3i64);
    assert_eq!(Value::U32(12).op_rem(&5u32), 2u64);
    assert_eq!(Value::U64(13).op_rem(&5u64), 3u64);
    assert_eq!(Value::F32(10.5).op_rem(&3.0f32), 1.5f64);
    assert_eq!(Value::F64(11.5).op_rem(&4.0f64), 3.5f64);
}

#[test]
fn test_primitive_rem_value() {
    // 原始类型 % Value（测试方向性问题）
    let result1 = 10i32.op_rem(&Value::I32(3));
    let result2 = 11i64.op_rem(&Value::I64(4));
    let result3 = 12u32.op_rem(&Value::U32(5));
    let result4 = 13u64.op_rem(&Value::U64(5));
    let result5 = 10.5f32.op_rem(&Value::F32(3.0));
    let result6 = 11.5f64.op_rem(&Value::F64(4.0));
    
    assert_eq!(result1, 1i64);
    assert_eq!(result2, 3i64);
    assert_eq!(result3, 2u64);
    assert_eq!(result4, 3u64);
    assert_eq!(result5, 1.5f64);
    assert_eq!(result6, 3.5f64);
    
    // 测试方向性
    // a % b != b % a，所以这两个应该不相等
    assert_ne!(5i32.op_rem(&Value::I32(2)), Value::I32(2).op_rem(&5i32));
    assert_ne!(7i64.op_rem(&Value::I64(3)), Value::I64(3).op_rem(&7i64));
}

#[test]
fn test_rem_ref_variants() {
    // 测试引用类型
    let v1 = Value::I32(10);
    let v2 = Value::I32(3);
    let i1 = 10i32;
    
    // Value和引用
    let result1 = (&v1).op_rem(&v2);
    let result2 = v1.op_rem(&&v2);
    
    // 原始类型和引用
    let result3 = (&i1).op_rem(&v2);
    let result4 = i1.op_rem(&&v2);
    
    assert!(matches!(result1, Value::I32(1)));
    assert!(matches!(result2, Value::I32(1)));
    assert_eq!(result3, 1i64);
    assert_eq!(result4, 1i64);
} 