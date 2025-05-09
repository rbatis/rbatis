use rbs::Value;
use rbatis_codegen::ops::Neg;

#[test]
fn test_value_neg() {
    // Value::neg
    assert_eq!(Value::I32(5).neg(), Value::I32(-5));
    assert_eq!(Value::I64(10).neg(), Value::I64(-10));
    assert_eq!(Value::F32(15.5).neg(), Value::F32(-15.5));
    assert_eq!(Value::F64(20.5).neg(), Value::F64(-20.5));
    
    // 双重取反应该回到原值
    assert_eq!(Value::I32(5).neg().neg(), Value::I32(5));
    assert_eq!(Value::I64(10).neg().neg(), Value::I64(10));
    assert_eq!(Value::F32(15.5).neg().neg(), Value::F32(15.5));
    assert_eq!(Value::F64(20.5).neg().neg(), Value::F64(20.5));
}

#[test]
fn test_value_ref_neg() {
    // &Value::neg
    let v1 = Value::I32(5);
    let v2 = Value::I64(10);
    let v3 = Value::F32(15.5);
    let v4 = Value::F64(20.5);
    
    assert_eq!((&v1).neg(), Value::I32(-5));
    assert_eq!((&v2).neg(), Value::I64(-10));
    assert_eq!((&v3).neg(), Value::F32(-15.5));
    assert_eq!((&v4).neg(), Value::F64(-20.5));
} 