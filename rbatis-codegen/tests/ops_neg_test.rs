use rbatis_codegen::ops::Neg;
use rbs::Value;

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

#[test]
fn test_neg_of_the_smallest_integer() {
    // -i32::MIN 放不进 i32（debug 编译会 panic），放得进 i64 -> 结果放宽成 I64
    assert_eq!(Value::I32(i32::MIN).neg(), Value::I64(-(i32::MIN as i64)));
    // 没有比 i64 更宽的 Value 整数，-i64::MIN 饱和成 i64::MAX
    assert_eq!(Value::I64(i64::MIN).neg(), Value::I64(i64::MAX));
    // 正常情况类型不变
    assert_eq!(Value::I32(-2).neg(), Value::I32(2));
    assert_eq!(Value::I64(-2).neg(), Value::I64(2));
}
