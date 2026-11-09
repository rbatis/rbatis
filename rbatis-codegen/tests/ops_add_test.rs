use rbatis_codegen::ops::Add;
use rbs::Value;

#[test]
fn test_value_add_value() {
    // 数值加法
    assert_eq!(Value::I32(5).op_add(&Value::I32(3)), Value::I32(8));
    assert_eq!(Value::I64(5).op_add(&Value::I64(3)), Value::I64(8));
    assert_eq!(Value::U32(5).op_add(&Value::U32(3)), Value::U32(8));
    assert_eq!(Value::U64(5).op_add(&Value::U64(3)), Value::U64(8));
    assert_eq!(Value::F32(5.0).op_add(&Value::F32(3.0)), Value::F32(8.0));
    assert_eq!(Value::F64(5.0).op_add(&Value::F64(3.0)), Value::F64(8.0));

    // 字符串加法
    assert_eq!(
        Value::String("hello".to_string()).op_add(&Value::String("world".to_string())),
        Value::String("helloworld".to_string())
    );
}

#[test]
fn test_value_add_primitive() {
    // Value + 原始类型
    assert_eq!(Value::I32(5).op_add(&3i32), 8i64);
    assert_eq!(Value::I64(5).op_add(&3i64), 8i64);
    assert_eq!(Value::U32(5).op_add(&3u32), 8u64);
    assert_eq!(Value::U64(5).op_add(&3u64), 8u64);
    assert_eq!(Value::F32(5.0).op_add(&3.0f32), 8.0f64);
    assert_eq!(Value::F64(5.0).op_add(&3.0f64), 8.0f64);

    // Value + 字符串
    let result = Value::String("hello".to_string()).op_add(&"world");
    assert_eq!(result, Value::String("helloworld".to_string()));

    // 字符串 + Value
    let string_val = "hello".op_add(&Value::String("world".to_string()));
    assert_eq!(string_val, "helloworld".to_string());
}

#[test]
fn test_primitive_add_value() {
    // 原始类型 + Value
    assert_eq!(5i32.op_add(&Value::I32(3)), 8i64);
    assert_eq!(5i64.op_add(&Value::I64(3)), 8i64);
    assert_eq!(5u32.op_add(&Value::U32(3)), 8u64);
    assert_eq!(5u64.op_add(&Value::U64(3)), 8u64);
    assert_eq!(5.0f32.op_add(&Value::F32(3.0)), 8.0f64);
    assert_eq!(5.0f64.op_add(&Value::F64(3.0)), 8.0f64);

    // 字符串 + Value - 方向性测试
    let s1 = "hello".to_string();
    let s2 = "world".to_string();
    let v1 = Value::String(s2);

    assert_eq!(s1.clone().op_add(&v1), "helloworld".to_string());
    assert_eq!((&s1).op_add(&v1), "helloworld".to_string());
}

#[test]
fn test_string_add_behavior() {
    // 测试各种字符串连接的方向性
    let s1 = "hello".to_string();
    let s2 = "world".to_string();
    let v1 = Value::String("hello".to_string());
    let v2 = Value::String("world".to_string());

    // String + Value::String
    assert_eq!(s1.clone().op_add(&v2), "helloworld".to_string());

    // Value::String + String
    let result = v1.clone().op_add(&s2);
    println!("Value::String + String = {:?}", result);
    assert_eq!(result, Value::String("helloworld".to_string()));

    // &str + Value::String
    assert_eq!("hello".op_add(&v2), "helloworld".to_string());

    // Value::String + &str
    assert_eq!(v1.op_add(&"world"), Value::String("helloworld".to_string()));
}

#[test]
fn test_string_add_number() {
    // 测试字符串与数字的连接
    let s = "number:".to_string();
    let v_num = Value::I32(42);

    // String + Value::Number
    let result = s.op_add(&v_num);
    println!("String + Value::Number = {:?}", result);
    // 预期应该是字符串连接，而不是尝试数值运算
    assert_eq!(result, "number:42".to_string());

    // Value::Number + String
    let result2 = v_num.op_add(&"42".to_string());
    println!("Value::Number + String = {:?}", result2);
    // 这里可能是实现方向性问题的地方
}

#[test]
fn test_mixed_type_add() {
    // 数值 + 字符串
    let num = 42i32;
    let s = "answer".to_string();

    // 测试双向操作
    let result1 = num.op_add(&Value::String(s.clone()));
    println!("Number + Value::String = {:?}", result1);

    let result2 = Value::I32(num).op_add(&s);
    println!("Value::Number + String = {:?}", result2);

    // 这里我们不做断言，只观察行为
}

#[test]
fn test_add_overflow_neither_panics_nor_wraps() {
    // 两个整数相加：debug 编译会 panic，release 编译会绕回。模板里的数字常常来自
    // 数据库列（U32/U64），所以同一段模板在 debug 和 release 下结果不同。
    // 现在：放得进更大的 Value 整数就用它（结果精确），放不下就饱和到边界
    assert_eq!(
        Value::I32(i32::MAX).op_add(&Value::I32(1)),
        Value::I64(i32::MAX as i64 + 1)
    );
    assert_eq!(
        Value::U32(u32::MAX).op_add(&Value::U32(1)),
        Value::U64(u32::MAX as u64 + 1)
    );
    // 没有比 i64/u64 更宽的 Value 整数，饱和
    assert_eq!(
        Value::I64(i64::MAX).op_add(&Value::I64(1)),
        Value::I64(i64::MAX)
    );
    assert_eq!(
        Value::U64(u64::MAX).op_add(&Value::U64(1)),
        Value::U64(u64::MAX)
    );
    // 原始类型的辅助函数返回类型是固定的，只能饱和
    assert_eq!(Value::U64(u64::MAX).op_add(1u64), u64::MAX);
    assert_eq!(Value::I64(i64::MAX).op_add(1i64), i64::MAX);
}
