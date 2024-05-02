#[cfg(test)]
mod test {
    use rbs::value::map::ValueMap;
    use rbs::{to_value, Value};
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;

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
}
