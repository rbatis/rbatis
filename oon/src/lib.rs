use indexmap::IndexMap;
use serde::Serialize;

///ORM Object Notation
pub enum OON {
    Null,
    String(String),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    Bool(bool),
    Double(f64),
    Bytes(Vec<u8>),
    Vec(Vec<OON>),
    Map(IndexMap<String, OON>),
    Struct(String, Vec<(String, OON)>),
}


#[cfg(test)]
mod test {
    // use serde::{Serialize,Deserialize};
    // use crate::OON;
    //
    // #[derive(Serialize, Deserialize)]
    // pub struct A {
    //     name: String,
    //     age: i32,
    // }
    //
    // #[test]
    // fn test_ser_a(){
    //     let a=A{
    //         name: "".to_string(),
    //         age: 0
    //     };
    // }
}