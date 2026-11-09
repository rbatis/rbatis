use rbatis_codegen::ops::Sub;
use rbs::Value;

#[test]
fn test_value_sub_value() {
    // Value - Value
    let result1 = Value::I32(10).op_sub(&Value::I32(3));
    let result2 = Value::I64(20).op_sub(&Value::I64(5));
    let result3 = Value::U32(30).op_sub(&Value::U32(10));
    let result4 = Value::U64(40).op_sub(&Value::U64(15));
    let result5 = Value::F32(10.5).op_sub(&Value::F32(2.5));
    let result6 = Value::F64(20.5).op_sub(&Value::F64(5.5));

    assert!(matches!(result1, Value::I32(7)));
    assert!(matches!(result2, Value::I64(15)));
    assert!(matches!(result3, Value::U32(20)));
    assert!(matches!(result4, Value::U64(25)));
    assert!(matches!(result5, Value::F32(8.0)));
    assert!(matches!(result6, Value::F64(15.0)));
}

#[test]
fn test_value_sub_primitive() {
    // Value - 原始类型
    assert_eq!(Value::I32(10).op_sub(&3i32), 7i64);
    assert_eq!(Value::I64(20).op_sub(&5i64), 15i64);
    assert_eq!(Value::U32(30).op_sub(&10u32), 20u64);
    assert_eq!(Value::U64(40).op_sub(&15u64), 25u64);
    assert_eq!(Value::F32(10.5).op_sub(&2.5f32), 8.0f64);
    assert_eq!(Value::F64(20.5).op_sub(&5.5f64), 15.0f64);
}

#[test]
fn test_primitive_sub_value() {
    // 原始类型 - Value（测试方向性问题）
    let result1 = 10i32.op_sub(&Value::I32(3));
    let result2 = 20i64.op_sub(&Value::I64(5));
    let result3 = 30u32.op_sub(&Value::U32(10));
    let result4 = 40u64.op_sub(&Value::U64(15));
    let result5 = 10.5f32.op_sub(&Value::F32(2.5));
    let result6 = 20.5f64.op_sub(&Value::F64(5.5));

    assert_eq!(result1, 7i64);
    assert_eq!(result2, 15i64);
    assert_eq!(result3, 20u64);
    assert_eq!(result4, 25u64);
    assert_eq!(result5, 8.0f64);
    assert_eq!(result6, 15.0f64);

    // 测试方向性
    // a - b != b - a，所以这两个应该不相等
    let a_minus_b = 10i32.op_sub(&Value::I32(3)); // 10 - 3 = 7
    let b_minus_a = Value::I32(3).op_sub(&10i32); // 3 - 10 = -7
    assert_ne!(a_minus_b, b_minus_a);

    // 验证b_minus_a是否正确
    assert_eq!(b_minus_a, -7i64);
}

#[test]
fn test_sub_ref_variants() {
    // 测试引用类型
    let v1 = Value::I32(10);
    let v2 = Value::I32(3);
    let i1 = 10i32;

    // Value和引用
    let result1 = (&v1).op_sub(&v2);
    let result2 = v1.op_sub(&&v2);

    // 原始类型和引用
    let result3 = (&i1).op_sub(&v2);
    let result4 = i1.op_sub(&&v2);

    assert!(matches!(result1, Value::I32(7)));
    assert!(matches!(result2, Value::I32(7)));
    assert_eq!(result3, 7i64);
    assert_eq!(result4, 7i64);
}

#[test]
fn test_sub_underflow_neither_panics_nor_wraps() {
    // `#{count} - 1`（count 是 0 的 U64/U32 列）以前在 debug 编译里 panic、
    // 在 release 编译里绕回成最大整数。现在是饱和或者换到能精确表示它的 Value 整数
    assert_eq!(Value::U64(0).op_sub(&Value::U64(1)), Value::U64(0));
    // u32 减成负数：结果换成有符号的 I64（换成 I32 的话 u32 会先绕回）
    assert_eq!(Value::U32(0).op_sub(&Value::U32(1)), Value::I64(-1));
    assert_eq!(
        Value::U32(0).op_sub(&Value::U32(u32::MAX)),
        Value::I64(-(u32::MAX as i64))
    );
    assert_eq!(
        Value::I32(i32::MIN).op_sub(&Value::I32(1)),
        Value::I64(i32::MIN as i64 - 1)
    );
    assert_eq!(
        Value::I64(i64::MIN).op_sub(&Value::I64(1)),
        Value::I64(i64::MIN)
    );
    // 固定返回类型的两条辅助路径
    assert_eq!(Value::U64(0).op_sub(1u64), 0u64);
    assert_eq!(1u64.op_sub(&Value::U64(2)), 0u64);
}
