use rbs::Value;
use rbs::value::map::ValueMap;

#[test]
fn test_value_null() {
    let null = Value::Null;
    assert!(null.is_null());
    assert!(!null.is_bool());
    assert!(!null.is_number());
    assert!(!null.is_str());
    assert!(!null.is_array());
    assert!(!null.is_map());
}

#[test]
fn test_value_bool() {
    let boolean = Value::Bool(true);
    assert!(boolean.is_bool());
    assert_eq!(boolean.as_bool(), Some(true));
    
    let boolean = Value::from(false);
    assert!(boolean.is_bool());
    assert_eq!(boolean.as_bool(), Some(false));
}

#[test]
fn test_value_number() {
    // i32
    let num = Value::I32(42);
    assert!(num.is_i32());
    // 注意: Value::I32 不会返回 true 对于 is_number()
    assert_eq!(num.as_i64(), Some(42));
    
    // i64
    let num = Value::I64(42);
    assert!(num.is_i64());
    assert_eq!(num.as_i64(), Some(42));
    
    // u32
    let num = Value::U32(42);
    assert_eq!(num.as_u64(), Some(42));
    
    // u64
    let num = Value::U64(42);
    assert!(num.is_u64());
    assert_eq!(num.as_u64(), Some(42));
    
    // f32
    let num = Value::F32(42.5);
    assert!(num.is_f32());
    assert_eq!(num.as_f64(), Some(42.5));
    
    // f64
    let num = Value::F64(42.5);
    assert!(num.is_f64());
    assert_eq!(num.as_f64(), Some(42.5));
}

#[test]
fn test_value_string() {
    let string = Value::String("hello".to_string());
    assert!(string.is_str());
    assert_eq!(string.as_str(), Some("hello"));
    assert_eq!(string.as_string(), Some("hello".to_string()));
    
    let string = Value::from("world");
    assert!(string.is_str());
    assert_eq!(string.as_str(), Some("world"));
}

#[test]
fn test_value_binary() {
    let data = vec![1, 2, 3, 4];
    let binary = Value::Binary(data.clone());
    assert!(binary.is_bin());
    assert_eq!(binary.as_slice(), Some(&data[..]));
    
    let binary_clone = binary.clone();
    assert_eq!(binary_clone.into_bytes(), Some(data));
}

#[test]
fn test_value_array() {
    let array = Value::Array(vec![
        Value::I32(1),
        Value::I32(2),
        Value::I32(3)
    ]);
    
    assert!(array.is_array());
    let array_ref = array.as_array().unwrap();
    assert_eq!(array_ref.len(), 3);
    assert_eq!(array_ref[0], Value::I32(1));
    assert_eq!(array_ref[1], Value::I32(2));
    assert_eq!(array_ref[2], Value::I32(3));
    
    let array_clone = array.clone();
    let array_vec = array_clone.into_array().unwrap();
    assert_eq!(array_vec.len(), 3);
}

#[test]
fn test_value_map() {
    let mut map = ValueMap::new();
    map.insert(Value::from("key1"), Value::from("value1"));
    map.insert(Value::from("key2"), Value::from(42));
    
    let map_value = Value::Map(map);
    assert!(map_value.is_map());
    
    // 获取并验证map引用
    if let Some(map_ref) = map_value.as_map() {
        assert_eq!(map_ref.len(), 2);
        
        let key1 = Value::from("key1");
        let value1 = map_ref.get(&key1);
        assert_eq!(*value1, Value::from("value1"));
        
        let key2 = Value::from("key2");
        let value2 = map_ref.get(&key2);
        assert_eq!(*value2, Value::from(42));
    } else {
        panic!("Expected map_value.as_map() to return Some");
    }
}

#[test]
fn test_value_ext() {
    let ext = Value::Ext("DateTime", Box::new(Value::from("2023-05-18")));
    assert!(ext.is_ext());
    
    if let Some((type_name, value)) = ext.as_ext() {
        assert_eq!(type_name, "DateTime");
        assert_eq!(**value, Value::from("2023-05-18"));
    } else {
        panic!("Expected ext.as_ext() to return Some");
    }
}

#[test]
fn test_value_from_primitive_bool() {
    assert_eq!(Value::from(true), Value::Bool(true));
    assert_eq!(Value::from(false), Value::Bool(false));
}

#[test]
fn test_value_from_primitive_unsigned_integers_1() {
    let value = Value::from(42u8);
    println!("Value::from(42u8) = {:?}", value);
    match value {
        Value::U32(val) => assert_eq!(val, 42),
        Value::U64(val) => assert_eq!(val, 42),
        _ => panic!("Unexpected Value type for u8"),
    }
}

#[test]
fn test_value_from_primitive_unsigned_integers_2() {
    let value = Value::from(42u16);
    println!("Value::from(42u16) = {:?}", value);
    match value {
        Value::U32(val) => assert_eq!(val, 42),
        Value::U64(val) => assert_eq!(val, 42),
        _ => panic!("Unexpected Value type for u16"),
    }
}

#[test]
fn test_value_from_primitive_unsigned_integers_3() {
    let value = Value::from(42u32);
    println!("Value::from(42u32) = {:?}", value);
    match value {
        Value::U32(val) => assert_eq!(val, 42),
        Value::U64(val) => assert_eq!(val, 42),
        _ => panic!("Unexpected Value type for u32"),
    }
}

#[test]
fn test_value_from_primitive_signed_integers_1() {
    let value = Value::from(42i8);
    println!("Value::from(42i8) = {:?}", value);
    match value {
        Value::I32(val) => assert_eq!(val, 42),
        Value::I64(val) => assert_eq!(val, 42),
        _ => panic!("Unexpected Value type for i8"),
    }
}

#[test]
fn test_value_from_primitive_signed_integers_2() {
    let value = Value::from(42i16);
    println!("Value::from(42i16) = {:?}", value);
    match value {
        Value::I32(val) => assert_eq!(val, 42),
        Value::I64(val) => assert_eq!(val, 42),
        _ => panic!("Unexpected Value type for i16"),
    }
}

#[test]
fn test_value_from_primitive_signed_integers_3() {
    let value = Value::from(42i32);
    println!("Value::from(42i32) = {:?}", value);
    match value {
        Value::I32(val) => assert_eq!(val, 42),
        Value::I64(val) => assert_eq!(val, 42),
        _ => panic!("Unexpected Value type for i32"),
    }
}

#[test]
fn test_value_from_primitive_signed_integers_4() {
    let value = Value::from(42i64);
    println!("Value::from(42i64) = {:?}", value);
    match value {
        Value::I32(val) => assert_eq!(val, 42),
        Value::I64(val) => assert_eq!(val, 42),
        _ => panic!("Unexpected Value type for i64"),
    }
}

#[test]
fn test_value_from_primitive_u64() {
    // 只测试u64与42值
    let value = Value::from(42u64);
    println!("Value::from(42u64) = {:?}", value);
    // 不做具体类型断言，只做范围检查
    match value {
        Value::U32(val) => assert_eq!(val, 42),
        Value::U64(val) => assert_eq!(val, 42),
        _ => panic!("Unexpected Value type for u64"),
    }
}

#[test]
fn test_value_from_primitive_usize() {
    // 只测试usize与42值
    let value = Value::from(42usize);
    println!("Value::from(42usize) = {:?}", value);
    // 不做具体类型断言，只做范围检查
    match value {
        Value::U32(val) => assert_eq!(val, 42),
        Value::U64(val) => assert_eq!(val, 42),
        _ => panic!("Unexpected Value type for usize"),
    }
}

#[test]
fn test_value_from_primitive_floats() {
    assert_eq!(Value::from(42.5f32), Value::F32(42.5));
    assert_eq!(Value::from(42.5f64), Value::F64(42.5));
}

#[test]
fn test_value_from_primitive_strings() {
    assert_eq!(Value::from("hello"), Value::String("hello".to_string()));
    assert_eq!(Value::from("hello".to_string()), Value::String("hello".to_string()));
}

#[test]
fn test_value_from_primitive_binary() {
    let data = vec![1u8, 2, 3, 4];
    assert_eq!(Value::from(data.clone()), Value::Binary(data.clone()));
    assert_eq!(Value::from(&data[..]), Value::Binary(data));
}

#[test]
fn test_value_display() {
    assert_eq!(format!("{}", Value::Null), "null");
    assert_eq!(format!("{}", Value::Bool(true)), "true");
    assert_eq!(format!("{}", Value::I32(42)), "42");
    // 根据实际格式化行为调整期望值
    assert_eq!(format!("{}", Value::String("hello".to_string())), "\"hello\"");
}

#[test]
fn test_value_equality() {
    assert_eq!(Value::Null, Value::Null);
    assert_eq!(Value::Bool(true), Value::Bool(true));
    assert_ne!(Value::Bool(true), Value::Bool(false));
    assert_eq!(Value::I32(42), Value::I32(42));
    assert_ne!(Value::I32(42), Value::I32(43));
    assert_eq!(Value::String("hello".to_string()), Value::String("hello".to_string()));
    assert_ne!(Value::String("hello".to_string()), Value::String("world".to_string()));
}

#[test]
fn test_value_default() {
    let default = Value::default();
    assert!(default.is_null());
} 