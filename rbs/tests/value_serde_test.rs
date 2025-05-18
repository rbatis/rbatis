use rbs::{from_value, to_value, Value};
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
struct TestStruct {
    name: String,
    age: i32,
    active: bool,
    data: Option<Vec<i32>>,
}

#[test]
fn test_to_value() {
    // 测试基本类型转换
    assert_eq!(to_value(123).unwrap(), Value::I32(123));
    assert_eq!(to_value("test").unwrap(), Value::String("test".to_string()));
    assert_eq!(to_value(true).unwrap(), Value::Bool(true));
    
    // 测试Option类型
    let opt_some: Option<i32> = Some(123);
    let opt_none: Option<i32> = None;
    
    assert_eq!(to_value(opt_some).unwrap(), Value::I32(123));
    assert_eq!(to_value(opt_none).unwrap(), Value::Null);
    
    // 测试Vec类型
    let vec_data = vec![1, 2, 3];
    let value = to_value(vec_data).unwrap();
    
    if let Value::Array(arr) = value {
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], Value::I32(1));
        assert_eq!(arr[1], Value::I32(2));
        assert_eq!(arr[2], Value::I32(3));
    } else {
        panic!("Expected Array value");
    }
    
    // 测试HashMap类型
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 123);
    map.insert("key2".to_string(), 456);
    
    let value = to_value(map).unwrap();
    if let Value::Map(value_map) = value {
        assert_eq!(value_map.len(), 2);
        
        let mut found_key1 = false;
        let mut found_key2 = false;
        
        for (k, v) in &value_map {
            if k.is_str() && k.as_str().unwrap() == "key1" {
                assert_eq!(v, &Value::I32(123));
                found_key1 = true;
            }
            if k.is_str() && k.as_str().unwrap() == "key2" {
                assert_eq!(v, &Value::I32(456));
                found_key2 = true;
            }
        }
        
        assert!(found_key1, "key1 not found in map");
        assert!(found_key2, "key2 not found in map");
    } else {
        panic!("Expected Map value");
    }
    
    // 测试结构体转换
    let test_struct = TestStruct {
        name: "test".to_string(),
        age: 30,
        active: true,
        data: Some(vec![1, 2, 3]),
    };
    
    let value = to_value(test_struct).unwrap();
    if let Value::Map(value_map) = value {
        assert_eq!(value_map.len(), 4);
        
        // 验证字段
        let mut found_fields = 0;
        
        for (k, v) in &value_map {
            if k.is_str() {
                match k.as_str().unwrap() {
                    "name" => {
                        assert_eq!(v, &Value::String("test".to_string()));
                        found_fields += 1;
                    },
                    "age" => {
                        assert_eq!(v, &Value::I32(30));
                        found_fields += 1;
                    },
                    "active" => {
                        assert_eq!(v, &Value::Bool(true));
                        found_fields += 1;
                    },
                    "data" => {
                        if let Value::Array(arr) = v {
                            assert_eq!(arr.len(), 3);
                            assert_eq!(arr[0], Value::I32(1));
                            assert_eq!(arr[1], Value::I32(2));
                            assert_eq!(arr[2], Value::I32(3));
                            found_fields += 1;
                        }
                    },
                    _ => {}
                }
            }
        }
        
        assert_eq!(found_fields, 4, "Not all fields were found in the map");
    } else {
        panic!("Expected Map value for struct");
    }
}

#[test]
fn test_from_value() {
    // 测试基本类型
    let i: i32 = from_value(Value::I32(123)).unwrap();
    assert_eq!(i, 123);
    
    let s: String = from_value(Value::String("test".to_string())).unwrap();
    assert_eq!(s, "test");
    
    let b: bool = from_value(Value::Bool(true)).unwrap();
    assert_eq!(b, true);
    
    // 测试Option类型
    let some: Option<i32> = from_value(Value::I32(123)).unwrap();
    assert_eq!(some, Some(123));
    
    let none: Option<i32> = from_value(Value::Null).unwrap();
    assert_eq!(none, None);
    
    // 测试Vec类型
    let arr = Value::Array(vec![
        Value::I32(1),
        Value::I32(2),
        Value::I32(3),
    ]);
    
    let vec_result: Vec<i32> = from_value(arr).unwrap();
    assert_eq!(vec_result, vec![1, 2, 3]);
    
    // 测试结构体反序列化
    let mut map = rbs::value::map::ValueMap::new();
    map.insert(Value::String("name".to_string()), Value::String("test".to_string()));
    map.insert(Value::String("age".to_string()), Value::I32(30));
    map.insert(Value::String("active".to_string()), Value::Bool(true));
    
    let data_arr = Value::Array(vec![
        Value::I32(1),
        Value::I32(2),
        Value::I32(3),
    ]);
    
    map.insert(Value::String("data".to_string()), data_arr);
    
    let value = Value::Map(map);
    let test_struct: TestStruct = from_value(value).unwrap();
    
    assert_eq!(test_struct, TestStruct {
        name: "test".to_string(),
        age: 30,
        active: true,
        data: Some(vec![1, 2, 3]),
    });
}

#[test]
fn test_roundtrip() {
    // 测试从结构体到Value再回到结构体
    let original = TestStruct {
        name: "roundtrip".to_string(),
        age: 42,
        active: false,
        data: Some(vec![4, 5, 6]),
    };
    
    let value = to_value(original.clone()).unwrap();
    let roundtrip: TestStruct = from_value(value).unwrap();
    
    assert_eq!(original, roundtrip);
} 