#[cfg(test)]
mod test {
    use rbatis_codegen::ops::{Add, BitAnd, BitOr, Div, Mul, Not, PartialEq, PartialOrd, Rem, Sub};
    use rbs::Value;
    use std::cmp::Ordering;
    use rbs::value::map::ValueMap;

    #[test]
    fn test_ser_i32() {
        let v = rbs::to_value!(1);
        assert_eq!(v, Value::I32(1));
    }

    #[test]
    fn test_ser_i64() {
        let v = rbs::to_value!(1i64);
        assert_eq!(v, Value::I64(1));
    }

    #[test]
    fn test_ser_u32() {
        let v = rbs::to_value!(1u32);
        assert_eq!(v, Value::U32(1));
    }

    #[test]
    fn test_ser_u64() {
        let v = rbs::to_value!(1u64);
        assert_eq!(v, Value::U64(1));
    }

    #[test]
    fn test_ser_f32() {
        let v = rbs::to_value!(1f32);
        assert_eq!(v, Value::F32(1.0));
    }

    #[test]
    fn test_ser_f64() {
        let v = rbs::to_value!(1f64);
        assert_eq!(v, Value::F64(1.0));
    }

    #[test]
    fn test_ser_bool() {
        let v = rbs::to_value!(true);
        assert_eq!(v, Value::Bool(true));
    }

    #[test]
    fn test_ser_null() {
        let v = rbs::to_value!(());
        assert_eq!(v, Value::Null);
    }

    #[test]
    fn test_ser_str() {
        let v = rbs::to_value!("1");
        assert_eq!(v, Value::String("1".to_string()));
    }

    #[test]
    fn test_add() {
        let a = rbs::to_value!(1);
        let b = rbs::to_value!(1);
        assert_eq!(a.op_add(b), Value::I32(2));
    }

    #[test]
    fn test_bit_and() {
        let a = rbs::to_value!(true);
        let b = rbs::to_value!(true);
        assert_eq!(a.op_bitand(b), true);
    }

    #[test]
    fn test_bit_or() {
        let a = rbs::to_value!(true);
        let b = rbs::to_value!(true);
        assert_eq!(a.op_bitor(b), true);
    }

    #[test]
    fn test_cmp() {
        let a = rbs::to_value!(true);
        let b = rbs::to_value!(true);
        assert_eq!(a.op_partial_cmp(&b), Some(Ordering::Equal));
    }

    #[test]
    fn test_div() {
        let a = rbs::to_value!(1);
        let b = rbs::to_value!(1);
        assert_eq!(a.op_div(b), Value::I32(1));
    }

    #[test]
    fn test_eq() {
        let a = rbs::to_value!(1);
        let b = rbs::to_value!(1);
        assert_eq!(a.op_eq(&b), true);
    }

    #[test]
    fn test_mul() {
        let a = rbs::to_value!(1);
        let b = rbs::to_value!(1);
        assert_eq!(a.op_mul(b), Value::I32(1));
    }

    #[test]
    fn test_not() {
        let a = rbs::to_value!(false);
        assert_eq!(a.op_not(), Value::Bool(true));
    }

    #[test]
    fn test_rem() {
        let a = rbs::to_value!(1);
        let b = rbs::to_value!(1);
        assert_eq!(a.op_rem(b), Value::I32(0));
    }

    #[test]
    fn test_sub() {
        let a = rbs::to_value!(1);
        let b = rbs::to_value!(1);
        assert_eq!(a.op_sub(b), Value::I32(0));
    }

    #[test]
    fn test_xor() {
        let a = rbs::to_value!(true);
        let b = rbs::to_value!(false);
        assert_eq!(a.op_bitor(b), true);
    }

    #[test]
    fn test_into_iter(){
        let mut  v= ValueMap::new();
        v.insert(1.into(),2.into());
        let m=Value::Map(v.clone());
        let r=&m;
        let mut items = vec![];
        for (k,v) in r.into_iter() {
            items.push((k.as_ref().clone(),v.clone()));
        }
        assert_eq!(ValueMap::from(items),v);
    }

    #[test]
    fn test_readme_code(){
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct A {
            pub name: String,
        }
        let a = A {
            name: "sss".to_string(),
        };
        let v = rbs::to_value(a).unwrap();
        println!("v: {}",v);
        let s: A = rbs::from_value(v.clone()).unwrap();
        println!("s:{:?}", s);
        let json = v.to_string();
        assert_eq!(r#"{"name":"sss"}"#,json);
    }
}
