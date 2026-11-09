use rbatis_codegen::ops::Div;
use rbs::Value;

#[test]
fn test_value_div_value() {
    // 数值除法
    let result1 = Value::I32(10).op_div(&Value::I32(2));
    let result2 = Value::I64(20).op_div(&Value::I64(4));
    let result3 = Value::U32(30).op_div(&Value::U32(3));
    let result4 = Value::U64(40).op_div(&Value::U64(5));
    let result5 = Value::F32(10.0).op_div(&Value::F32(2.0));
    let result6 = Value::F64(20.0).op_div(&Value::F64(4.0));

    // 使用打印来检查实际值
    println!("Value::F32(10.0) / Value::F32(2.0) = {:?}", result5);
    println!("Value::F64(20.0) / Value::F64(4.0) = {:?}", result6);
    assert!(matches!(result1, Value::I32(5)));
    assert!(matches!(result2, Value::I64(5)));
    assert!(matches!(result3, Value::U32(10)));
    assert!(matches!(result4, Value::U64(8)));

    // 对于浮点数结果，我们不用精确匹配，而是检查类型和近似值
    if let Value::F32(val) = result5 {
        assert!((val - 5.0).abs() < 1e-10);
    } else {
        panic!("Expected Value::F64, got {:?}", result5);
    }

    if let Value::F64(val) = result6 {
        assert!((val - 5.0).abs() < 1e-10);
    } else {
        panic!("Expected Value::F64, got {:?}", result6);
    }
}

#[test]
fn test_value_div_primitive() {
    // Value / 原始类型
    let result1 = Value::I32(10).op_div(&2i32);
    let result2 = Value::I64(20).op_div(&4i64);
    let result3 = Value::U32(30).op_div(&3u32);
    let result4 = Value::U64(40).op_div(&5u64);
    let result5 = Value::F32(10.0).op_div(&2.0f32);
    let result6 = Value::F64(20.0).op_div(&4.0f64);

    assert_eq!(result1, 5i64);
    assert_eq!(result2, 5i64);
    assert_eq!(result3, 10u64);
    assert_eq!(result4, 8u64);
    assert_eq!(result5, 5.0f64);
    assert_eq!(result6, 5.0f64);
}

#[test]
fn test_primitive_div_value() {
    // 原始类型 / Value（测试方向性问题）
    let result1 = 20i32.op_div(&Value::I32(4));
    let result2 = 30i64.op_div(&Value::I64(6));
    let result3 = 40u32.op_div(&Value::U32(8));
    let result4 = 50u64.op_div(&Value::U64(10));
    let result5 = 20.0f32.op_div(&Value::F32(5.0));
    let result6 = 30.0f64.op_div(&Value::F64(10.0));

    // 检查操作是否正确
    assert_eq!(result1, 5i64);
    assert_eq!(result2, 5i64);
    assert_eq!(result3, 5u64);
    assert_eq!(result4, 5u64);
    assert_eq!(result5, 4.0f64);
    assert_eq!(result6, 3.0f64);

    // 反向情况测试，检验方向性是否正确
    let reverse1 = Value::I32(20).op_div(&4i32);
    let reverse2 = 5i32.op_div(&Value::I32(1));

    assert_eq!(reverse1, 5i64); // 20 / 4 = 5
    assert_eq!(reverse2, 5i64); // 5 / 1 = 5
}

#[test]
fn test_div_ref_variants() {
    // 测试引用类型
    let v1 = Value::I32(10);
    let v2 = Value::I32(2);
    let i1 = 10i32;

    // Value和引用
    let result1 = (&v1).op_div(&v2);
    let result2 = v1.op_div(&&v2);

    // 原始类型和引用
    let result3 = (&i1).op_div(&v2);
    let result4 = i1.op_div(&&v2);

    assert!(matches!(result1, Value::I32(5)));
    assert!(matches!(result2, Value::I32(5)));
    assert_eq!(result3, 5i64);
    assert_eq!(result4, 5i64);
}

#[test]
fn test_div_by_zero_of_value() {
    // 除数是 0 时结果是 0。整数分支和 `Value / 0.0 字面量` 本来就是 0，
    // 只有 `Value::F32 / Value::F32(0.0)`、`Value::F64 / Value::F64(0.0)` 会得到 inf，
    // 而 inf 会污染后面所有用它算出来的值。两个浮点分支现在也走同样的保护
    assert_eq!(Value::F64(20.0).op_div(&Value::F64(0.0)), Value::F64(0.0));
    assert_eq!(Value::F32(20.0).op_div(&Value::F32(0.0)), Value::F32(0.0));
    // 原始类型的一条路（除数是 0.0 字面量）
    assert_eq!(Value::F64(20.0).op_div(&0.0f64), 0.0f64);
    // 另一条（Value 是除数）
    assert_eq!(10.0f64.op_div(&Value::F64(0.0)), 0.0f64);
    // 整数 Value 除以 0.0（f64 回退分支）
    assert!(matches!(
        Value::I64(10).op_div(&Value::F64(0.0)),
        Value::I64(0)
    ));
}
