#[cfg(test)]
mod test {
    use rbs::value::map::ValueMap;
    use rbs::{to_value, Value};
    use std::str::FromStr;

    #[test]
    fn test_decode_value() {
        let m = to_value! {
            1: 1,
            "2": 2,
        };
        let v: Value = rbatis::decode(Value::Array(vec![m.clone()])).unwrap();
        assert_eq!(v, Value::Array(vec![m]));
    }

    #[test]
    fn test_decode_one() {
        let date = rbdc::types::datetime::DateTime::now();
        let m = to_value! {
            "1" : date.clone(),
        };
        let v: rbdc::types::datetime::DateTime = rbatis::decode(Value::Array(vec![m])).unwrap();
        assert_eq!(v.to_string(), date.to_string());
        println!("{}",v.offset());
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
}
