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


pub struct Ser{}

// impl Serializer for Ser{
//     type Ok = ();
//     type Error = ();
//     type SerializeSeq = ();
//     type SerializeTuple = ();
//     type SerializeTupleStruct = ();
//     type SerializeTupleVariant = ();
//     type SerializeMap = ();
//     type SerializeStruct = ();
//     type SerializeStructVariant = ();
//
//     fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
//         todo!()
//     }
//
//     fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
//         todo!()
//     }
//
//     fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
//         todo!()
//     }
//
//     fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
//         todo!()
//     }
//
//     fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
//         todo!()
//     }
// }

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