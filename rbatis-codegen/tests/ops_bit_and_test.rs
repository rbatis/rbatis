use rbs::Value;
use rbatis_codegen::ops::BitAnd;

#[test]
fn test_value_bitand_value() {
    // Value & Value 返回 bool
    let result1 = Value::I32(5).op_bitand(&Value::I32(3));
    let result2 = Value::I64(10).op_bitand(&Value::I64(7));
    let result3 = Value::U32(12).op_bitand(&Value::U32(5));
    let result4 = Value::U64(15).op_bitand(&Value::U64(9));
    
    // 检查返回的bool值
    println!("Value::I32(5) & Value::I32(3) = {:?}", result1);
    println!("Value::I64(10) & Value::I64(7) = {:?}", result2);
    println!("Value::U32(12) & Value::U32(5) = {:?}", result3);
    println!("Value::U64(15) & Value::U64(9) = {:?}", result4);
}

#[test]
fn test_value_bitand_primitive() {
    // Value & 原始类型 (这些返回数值类型)
    assert_eq!(Value::I32(5).op_bitand(&3i32), 1i64);
    assert_eq!(Value::I64(10).op_bitand(&7i64), 2i64);
    assert_eq!(Value::U32(12).op_bitand(&5u32), 4u64);
    assert_eq!(Value::U64(15).op_bitand(&9u64), 9u64);
}

#[test]
fn test_primitive_bitand_value() {
    // 原始类型 & Value
    assert_eq!(5i32.op_bitand(&Value::I32(3)), 1i64);
    assert_eq!(10i64.op_bitand(&Value::I64(7)), 2i64);
    assert_eq!(12u32.op_bitand(&Value::U32(5)), 4u64);
    assert_eq!(15u64.op_bitand(&Value::U64(9)), 9u64);
}

#[test]
fn test_ref_variants() {
    // 测试引用类型
    let v1 = Value::I32(5);
    let v2 = Value::I32(3);
    
    // Value类型的引用操作
    let result1 = (&v1).op_bitand(&v2);
    
    // 由于v1被移动，我们需要重新创建v1
    let v1_new = Value::I32(5);
    let result2 = v1_new.op_bitand(&&v2);
    
    // 同样地
    let v1_new2 = Value::I32(5);
    let result3 = (&v1_new2).op_bitand(&&v2);
    
    // 使用打印测试
    println!("&Value::I32(5) & Value::I32(3) = {:?}", result1);
    println!("Value::I32(5) & &&Value::I32(3) = {:?}", result2);
    println!("&Value::I32(5) & &&Value::I32(3) = {:?}", result3);
    
    // 原始类型和引用
    let i1 = 5i32;
    assert_eq!((&i1).op_bitand(&v2), 1i64);
    assert_eq!(i1.op_bitand(&&v2), 1i64);
} 