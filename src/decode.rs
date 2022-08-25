//! Types and traits for decoding values from the database.
use log::info;
use rbs::Value;
use serde::de::DeserializeOwned;

use crate::Error;

/// decode json vec to an object
/// support decode types:
/// Value,BigDecimal, i8..i64,u8..u64,Int64(),bool,String
/// or object used bson macro object
pub fn decode<T: ?Sized>(bs: Value) -> Result<T, crate::Error>
    where
        T: DeserializeOwned,
{
    let type_name = std::any::type_name::<T>();
    //debug_mode feature print json_decode json data
    #[cfg(feature = "debug_mode")]
    {
        println!("[rbatis] [debug_mode] [value]   {} => {}", type_name, bs);
    }
    let mut datas = vec![];
    match bs {
        Value::Array(arr) => {
            datas = arr;
        }
        _ => {}
    }
    // try speculate type
    let is_array = rbs::from_value::<T>(Value::Array(vec![])).is_ok();
    if is_array {
        //decode array
        Ok(rbs::from_value(Value::Array(datas))?)
    } else {
        Ok(try_decode_map(type_name, &mut datas)?)
    }
}

//decode doc or one type
pub fn try_decode_map<T>(type_name: &str, datas: &mut Vec<Value>) -> Result<T, crate::Error>
    where
        T: DeserializeOwned,
{
    //decode struct
    if datas.len() > 1 {
        return Result::Err(Error::from(format!(
            "[rbatis] rows.rows_affected > 1,but decode one type ({})!",
            type_name
        )));
    }
    //single try decode
    if datas.is_empty() {
        return Ok(rbs::from_value::<T>(Value::Null)?);
    }
    let m = datas.remove(0);
    let mut doc_len = 0;
    match &m {
        Value::Map(doc) => {
            doc_len = doc.len();
            if doc_len == 1 {
                if let Some((_, value)) = doc.into_iter().next() {
                    //try one
                    if let Ok(v)=rbs::from_value::<T>(value.clone()){
                        return Ok(v);
                    }
                }
            }
        }
        _ => {}
    }
    Ok(rbs::from_value::<T>(m)?)
}

#[cfg(test)]
mod test {
    use crate::decode::decode;
    use rbs::value::map::ValueMap;
    use rbs::{to_value, Value};
    use std::collections::HashMap;

    #[test]
    fn test_decode_hashmap() {
        let mut v = ValueMap::new();
        v.insert(1.into(), 2.into());
        let m: HashMap<i32, Value> = decode(Value::Array(vec![Value::Map(v)])).unwrap();
        println!("{:#?}", m);
        assert_eq!(m.get(&1).unwrap().as_i64(), Value::I32(2).as_i64());
    }
}
