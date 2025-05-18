#[cfg(test)]
mod test {
    use rbs::value::map::ValueMap;
    use rbs::{to_value, Value};
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;
    use std::collections::HashMap;

    #[test]
    fn test_decode_value() {
        let m = Value::Array(vec![to_value! {
            "1": 1
        }]);
        let v: i64 = rbatis::decode(m).unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_decode_type_fail() {
        #[derive(Serialize, Deserialize)]
        pub struct A {
            pub aa: i32,
        }
        let m = Value::Array(vec![to_value! {
            "aa": ""
        }]);
        let v = rbatis::decode::<A>(m).err().unwrap();
        assert_eq!(
            v.to_string(),
            "invalid type: string \"\", expected i32, key = `aa`"
        );
    }

    //https://github.com/rbatis/rbatis/issues/498
    #[test]
    fn test_decode_type_fail_498() {
        let m = Value::Array(vec![to_value! {
            "aa": 0.0
        }]);
        let v = rbatis::decode::<i64>(m).err().unwrap();
        assert_eq!(
            v.to_string(),
            "invalid type: floating point `0.0`, expected i64"
        );
    }

    #[test]
    fn test_decode_one() {
        let date = rbdc::types::datetime::DateTime::now();
        let m = to_value! {
            "1" : date.clone(),
        };
        let v: rbdc::types::datetime::DateTime = rbatis::decode(Value::Array(vec![m])).unwrap();
        assert_eq!(v.to_string(), date.to_string());
        println!("{}", v.offset());
    }

    #[test]
    fn test_decode_i32() {
        let v: i32 = rbatis::decode(Value::Array(vec![Value::Map({
            let mut m = ValueMap::new();
            m.insert(Value::String("a".to_string()), Value::I64(1));
            m
        })]))
        .unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_decode_i64() {
        let v: i64 = rbatis::decode(Value::Array(vec![Value::Map({
            let mut m = ValueMap::new();
            m.insert(Value::String("a".to_string()), Value::I64(1));
            m
        })]))
        .unwrap();
        assert_eq!(v, 1i64);
    }

    #[test]
    fn test_decode_string() {
        let v: String = rbatis::decode(Value::Array(vec![to_value! {
            "a":"a",
        }]))
        .unwrap();
        assert_eq!(v, "a");
    }

    #[test]
    fn test_decode_json_array() {
        let m = to_value! {
            "1" : 1,
            "2" : 2,
        };
        let v: serde_json::Value =
            rbatis::decode(Value::Array(vec![m.clone(), m.clone()])).unwrap();
        assert_eq!(
            v,
            serde_json::from_str::<serde_json::Value>(r#"[{"1":1,"2":2},{"1":1,"2":2}]"#).unwrap()
        );
    }

    #[test]
    fn test_decode_rbdc_types() {
        use rbdc::types::*;
        let date = date::Date::from_str("2023-12-12").unwrap();
        let date_new: date::Date = rbs::from_value(rbs::to_value!(date.clone())).unwrap();
        assert_eq!(date, date_new);

        let datetime = datetime::DateTime::from_str("2023-12-12 12:12:12").unwrap();
        let datetime_new: datetime::DateTime =
            rbs::from_value(rbs::to_value!(datetime.clone())).unwrap();
        assert_eq!(datetime, datetime_new);
    }
    
    #[test]
    fn test_decode_hashmap() {
        let mut v = ValueMap::new();
        v.insert(Value::String("key".to_string()), Value::I32(2));
        let m: HashMap<String, i32> = rbatis::decode(Value::Array(vec![Value::Map(v)])).unwrap();
        assert_eq!(*m.get("key").unwrap(), 2);
    }
    
    #[test]
    fn test_decode_empty_array() {
        // 测试空数组的解码
        let empty_array = Value::Array(vec![]);
        let result: Option<i32> = rbatis::decode(empty_array).unwrap();
        assert_eq!(result, None);
    }
    
    #[test]
    fn test_decode_multiple_rows_to_single_type() {
        // 测试解码多行数据到单一类型的情况（应当返回错误）
        let data = Value::Array(vec![
            to_value!{ "a": 1 },
            to_value!{ "b": 2 }
        ]);
        
        let result = rbatis::decode::<i32>(data);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("rows.rows_affected > 1"));
    }
    
    #[test]
    fn test_decode_f32() {
        let v: f32 = rbatis::decode(Value::Array(vec![Value::Map({
            let mut m = ValueMap::new();
            m.insert(Value::String("a".to_string()), Value::F64(1.0));
            m
        })]))
        .unwrap();
        assert_eq!(v, 1.0);
    }
    
    #[test]
    fn test_decode_f64() {
        let v: f64 = rbatis::decode(Value::Array(vec![Value::Map({
            let mut m = ValueMap::new();
            m.insert(Value::String("a".to_string()), Value::F64(1.0));
            m
        })]))
        .unwrap();
        assert_eq!(v, 1.0);
    }
    
    #[test]
    fn test_decode_u32() {
        let v: u32 = rbatis::decode(Value::Array(vec![Value::Map({
            let mut m = ValueMap::new();
            m.insert(Value::String("a".to_string()), Value::U64(1));
            m
        })]))
        .unwrap();
        assert_eq!(v, 1);
    }
    
    #[test]
    fn test_decode_u64() {
        let v: u64 = rbatis::decode(Value::Array(vec![Value::Map({
            let mut m = ValueMap::new();
            m.insert(Value::String("a".to_string()), Value::U64(1));
            m
        })]))
        .unwrap();
        assert_eq!(v, 1);
    }
    
    #[test]
    fn test_decode_bool() {
        let v: bool = rbatis::decode(Value::Array(vec![Value::Map({
            let mut m = ValueMap::new();
            m.insert(Value::String("a".to_string()), Value::Bool(true));
            m
        })]))
        .unwrap();
        assert_eq!(v, true);
    }
    
    #[test]
    fn test_decode_option_types() {
        // 测试Option<T>类型的解码
        let test_map = |value: Value| -> ValueMap {
            let mut m = ValueMap::new();
            m.insert(Value::String("a".to_string()), value);
            m
        };
        
        // Option<i32>
        let v1: Option<i32> = rbatis::decode(Value::Array(vec![
            Value::Map(test_map(Value::I32(1)))
        ])).unwrap();
        assert_eq!(v1, Some(1));
        
        // Option<String>
        let v2: Option<String> = rbatis::decode(Value::Array(vec![
            Value::Map(test_map(Value::String("test".to_string())))
        ])).unwrap();
        assert_eq!(v2, Some("test".to_string()));
        
        // null值解码为None
        let v3: Option<i32> = rbatis::decode(Value::Array(vec![
            Value::Map(test_map(Value::Null))
        ])).unwrap();
        assert_eq!(v3, None);
    }
    
    #[test]
    fn test_decode_struct() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct TestStruct {
            pub id: i32,
            pub name: String,
            pub active: bool,
        }
        
        let mut value_map = ValueMap::new();
        value_map.insert(Value::String("id".to_string()), Value::I32(1));
        value_map.insert(Value::String("name".to_string()), Value::String("test".to_string()));
        value_map.insert(Value::String("active".to_string()), Value::Bool(true));
        
        let value = Value::Array(vec![Value::Map(value_map)]);
        
        let result: TestStruct = rbatis::decode(value).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "test");
        assert_eq!(result.active, true);
    }
    
    #[test]
    fn test_decode_nested_struct() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct Inner {
            pub value: i32,
        }
        
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct Outer {
            pub id: i32,
            pub inner: Inner,
        }
        
        // 手动构建嵌套结构
        let mut inner_map = ValueMap::new();
        inner_map.insert(Value::String("value".to_string()), Value::I32(42));
        
        let mut outer_map = ValueMap::new();
        outer_map.insert(Value::String("id".to_string()), Value::I32(1));
        outer_map.insert(Value::String("inner".to_string()), Value::Map(inner_map));
        
        let value = Value::Array(vec![Value::Map(outer_map)]);
        
        let result: Outer = rbatis::decode(value).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.inner.value, 42);
    }
    
    #[test]
    fn test_decode_vec() {
        // 测试解码到Vec<T>
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct Item {
            pub id: i32,
            pub name: String,
        }
        
        let mut item1 = ValueMap::new();
        item1.insert(Value::String("id".to_string()), Value::I32(1));
        item1.insert(Value::String("name".to_string()), Value::String("test1".to_string()));
        
        let mut item2 = ValueMap::new();
        item2.insert(Value::String("id".to_string()), Value::I32(2));
        item2.insert(Value::String("name".to_string()), Value::String("test2".to_string()));
        
        let value = Value::Array(vec![
            Value::Map(item1),
            Value::Map(item2)
        ]);
        
        let result: Vec<Item> = rbatis::decode(value).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "test1");
        assert_eq!(result[1].id, 2);
        assert_eq!(result[1].name, "test2");
    }
    
    #[test]
    fn test_decode_not_array() {
        // 测试解码非数组值的情况
        let value = Value::I32(1);
        let result = rbatis::decode::<i32>(value);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "decode an not array value");
    }
    
    #[test]
    fn test_decode_ref() {
        // 测试decode_ref函数
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct Item {
            pub id: i32,
            pub name: String,
        }
        
        let mut item_map = ValueMap::new();
        item_map.insert(Value::String("id".to_string()), Value::I32(1));
        item_map.insert(Value::String("name".to_string()), Value::String("test".to_string()));
        
        let value = Value::Array(vec![Value::Map(item_map)]);
        
        let result: Item = rbatis::decode::decode_ref(&value).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "test");
    }
    
    #[test]
    fn test_is_debug_mode() {
        // 测试is_debug_mode函数
        let _debug_mode = rbatis::decode::is_debug_mode();
        // 这里我们不断言具体值，因为它依赖于编译模式和特性开启状态
    }
} 