//! Types and traits for decoding values from the database.
use rbs::Value;
use serde::de::DeserializeOwned;

use crate::Error;

/// decode json vec to an object
/// support decode types:
/// Value,BigDecimal, i8..i64,u8..u64,i64,bool,String
/// or object used rbs::Value macro object
pub fn decode_ref<T: ?Sized>(values: &Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    // try to identify type
    let is_array = rbs::from_value::<T>(Value::Array(vec![])).is_ok();
    if is_array {
        //decode array
        Ok(rbs::from_value_ref(values)?)
    } else {
        match values {
            Value::Array(datas) => Ok(try_decode_map(datas)?),
            _ => Err(Error::from("decode an not array value")),
        }
    }
}

pub fn decode<T: ?Sized>(bs: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    decode_ref(&bs)
}

//decode doc or one type
pub fn try_decode_map<T>(datas: &Vec<Value>) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    //decode struct
    if datas.len() > 1 {
        return Err(Error::from(format!(
            "[rbatis] rows.rows_affected > 1,but decode one type ({})!",
            std::any::type_name::<T>()
        )));
    }
    //single try decode
    if datas.is_empty() {
        return Ok(rbs::from_value::<T>(Value::Null)?);
    }
    let m = datas.get(0).unwrap();
    match &m {
        Value::Map(map) => {
            if map.len() == 1 {
                //try one
                let type_name = std::any::type_name::<T>();
                if type_name == std::any::type_name::<i32>()
                    || type_name == std::any::type_name::<i64>()
                    || type_name == std::any::type_name::<f32>()
                    || type_name == std::any::type_name::<f64>()
                    || type_name == std::any::type_name::<u32>()
                    || type_name == std::any::type_name::<u64>()
                    || type_name == std::any::type_name::<String>()
                    || type_name == std::any::type_name::<bool>()
                    || type_name == std::any::type_name::<Option<i32>>()
                    || type_name == std::any::type_name::<Option<i64>>()
                    || type_name == std::any::type_name::<Option<f32>>()
                    || type_name == std::any::type_name::<Option<f64>>()
                    || type_name == std::any::type_name::<Option<u32>>()
                    || type_name == std::any::type_name::<Option<u64>>()
                    || type_name == std::any::type_name::<Option<String>>()
                    || type_name == std::any::type_name::<Option<bool>>()
                    || type_name.starts_with("rbdc::types::")
                    || type_name.starts_with("core::option::Option<rbdc::types::")
                {
                    let (_, value) = map.into_iter().next().unwrap();
                    return Ok(rbs::from_value_ref::<T>(value)?);
                }
            }
        }
        _ => {}
    }
    Ok(rbs::from_value_ref::<T>(m)?)
}

pub fn is_debug_mode() -> bool {
    if cfg!(debug_assertions) {
        #[cfg(feature = "debug_mode")]
        {
            true
        }
        #[cfg(not(feature = "debug_mode"))]
        {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use crate::decode::decode;
    use rbs::value::map::ValueMap;
    use rbs::Value;
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
