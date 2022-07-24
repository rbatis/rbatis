pub mod common;
pub use common::*;
pub mod db;
pub mod error;
#[macro_use]
pub mod ext;
pub mod io;
pub mod net;
pub mod pool;
pub mod rt;
pub mod time;
pub mod types;

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
        let mut m = HashMap::new();
        m.insert(1, 2);
        let v = rbs::to_value(a).unwrap();
        println!("v: {}", v);
        let s: A = rbs::from_value(v).unwrap();
        println!("s:{:?}", s);
    }

    #[test]
    fn test_ext() {
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct ExtStruct(String);
        let arg = ExtStruct {
            0: "saasdfas".to_string(),
        };
        let v = rbs::to_value(&arg).unwrap();
        println!("{:?}", v);

        let ext: ExtStruct = rbs::from_value(v).unwrap();
        assert_eq!(arg, ext);
    }
}
