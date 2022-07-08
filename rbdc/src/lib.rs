use std::collections::HashMap;
use indexmap::IndexMap;

pub mod encode;
pub mod decode;
pub mod db;


#[cfg(test)]
mod test {
    #[test]
    fn test_ser() {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct A {
            pub name: String,
        }
        let buf = rbmp_serde::to_vec(&A {
            name: "s".to_string()
        }).unwrap();
        let v: rbmpv::Value = rbmpv::decode::read_value(&mut &buf[..]).unwrap();
        println!("{}", v);

        let v: A = rbmp_serde::decode::from_slice(&buf).unwrap();
        println!("{:?}", v);
    }
}