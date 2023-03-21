#[cfg(test)]
mod test {
    use rbs::value::map::ValueMap;
    use rbs::{value_map, Value};

    #[test]
    fn test_decode_value() {
        let m = value_map! {
            1.to_string() => 1,
            2.to_string() => 2,
        };
        let m = Value::Map(m);
        let v: Value = rbatis::decode(Value::Array(vec![m.clone()])).unwrap();
        assert_eq!(v, Value::Array(vec![m]));
    }

    #[test]
    fn test_decode_one() {
        let date = rbdc::types::datetime::FastDateTime::now();
        let m = value_map! {
            1.to_string() => date.clone(),
        };
        let v: rbdc::types::datetime::FastDateTime =
            rbatis::decode(Value::Array(vec![Value::Map(m)])).unwrap();
        assert_eq!(v, date);
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
    fn test_decode_json_array() {
        let m = value_map! {
            1.to_string() => 1,
            2.to_string() => 2,
        };
        let m = Value::Map(m);
        let v: serde_json::Value =
            rbatis::decode(Value::Array(vec![m.clone(), m.clone()])).unwrap();
        assert_eq!(
            v,
            serde_json::from_str::<serde_json::Value>(r#"[{"1":1,"2":2},{"1":1,"2":2}]"#).unwrap()
        );
    }
}
