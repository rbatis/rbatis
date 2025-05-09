use std::cmp::Ordering;
use rbs::Value;
use rbatis_codegen::ops::PartialOrd;

#[test]
fn test_value_cmp_value() {
    // Value::Null
    assert_eq!(Value::Null.op_partial_cmp(&Value::Null), Some(Ordering::Equal));

    // Value::Bool
    assert_eq!(Value::Bool(true).op_partial_cmp(&Value::Bool(true)), Some(Ordering::Equal));
    assert_eq!(Value::Bool(true).op_partial_cmp(&Value::Bool(false)), Some(Ordering::Greater));
    assert_eq!(Value::Bool(false).op_partial_cmp(&Value::Bool(true)), Some(Ordering::Less));

    // Value::Number
    assert_eq!(Value::I32(10).op_partial_cmp(&Value::I32(10)), Some(Ordering::Equal));
    assert_eq!(Value::I32(20).op_partial_cmp(&Value::I32(10)), Some(Ordering::Greater));
    assert_eq!(Value::I32(10).op_partial_cmp(&Value::I32(20)), Some(Ordering::Less));

    assert_eq!(Value::I64(10).op_partial_cmp(&Value::I64(10)), Some(Ordering::Equal));
    assert_eq!(Value::I64(20).op_partial_cmp(&Value::I64(10)), Some(Ordering::Greater));
    assert_eq!(Value::I64(10).op_partial_cmp(&Value::I64(20)), Some(Ordering::Less));

    assert_eq!(Value::U32(10).op_partial_cmp(&Value::U32(10)), Some(Ordering::Equal));
    assert_eq!(Value::U32(20).op_partial_cmp(&Value::U32(10)), Some(Ordering::Greater));
    assert_eq!(Value::U32(10).op_partial_cmp(&Value::U32(20)), Some(Ordering::Less));

    assert_eq!(Value::U64(10).op_partial_cmp(&Value::U64(10)), Some(Ordering::Equal));
    assert_eq!(Value::U64(20).op_partial_cmp(&Value::U64(10)), Some(Ordering::Greater));
    assert_eq!(Value::U64(10).op_partial_cmp(&Value::U64(20)), Some(Ordering::Less));

    assert_eq!(Value::F32(10.0).op_partial_cmp(&Value::F32(10.0)), Some(Ordering::Equal));
    assert_eq!(Value::F32(20.0).op_partial_cmp(&Value::F32(10.0)), Some(Ordering::Greater));
    assert_eq!(Value::F32(10.0).op_partial_cmp(&Value::F32(20.0)), Some(Ordering::Less));

    assert_eq!(Value::F64(10.0).op_partial_cmp(&Value::F64(10.0)), Some(Ordering::Equal));
    assert_eq!(Value::F64(20.0).op_partial_cmp(&Value::F64(10.0)), Some(Ordering::Greater));
    assert_eq!(Value::F64(10.0).op_partial_cmp(&Value::F64(20.0)), Some(Ordering::Less));

    // Value::String
    assert_eq!(Value::String("a".to_string()).op_partial_cmp(&Value::String("a".to_string())), Some(Ordering::Equal));
    assert_eq!(Value::String("b".to_string()).op_partial_cmp(&Value::String("a".to_string())), Some(Ordering::Greater));
    assert_eq!(Value::String("a".to_string()).op_partial_cmp(&Value::String("b".to_string())), Some(Ordering::Less));

    // 跨类型比较
    assert_eq!(Value::I32(10).op_partial_cmp(&Value::I64(10)), Some(Ordering::Equal));
    assert_eq!(Value::I32(10).op_partial_cmp(&Value::F64(10.0)), Some(Ordering::Equal));
    assert_eq!(Value::U32(10).op_partial_cmp(&Value::I64(10)), Some(Ordering::Equal));
}

#[test]
fn test_value_cmp_primitive() {
    // Value vs u64
    assert_eq!(Value::U64(10).op_partial_cmp(&10u64), Some(Ordering::Equal));
    assert_eq!(Value::U64(20).op_partial_cmp(&10u64), Some(Ordering::Greater));
    assert_eq!(Value::U64(10).op_partial_cmp(&20u64), Some(Ordering::Less));

    // Value vs i64
    assert_eq!(Value::I64(10).op_partial_cmp(&10i64), Some(Ordering::Equal));
    assert_eq!(Value::I64(20).op_partial_cmp(&10i64), Some(Ordering::Greater));
    assert_eq!(Value::I64(10).op_partial_cmp(&20i64), Some(Ordering::Less));

    // Value vs f64
    assert_eq!(Value::F64(10.0).op_partial_cmp(&10.0f64), Some(Ordering::Equal));
    assert_eq!(Value::F64(20.0).op_partial_cmp(&10.0f64), Some(Ordering::Greater));
    assert_eq!(Value::F64(10.0).op_partial_cmp(&20.0f64), Some(Ordering::Less));

    // Value vs bool
    assert_eq!(Value::Bool(true).op_partial_cmp(&true), Some(Ordering::Equal));
    assert_eq!(Value::Bool(true).op_partial_cmp(&false), Some(Ordering::Greater));
    assert_eq!(Value::Bool(false).op_partial_cmp(&true), Some(Ordering::Less));

    // Value vs &str
    assert_eq!(Value::String("a".to_string()).op_partial_cmp(&"a"), Some(Ordering::Equal));
    assert_eq!(Value::String("b".to_string()).op_partial_cmp(&"a"), Some(Ordering::Greater));
    assert_eq!(Value::String("a".to_string()).op_partial_cmp(&"b"), Some(Ordering::Less));
}

#[test]
fn test_primitive_cmp_value() {
    println!("{:?}",20i64.op_partial_cmp(&10i64));//Some(Greater)
    
    // u64 vs Value
    assert_eq!(10u64.op_partial_cmp(&Value::U64(10)), Some(Ordering::Equal));
    assert_eq!(20u64.op_partial_cmp(&Value::U64(10)), Some(Ordering::Greater));
    assert_eq!(10u64.op_partial_cmp(&Value::U64(20)), Some(Ordering::Less));

    // i64 vs Value
    assert_eq!(10i64.op_partial_cmp(&Value::I64(10)), Some(Ordering::Equal));
    assert_eq!(20i64.op_partial_cmp(&Value::I64(10)), Some(Ordering::Greater));
    assert_eq!(10i64.op_partial_cmp(&Value::I64(20)), Some(Ordering::Less));
    
    // f64 vs Value
    assert_eq!(10.0f64.op_partial_cmp(&Value::F64(10.0)), Some(Ordering::Equal));
    assert_eq!(20.0f64.op_partial_cmp(&Value::F64(10.0)), Some(Ordering::Greater));
    assert_eq!(10.0f64.op_partial_cmp(&Value::F64(20.0)), Some(Ordering::Less));

    // bool vs Value
    assert_eq!(true.op_partial_cmp(&Value::Bool(true)), Some(Ordering::Equal));
    assert_eq!(true.op_partial_cmp(&Value::Bool(false)), Some(Ordering::Greater));
    assert_eq!(false.op_partial_cmp(&Value::Bool(true)), Some(Ordering::Less));
    assert_eq!(false.op_partial_cmp(&Value::Bool(false)), Some(Ordering::Equal));

    // &str vs Value
    assert_eq!("a".op_partial_cmp(&Value::String("a".to_string())), Some(Ordering::Equal));
    assert_eq!("b".op_partial_cmp(&Value::String("a".to_string())), Some(Ordering::Greater));
    assert_eq!("a".op_partial_cmp(&Value::String("b".to_string())), Some(Ordering::Less));
}

#[test]
fn test_string_cmp() {
    // String vs &str
    assert_eq!("a".to_string().op_partial_cmp(&"a"), Some(Ordering::Equal));
    assert_eq!("b".to_string().op_partial_cmp(&"a"), Some(Ordering::Greater));
    assert_eq!("a".to_string().op_partial_cmp(&"b"), Some(Ordering::Less));

    // String vs String
    assert_eq!("a".to_string().op_partial_cmp(&"a".to_string()), Some(Ordering::Equal));
    assert_eq!("b".to_string().op_partial_cmp(&"a".to_string()), Some(Ordering::Greater));
    assert_eq!("a".to_string().op_partial_cmp(&"b".to_string()), Some(Ordering::Less));

    // &String vs &str
    let a_string = "a".to_string();
    let b_string = "b".to_string();
    assert_eq!((&a_string).op_partial_cmp(&"a"), Some(Ordering::Equal));
    assert_eq!((&b_string).op_partial_cmp(&"a"), Some(Ordering::Greater));
    assert_eq!((&a_string).op_partial_cmp(&"b"), Some(Ordering::Less));

    // &String vs String
    assert_eq!((&a_string).op_partial_cmp(&"a".to_string()), Some(Ordering::Equal));
    assert_eq!((&b_string).op_partial_cmp(&"a".to_string()), Some(Ordering::Greater));
    assert_eq!((&a_string).op_partial_cmp(&"b".to_string()), Some(Ordering::Less));

    // &String vs &String
    assert_eq!((&a_string).op_partial_cmp(&&a_string), Some(Ordering::Equal));
    assert_eq!((&b_string).op_partial_cmp(&&a_string), Some(Ordering::Greater));
    assert_eq!((&a_string).op_partial_cmp(&&b_string), Some(Ordering::Less));
}

#[test]
fn test_numeric_cmp() {
    // 只测试同类型之间的比较，这些比较已经在self_cmp!宏中实现
    // u8, u16, u32, u64
    assert_eq!(10u8.op_partial_cmp(&10u8), Some(Ordering::Equal));
    assert_eq!(10u16.op_partial_cmp(&10u16), Some(Ordering::Equal));
    assert_eq!(10u32.op_partial_cmp(&10u32), Some(Ordering::Equal));
    assert_eq!(10u64.op_partial_cmp(&10u64), Some(Ordering::Equal));

    // i8, i16, i32, i64, isize, usize
    assert_eq!(10i8.op_partial_cmp(&10i8), Some(Ordering::Equal));
    assert_eq!(10i16.op_partial_cmp(&10i16), Some(Ordering::Equal));
    assert_eq!(10i32.op_partial_cmp(&10i32), Some(Ordering::Equal));
    assert_eq!(10i64.op_partial_cmp(&10i64), Some(Ordering::Equal));
    assert_eq!(10isize.op_partial_cmp(&10isize), Some(Ordering::Equal));
    assert_eq!(10usize.op_partial_cmp(&10usize), Some(Ordering::Equal));

    // f32, f64
    assert_eq!(10.0f32.op_partial_cmp(&10.0f32), Some(Ordering::Equal));
    assert_eq!(10.0f64.op_partial_cmp(&10.0f64), Some(Ordering::Equal));
}

#[test]
fn test_cross_type_cmp() {
    // 只测试在cmp_diff!宏中明确实现的跨类型比较
    // 这些测试使用引用版本
    let i64_val = 10i64;
    let i64_ref = &i64_val;
    
    // &i64 vs f64
    assert_eq!(i64_ref.op_partial_cmp(&10.0f64), Some(Ordering::Equal));
    
    // &i64 vs u64
    assert_eq!(i64_ref.op_partial_cmp(&10u64), Some(Ordering::Equal));
    
    // u64与其他类型的比较
    let u64_val = 10u64;
    let u64_ref = &u64_val;
    
    // &u64 vs i64
    assert_eq!(u64_ref.op_partial_cmp(&10i64), Some(Ordering::Equal));
    
    // &u64 vs f64
    assert_eq!(u64_ref.op_partial_cmp(&10.0f64), Some(Ordering::Equal));
    
    // f64与其他类型的比较
    let f64_val = 10.0f64;
    let f64_ref = &f64_val;
    
    // &f64 vs i64
    assert_eq!(f64_ref.op_partial_cmp(&10i64), Some(Ordering::Equal));
    
    // &f64 vs u64
    assert_eq!(f64_ref.op_partial_cmp(&10u64), Some(Ordering::Equal));
}

#[test]
fn test_reference_variants() {
    // 测试Value与原始类型的各种引用形式比较
    let val_i64 = 10i64;
    let val_u64 = 10u64;
    let val_f64 = 10.0f64;
    let val_bool = true;
    let val_str = "test";
    let val_string = "test".to_string();
    let value = Value::I64(10);
    
    // Value vs primitive
    assert_eq!(Value::I64(10).op_partial_cmp(&val_i64), Some(Ordering::Equal));
    assert_eq!(Value::U64(10).op_partial_cmp(&val_u64), Some(Ordering::Equal));
    assert_eq!(Value::F64(10.0).op_partial_cmp(&val_f64), Some(Ordering::Equal));
    assert_eq!(Value::Bool(true).op_partial_cmp(&val_bool), Some(Ordering::Equal));
    assert_eq!(Value::String("test".to_string()).op_partial_cmp(&val_str), Some(Ordering::Equal));
    
    // Value vs &primitive
    assert_eq!(Value::I64(10).op_partial_cmp(&&val_i64), Some(Ordering::Equal));
    assert_eq!(Value::U64(10).op_partial_cmp(&&val_u64), Some(Ordering::Equal));
    assert_eq!(Value::F64(10.0).op_partial_cmp(&&val_f64), Some(Ordering::Equal));
    assert_eq!(Value::Bool(true).op_partial_cmp(&&val_bool), Some(Ordering::Equal));
    assert_eq!(Value::String("test".to_string()).op_partial_cmp(&&val_str), Some(Ordering::Equal));
    
    // &Value vs primitive
    assert_eq!((&value).op_partial_cmp(&val_i64), Some(Ordering::Equal));
    
    // &Value vs &primitive
    assert_eq!((&value).op_partial_cmp(&&val_i64), Some(Ordering::Equal));
    
    // primitive vs Value
    assert_eq!(val_i64.op_partial_cmp(&value), Some(Ordering::Equal));
    assert_eq!(val_u64.op_partial_cmp(&Value::U64(10)), Some(Ordering::Equal));
    assert_eq!(val_f64.op_partial_cmp(&Value::F64(10.0)), Some(Ordering::Equal));
    assert_eq!(val_bool.op_partial_cmp(&Value::Bool(true)), Some(Ordering::Equal));
    assert_eq!(val_str.op_partial_cmp(&Value::String("test".to_string())), Some(Ordering::Equal));
    
    // primitive vs &Value
    assert_eq!(val_i64.op_partial_cmp(&&value), Some(Ordering::Equal));
    
    // &primitive vs Value
    assert_eq!((&val_i64).op_partial_cmp(&value), Some(Ordering::Equal));
    
    // &primitive vs &Value
    assert_eq!((&val_i64).op_partial_cmp(&&value), Some(Ordering::Equal));
    
    // String引用相关测试 - 确保只测试已实现的比较
    // String vs &str
    assert_eq!(val_string.op_partial_cmp(&val_str), Some(Ordering::Equal));
    
    // String vs String
    assert_eq!(val_string.op_partial_cmp(&"test".to_string()), Some(Ordering::Equal));
    
    // &String vs &str
    assert_eq!((&val_string).op_partial_cmp(&val_str), Some(Ordering::Equal));
    
    // &String vs String
    assert_eq!((&val_string).op_partial_cmp(&val_string), Some(Ordering::Equal));
    
    // &String vs &String
    assert_eq!((&val_string).op_partial_cmp(&&val_string), Some(Ordering::Equal));
    
    // &&String vs String
    assert_eq!((&&val_string).op_partial_cmp(&val_string), Some(Ordering::Equal));
    
    // &&String vs &String
    assert_eq!((&&val_string).op_partial_cmp(&&val_string), Some(Ordering::Equal));
}

#[test]
fn test_bool_comparison() {
    // 直接打印结果
    let result = false.op_partial_cmp(&Value::Bool(true));
    println!("false.op_partial_cmp(&Value::Bool(true)) = {:?}", result);
    
    // 反向比较
    let result2 = Value::Bool(true).op_partial_cmp(&false);
    println!("Value::Bool(true).op_partial_cmp(&false) = {:?}", result2);
}

#[test]
fn test_primitive_cmp_value_debug() {
    // 打印所有bool比较的结果
    println!("true.op_partial_cmp(&Value::Bool(true)) = {:?}", true.op_partial_cmp(&Value::Bool(true)));
    println!("true.op_partial_cmp(&Value::Bool(false)) = {:?}", true.op_partial_cmp(&Value::Bool(false)));
    println!("false.op_partial_cmp(&Value::Bool(true)) = {:?}", false.op_partial_cmp(&Value::Bool(true)));
    println!("false.op_partial_cmp(&Value::Bool(false)) = {:?}", false.op_partial_cmp(&Value::Bool(false)));
}

#[test]
fn test_number_cmp_value_debug() {
    // 打印i64与Value比较的结果
    println!("10i64.op_partial_cmp(&Value::I64(10)) = {:?}", 10i64.op_partial_cmp(&Value::I64(10)));
    println!("20i64.op_partial_cmp(&Value::I64(10)) = {:?}", 20i64.op_partial_cmp(&Value::I64(10)));
    println!("10i64.op_partial_cmp(&Value::I64(20)) = {:?}", 10i64.op_partial_cmp(&Value::I64(20)));
    
    // 打印u64与Value比较的结果
    println!("10u64.op_partial_cmp(&Value::U64(10)) = {:?}", 10u64.op_partial_cmp(&Value::U64(10)));
    println!("20u64.op_partial_cmp(&Value::U64(10)) = {:?}", 20u64.op_partial_cmp(&Value::U64(10)));
    println!("10u64.op_partial_cmp(&Value::U64(20)) = {:?}", 10u64.op_partial_cmp(&Value::U64(20)));
    
    // 打印f64与Value比较的结果
    println!("10.0f64.op_partial_cmp(&Value::F64(10.0)) = {:?}", 10.0f64.op_partial_cmp(&Value::F64(10.0)));
    println!("20.0f64.op_partial_cmp(&Value::F64(10.0)) = {:?}", 20.0f64.op_partial_cmp(&Value::F64(10.0)));
    println!("10.0f64.op_partial_cmp(&Value::F64(20.0)) = {:?}", 10.0f64.op_partial_cmp(&Value::F64(20.0)));
}

#[test]
fn test_string_cmp_value_debug() {
    // 打印&str与Value比较的结果
    println!("\"a\".op_partial_cmp(&Value::String(\"a\".to_string())) = {:?}", "a".op_partial_cmp(&Value::String("a".to_string())));
    println!("\"b\".op_partial_cmp(&Value::String(\"a\".to_string())) = {:?}", "b".op_partial_cmp(&Value::String("a".to_string())));
    println!("\"a\".op_partial_cmp(&Value::String(\"b\".to_string())) = {:?}", "a".op_partial_cmp(&Value::String("b".to_string())));
}

#[test]
fn test_bool_value_comparison_comprehensive() {
    // 测试 Value::Bool 与 bool 之间的比较
    // Value::Bool -> bool 方向
    assert_eq!(Value::Bool(true).op_partial_cmp(&true), Some(Ordering::Equal));
    assert_eq!(Value::Bool(true).op_partial_cmp(&false), Some(Ordering::Greater));
    assert_eq!(Value::Bool(false).op_partial_cmp(&true), Some(Ordering::Less));
    assert_eq!(Value::Bool(false).op_partial_cmp(&false), Some(Ordering::Equal));
    
    // bool -> Value::Bool 方向
    assert_eq!(true.op_partial_cmp(&Value::Bool(true)), Some(Ordering::Equal));
    assert_eq!(true.op_partial_cmp(&Value::Bool(false)), Some(Ordering::Greater));
    assert_eq!(false.op_partial_cmp(&Value::Bool(true)), Some(Ordering::Less));
    assert_eq!(false.op_partial_cmp(&Value::Bool(false)), Some(Ordering::Equal));
    
    // 带引用版本 &Value::Bool -> bool
    let true_value = Value::Bool(true);
    let false_value = Value::Bool(false);
    assert_eq!((&true_value).op_partial_cmp(&true), Some(Ordering::Equal));
    assert_eq!((&true_value).op_partial_cmp(&false), Some(Ordering::Greater));
    assert_eq!((&false_value).op_partial_cmp(&true), Some(Ordering::Less));
    assert_eq!((&false_value).op_partial_cmp(&false), Some(Ordering::Equal));
    
    // 带引用版本 bool -> &Value::Bool
    assert_eq!(true.op_partial_cmp(&&true_value), Some(Ordering::Equal));
    assert_eq!(true.op_partial_cmp(&&false_value), Some(Ordering::Greater));
    assert_eq!(false.op_partial_cmp(&&true_value), Some(Ordering::Less));
    assert_eq!(false.op_partial_cmp(&&false_value), Some(Ordering::Equal));
    
    // 带引用版本 &bool -> Value::Bool
    let true_bool = true;
    let false_bool = false;
    assert_eq!((&true_bool).op_partial_cmp(&true_value), Some(Ordering::Equal));
    assert_eq!((&true_bool).op_partial_cmp(&false_value), Some(Ordering::Greater));
    assert_eq!((&false_bool).op_partial_cmp(&true_value), Some(Ordering::Less));
    assert_eq!((&false_bool).op_partial_cmp(&false_value), Some(Ordering::Equal));
    
    // 带引用版本 &bool -> &Value::Bool
    assert_eq!((&true_bool).op_partial_cmp(&&true_value), Some(Ordering::Equal));
    assert_eq!((&true_bool).op_partial_cmp(&&false_value), Some(Ordering::Greater));
    assert_eq!((&false_bool).op_partial_cmp(&&true_value), Some(Ordering::Less));
    assert_eq!((&false_bool).op_partial_cmp(&&false_value), Some(Ordering::Equal));
}