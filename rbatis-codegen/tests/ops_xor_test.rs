use rbs::Value;
use rbatis_codegen::ops::BitXor;

#[test]
fn test_value_bitxor_value() {
    // Value ^ Value 返回 Value
    let result1 = Value::I32(5).op_bitxor(&Value::I32(3));
    let result2 = Value::I64(10).op_bitxor(&Value::I64(7));
    let result3 = Value::U32(12).op_bitxor(&Value::U32(5));
    let result4 = Value::U64(15).op_bitxor(&Value::U64(9));
    
    // 由于返回的是Value类型，我们可以直接进行断言
    assert!(matches!(result1, Value::I32(6)));
    assert!(matches!(result2, Value::I64(13)));
    assert!(matches!(result3, Value::U32(9)));
    assert!(matches!(result4, Value::U64(6)));
}

#[test]
fn test_value_bitxor_primitive() {
    // Value ^ 原始类型 - 只测试整数类型
    let v1 = Value::I32(5);
    let v2 = Value::I64(10);
    
    // 只使用支持的类型：i32, i64, isize
    assert_eq!((&v1).op_bitxor(3i32), 6i64);
    assert_eq!((&v2).op_bitxor(7i64), 13i64);
    
    // 也可以使用其他有符号整数类型
    let v3 = Value::I32(5);
    let v4 = Value::I64(10);
    assert_eq!((&v3).op_bitxor(3i16), 6i64);
    assert_eq!((&v4).op_bitxor(7i8), 13i64);
}

#[test]
fn test_primitive_bitxor_value() {
    // 原始类型 ^ Value - 只测试整数类型
    let v1 = Value::I32(3);
    let v2 = Value::I64(7);
    
    // 只使用支持的类型：i32, i64, isize
    assert_eq!(5i32.op_bitxor(v1), 6i64);
    assert_eq!(10i64.op_bitxor(v2), 13i64);
    
    // 也可以使用其他有符号整数类型
    let v3 = Value::I32(3);
    let v4 = Value::I64(7);
    assert_eq!(5i16.op_bitxor(v3), 6i64);
    assert_eq!(10i8.op_bitxor(v4), 13i64);
    
    // 确认异或的可交换性 a^b = b^a
    let v5 = Value::I32(3);
    assert_eq!((&Value::I32(5)).op_bitxor(3i32), 5i32.op_bitxor(v5));
}

#[test]
fn test_bitxor_ref_variants() {
    // 测试引用类型
    let v1 = Value::I32(5);
    let v2 = Value::I32(3);
    
    // Value和引用
    let result1 = (&v1).op_bitxor(&v2);
    
    // 由于v1被移动，我们需要重新创建v1
    let v1_new = Value::I32(5);
    let result2 = v1_new.op_bitxor(&&v2);
    
    // 使用断言测试
    assert!(matches!(result1, Value::I32(6)));
    assert!(matches!(result2, Value::I32(6)));
    
    // 原始类型和引用 - 注意引用方向
    let i1 = 5i32;
    let v3 = Value::I32(3);
    assert_eq!(i1.op_bitxor(v3), 6i64);  // 原始类型需要Value而不是&Value
    
    let v4 = Value::I32(3);
    assert_eq!((&Value::I32(5)).op_bitxor(3i32), 6i64);  // &Value需要原始类型而不是&原始类型
} 