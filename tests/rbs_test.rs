#[cfg(test)]
mod test {
    use rbatis_codegen::ops::{Add, BitAnd, BitOr, Div, Mul, Not, PartialEq, PartialOrd, Rem, Sub};
    use rbdc::datetime::DateTime;
    use rbdc::Timestamp;
    use rbs::Value;
    use serde::{Deserialize, Serialize};
    use std::cmp::Ordering;

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
    fn test_fmt() {
        use std::str::FromStr;
        let a = rbs::to_value!(true);
        let b = rbs::to_value!("11");
        let c = rbs::to_value!(DateTime::from_str("2023-03-22T00:39:04.0278992Z").unwrap());
        let d = rbs::to_value! {
            "1":1,
        };
        assert_eq!(a.to_string(), "true");
        assert_eq!(b.to_string(), r#""11""#);
        println!("{},{:?}", c, c);
        assert_eq!(c.to_string(), r#""2023-03-22T00:39:04.0278992Z""#);
        assert_eq!(
            format!("{:?}", c),
            r#"Ext("DateTime", String("2023-03-22T00:39:04.0278992Z"))"#
        );
        assert_eq!(d.to_string(), r#"{"1":1}"#);
    }

    #[test]
    fn test_ser() {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        pub enum A {
            AA,
            BB,
        }
        let v = rbs::to_value!(A::BB);
        println!("{:?}", v);

        let nv: A = rbs::from_value(v).unwrap();
        println!("{:?}", nv);
        assert_eq!(nv, A::BB);
    }

    #[test]
    fn test_ser_variant() {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        pub enum A {
            BB(i32), //{"BB":2}
        }
        let v = rbs::to_value!(A::BB(2));
        println!("{:?}", v);
        let nv: A = rbs::from_value(v).unwrap();
        assert_eq!(nv, A::BB(2));
    }

    #[test]
    fn test_ser_variant2() {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        pub enum A {
            BB(String), //{"BB":"2"}
        }
        let v = rbs::to_value!(A::BB(2.to_string()));
        println!("{:?}", v);
        let nv: A = rbs::from_value(v).unwrap();
        assert_eq!(nv, A::BB(2.to_string()));
    }

    #[test]
    fn test_ser_num() {
        let v = rbs::to_value!(1i8);
        let d: u64 = rbs::from_value(v).unwrap();
        assert_eq!(d, 1);
    }

    #[test]
    fn test_ser_newtype_struct() {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        pub struct A(i32);
        let v = rbs::to_value!(A(1));
        assert_eq!(v, Value::Ext("A", Box::new(Value::I32(1))));
    }

    #[test]
    fn test_ser_newtype_struct_timestamp() {
        let v = rbs::to_value!(Timestamp(1));
        assert_eq!(v, Value::Ext("Timestamp", Box::new(Value::I64(1))));
    }

    #[test]
    fn test_ser_newtype_struct_timestamp_tz() {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
        #[serde(rename = "Timestamptz")]
        pub struct Timestamptz(pub i64, pub i32);

        let v = rbs::to_value!(Timestamptz(1, 1));
        assert_eq!(
            v,
            Value::Ext(
                "Timestamptz",
                Box::new(Value::Array(vec![Value::I64(1), Value::I32(1)]))
            )
        );
    }


    #[test]
    fn test_de_string() {
        let v = rbs::to_value!("1");
        let r:String = rbs::from_value(v).unwrap();
        assert_eq!(r, "1");
    }

    #[test]
    fn test_de_null() {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        struct MockTable {
            pub id: Option<String>,
            pub name: Option<String>,
        }
        let v: Option<MockTable> = rbs::from_value(Value::Null).unwrap();
        assert_eq!(v.is_none(), true);
    }

    #[test]
    fn test_to_value_map() {
        let v = rbs::to_value! {
            "1":"1",
            "2":"2",
        };
        assert_eq!(v.to_string(), "{\"1\":\"1\",\"2\":\"2\"}");
    }

    #[test]
    fn test_de_position() {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        struct MockTable {
            // pub id: Option<String>,
            pub name: Option<String>,
        }
        let value = rbs::to_value! {
            "name": 0,
        };
        let v = rbs::from_value::<MockTable>(value).err().unwrap();
        println!("{}",v.to_string());
        #[cfg(feature = "debug_mode")]
        assert_eq!(v.to_string(),"invalid type: integer `0`, expected a string,key = `name`");
    }
}
