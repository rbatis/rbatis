pub mod common;
pub mod db;
pub mod error;
#[macro_use]
pub mod ext;
pub mod io;
pub mod net;
pub mod rt;
pub use error::*;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn test_ser_ref() {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct A {
            pub name: String,
        }
        let a = A {
            name: "sss".to_string(),
        };
        let v = rbs::to_value_ref(&a).unwrap();
        println!("{:?}", v);

        let mut m = HashMap::new();
        m.insert(1, 2);
        let v = rbs::to_value_ref(&m).unwrap();
        println!("{:?}", v);

        let v = rbs::to_value(a).unwrap();
        println!("v: {}", v);
        let s: A = rbs::from_value(v).unwrap();
        println!("s:{:?}", s);
    }

    #[test]
    fn test_ser() {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct A {
            pub name: String,
            pub i32: i32,
            pub u32: u32,
            pub i64: i64,
            pub u64: u64,
        }
        let buf = rbs::to_vec(&A {
            name: "s".to_string(),
            i32: i32::MAX,
            u32: u32::MAX,
            i64: i64::MAX,
            u64: u64::MAX,
        })
        .unwrap();
        let v: rbs::Value = rbs::read_value(&mut &buf[..]).unwrap();
        println!("{}", v);

        let v: A = rbs::decode::from_slice(&buf).unwrap();
        println!("{:?}", v);
    }
}
