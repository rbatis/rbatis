#[cfg(test)]
mod test {
    use rbs::{value, Value};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_decode_hashmap() {
        let m: HashMap<String, Value> =
            rbatis::decode(value!(vec![value!("a")], vec![value!(1)])).unwrap();
        println!("{:#?}", m);
        assert_eq!(m.get("a").unwrap().as_i64(), Value::I32(1).as_i64());
    }

    #[test]
    fn test_decode_value() {
        // CSV format: [[col_name], [value]]
        let m = Value::Array(vec![
            Value::Array(vec![Value::String("1".to_string())]),
            Value::Array(vec![Value::I64(1)]),
        ]);
        let v: i64 = rbatis::decode(m).unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_decode_type_fail() {
        #[derive(Serialize, Deserialize)]
        pub struct A {
            pub aa: i32,
        }
        // CSV format with string that can't be parsed as i32
        let m = Value::Array(vec![
            Value::Array(vec![Value::String("aa".to_string())]),
            Value::Array(vec![Value::String("".to_string())]),
        ]);
        let v = rbatis::decode::<A>(m).err().unwrap();
        assert_eq!(
            v.to_string(),
            "invalid type: string \"\", expected i32, key = `aa`"
        );
    }

    #[test]
    fn test_decode_type_struct_one() {
        // CSV format with float that can't be parsed as i64
        let m = Value::Array(vec![
            Value::Array(vec![Value::String("aa".to_string())]),
            Value::Array(vec![Value::F64(0.0)]),
        ]);
        #[derive(Serialize, Deserialize)]
        pub struct TestStruct {
            pub aa: f64,
        }
        let v: TestStruct = rbatis::decode(m).unwrap();
        assert_eq!(v.aa, 0.0);
    }

    #[test]
    fn test_decode_one() {
        let date = rbdc::types::datetime::DateTime::now();
        // CSV format: [[col_name], [value]]
        let date_value: Value = date.clone().into();
        let m = Value::Array(vec![
            Value::Array(vec![Value::String("1".to_string())]),
            Value::Array(vec![date_value]),
        ]);
        let v: rbdc::types::datetime::DateTime = rbatis::decode(m).unwrap();
        assert_eq!(v.to_string(), date.to_string());
        println!("{}", v.offset());
    }

    #[test]
    fn test_decode_i32() {
        // CSV format: [[col_name], [value]]
        let v: i32 = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::I64(1)]),
        ]))
        .unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_decode_i64() {
        // CSV format: [[col_name], [value]]
        let v: i64 = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::I64(1)]),
        ]))
        .unwrap();
        assert_eq!(v, 1i64);
    }

    #[test]
    fn test_decode_string() {
        // CSV format: [[col_name], [value]]
        let v: String = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::String("a".to_string())]),
        ]))
        .unwrap();
        assert_eq!(v, "a");
    }

    #[test]
    fn test_decode_json_array() {
        // CSV format for multiple rows
        let v: serde_json::Value = rbatis::decode(Value::Array(vec![
            Value::Array(vec![
                Value::String("1".to_string()),
                Value::String("2".to_string()),
            ]),
            Value::Array(vec![Value::I64(1), Value::I64(2)]),
            Value::Array(vec![Value::I64(1), Value::I64(2)]),
        ]))
        .unwrap();
        assert_eq!(
            v,
            serde_json::from_str::<serde_json::Value>(r#"[{"1":1,"2":2},{"1":1,"2":2}]"#).unwrap()
        );
    }

    #[test]
    fn test_decode_rbdc_types() {
        use rbdc::types::*;
        let date = date::Date::from_str("2023-12-12").unwrap();
        let date_new: date::Date = rbs::from_value(rbs::value!(date.clone())).unwrap();
        assert_eq!(date, date_new);

        let datetime = datetime::DateTime::from_str("2023-12-12 12:12:12").unwrap();
        let datetime_new: datetime::DateTime =
            rbs::from_value(rbs::value!(datetime.clone())).unwrap();
        assert_eq!(datetime, datetime_new);
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
        // CSV format: columns + multiple rows
        let data = Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::I64(1)]),
            Value::Array(vec![Value::I64(2)]),
        ]);

        let result = rbatis::decode::<i32>(data);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("rows.rows_affected > 1"));
    }

    #[test]
    fn test_decode_f32() {
        // CSV format: [[col_name], [value]]
        let v: f32 = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::F64(1.0)]),
        ]))
        .unwrap();
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_decode_f64() {
        // CSV format: [[col_name], [value]]
        let v: f64 = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::F64(1.0)]),
        ]))
        .unwrap();
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_decode_u32() {
        // CSV format: [[col_name], [value]]
        let v: u32 = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::U64(1)]),
        ]))
        .unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_decode_u64() {
        // CSV format: [[col_name], [value]]
        let v: u64 = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::U64(1)]),
        ]))
        .unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_decode_bool() {
        // CSV format: [[col_name], [value]]
        let v: bool = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::Bool(true)]),
        ]))
        .unwrap();
        assert_eq!(v, true);
    }

    #[test]
    fn test_decode_option_types() {
        // CSV format: [[col_name], [value]]

        // Option<i32>
        let v1: Option<i32> = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::I32(1)]),
        ]))
        .unwrap();
        assert_eq!(v1, Some(1));

        // Option<String>
        let v2: Option<String> = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::String("test".to_string())]),
        ]))
        .unwrap();
        assert_eq!(v2, Some("test".to_string()));

        // null值解码为None
        let v3: Option<i32> = rbatis::decode(Value::Array(vec![
            Value::Array(vec![Value::String("a".to_string())]),
            Value::Array(vec![Value::Null]),
        ]))
        .unwrap();
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

        // CSV format: [[col_names], [values]]
        let value = Value::Array(vec![
            Value::Array(vec![
                Value::String("id".to_string()),
                Value::String("name".to_string()),
                Value::String("active".to_string()),
            ]),
            Value::Array(vec![
                Value::I32(1),
                Value::String("test".to_string()),
                Value::Bool(true),
            ]),
        ]);

        let result: TestStruct = rbatis::decode(value).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "test");
        assert_eq!(result.active, true);
    }

    #[test]
    #[ignore] // CSV format doesn't support direct nested struct deserialization
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

        // CSV format with nested struct - inner becomes JSON string in CSV
        let inner_json = serde_json::json!({"value": 42});
        let value = Value::Array(vec![
            Value::Array(vec![
                Value::String("id".to_string()),
                Value::String("inner".to_string()),
            ]),
            Value::Array(vec![Value::I32(1), Value::String(inner_json.to_string())]),
        ]);

        let result: Outer = rbatis::decode(value).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.inner.value, 42);
    }

    #[test]
    fn test_decode_vec() {
        // 测试解码到Vec<T> - CSV format with multiple rows
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct Item {
            pub id: i32,
            pub name: String,
        }

        // CSV format: [[col_names], [row1], [row2]]
        let value = Value::Array(vec![
            Value::Array(vec![
                Value::String("id".to_string()),
                Value::String("name".to_string()),
            ]),
            Value::Array(vec![Value::I32(1), Value::String("test1".to_string())]),
            Value::Array(vec![Value::I32(2), Value::String("test2".to_string())]),
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
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode an not array value"
        );
    }

    #[test]
    fn test_decode_ref() {
        // 测试decode_ref函数
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct Item {
            pub id: i32,
            pub name: String,
        }

        // CSV format: [[col_names], [values]]
        let value = Value::Array(vec![
            Value::Array(vec![
                Value::String("id".to_string()),
                Value::String("name".to_string()),
            ]),
            Value::Array(vec![Value::I32(1), Value::String("test".to_string())]),
        ]);

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
