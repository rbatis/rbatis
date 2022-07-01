mod ser;
mod de;
pub mod error;
use std::marker::PhantomData;
use indexmap::IndexMap;
use serde::{Deserializer, Serialize, Serializer};
use serde::de::Visitor;

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
    Struct(Struct),
}

pub struct Struct{
   pub name:String,
   pub fields:Vec<(String,OON)>
}

impl Struct {
    pub fn get(&self,key:&str)->Option<&OON>{
        for (k,v) in &self.fields {
            if k.eq(key){
                return Some(v);
            }
        }
        return None;
    }
}



#[cfg(test)]
mod test {

    #[derive(serde::Serialize,serde::Deserialize)]
    pub struct A {
        name: String,
        age: i32,
    }

    #[test]
    fn test_ser_a(){
        let a=A{
            name: "".to_string(),
            age: 0
        };
    }
}