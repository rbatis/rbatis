use rbs::Value;
use rbatis_codegen::ops::Mul;

#[test]
fn test_value_mul_value() {
    // Value * Value
    let result1 = Value::I32(5).op_mul(&Value::I32(3));
    let result2 = Value::I64(6).op_mul(&Value::I64(4));
    let result3 = Value::U32(7).op_mul(&Value::U32(2));
    let result4 = Value::U64(8).op_mul(&Value::U64(5));
    let result5 = Value::F32(2.5).op_mul(&Value::F32(3.0));
    let result6 = Value::F64(4.5).op_mul(&Value::F64(2.0));
    
    assert!(matches!(result1, Value::I32(15)));
    assert!(matches!(result2, Value::I64(24)));
    assert!(matches!(result3, Value::U32(14)));
    assert!(matches!(result4, Value::U64(40)));
    assert!(matches!(result5, Value::F32(7.5)));
    assert!(matches!(result6, Value::F64(9.0)));
}

#[test]
fn test_value_mul_primitive() {
    // Value * 原始类型
    assert_eq!(Value::I32(5).op_mul(&3i32), 15i64);
    assert_eq!(Value::I64(6).op_mul(&4i64), 24i64);
    assert_eq!(Value::U32(7).op_mul(&2u32), 14u64);
    assert_eq!(Value::U64(8).op_mul(&5u64), 40u64);
    assert_eq!(Value::F32(2.5).op_mul(&3.0f32), 7.5f64);
    assert_eq!(Value::F64(4.5).op_mul(&2.0f64), 9.0f64);
}

#[test]
fn test_primitive_mul_value() {
    // 原始类型 * Value (测试方向性)
    assert_eq!(3i32.op_mul(&Value::I32(5)), 15i64);
    assert_eq!(4i64.op_mul(&Value::I64(6)), 24i64);
    assert_eq!(2u32.op_mul(&Value::U32(7)), 14u64);
    assert_eq!(5u64.op_mul(&Value::U64(8)), 40u64);
    assert_eq!(3.0f32.op_mul(&Value::F32(2.5)), 7.5f64);
    assert_eq!(2.0f64.op_mul(&Value::F64(4.5)), 9.0f64);
    
    // 确认乘法的可交换性 a*b = b*a
    assert_eq!(Value::I32(5).op_mul(&3i32), 3i32.op_mul(&Value::I32(5)));
    assert_eq!(Value::F64(4.5).op_mul(&2.0f64), 2.0f64.op_mul(&Value::F64(4.5)));
}

#[test]
fn test_mul_ref_variants() {
    // 测试引用类型
    let v1 = Value::I32(5);
    let v2 = Value::I32(3);
    let i1 = 5i32;
    
    // Value和引用
    let result1 = (&v1).op_mul(&v2);
    let result2 = v1.op_mul(&&v2);
    
    // 原始类型和引用
    let result3 = (&i1).op_mul(&v2);
    let result4 = i1.op_mul(&&v2);
    
    assert!(matches!(result1, Value::I32(15)));
    assert!(matches!(result2, Value::I32(15)));
    assert_eq!(result3, 15i64);
    assert_eq!(result4, 15i64);
} 