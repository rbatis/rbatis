//! Types and traits for decoding values from the database.
use std::ops::Index;

use rbs::{Value};
use serde::de::DeserializeOwned;

use crate::Error;

/// decode json vec to an object
/// support decode types:
/// Value,BigDecimal, i8..i64,u8..u64,i64,bool,String
/// or object used rbs::Value macro object
/// values = [[col1,col2],[val1,val2],...]]
pub fn decode_ref<T: ?Sized>(values: &Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    // Check if value is Array first (required by API contract)
    match values {
        Value::Array(_) => {
            // First try rbs direct decode (handles CSV format [[col1,col2],[val1,val2],...])
            let direct_result = rbs::from_value_ref::<T>(values);
            if direct_result.is_ok() {
                return direct_result;
            }
            // If direct decode failed, try the old map format conversion
            try_decode_map(values)
        }
        _ => {
            // Non-array values are not supported (maintain API contract)
            Err(Error::from("decode an not array value"))
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
/// values = [[col1,col2],[val1,val2],...]]
pub fn try_decode_map<T>(datas: &Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    //decode struct
    if datas.len() > 2 {
        return Err(Error::from(format!(
            "[rb] rows.rows_affected > 1,but decode one type ({})!",
            std::any::type_name::<T>()
        )));
    }
    //single try decode
    if datas.len() <= 1 {
        return Ok(rbs::from_value::<T>(Value::Null)?);
    }
    //let columns = datas.index(0);
    let values = datas.index(1);
    match values {
        Value::Array(arr) => {
             if arr.len() == 1 {
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
                    if let Some(value) = arr.into_iter().next() {
                        return Ok(rbs::from_value_ref::<T>(value)?);
                    }
                }
            }
        }
        _ => {}
     }
     //convert to map
     let arr = rbs::from_value_ref::<Vec<T>>(datas)?;
     arr.into_iter().next().ok_or_else(||Error::from("fail type"))
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