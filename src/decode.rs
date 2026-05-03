//! Types and traits for decoding values from the database.
use std::ops::Index;

use rbs::Value;
use serde::de::DeserializeOwned;

use crate::Error;

/// decode json vec to an object
/// support decode types:
/// Value,BigDecimal, i8..i64,u8..u64,i64,bool,String
/// or object used rbs::Value macro object
/// values = [{k:v},...]
/// T = Vec<YourStruct>
pub fn decode_ref<T: ?Sized>(values: &Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    // Check if value is Array first (required by API contract)
    match values {
        Value::Array(_) => {
            // First try rbs direct decode (handles [{k:v},...] format to Vec<T>)
            let direct_result = rbs::from_value_ref::<T>(values);
            if direct_result.is_ok() {
                return direct_result;
            }
            // If direct decode failed, try the old map format conversion
            try_decode_map(values)
        }
        _ => {
            // Non-array values are not supported (maintain API contract)
            Err(Error::from("decode error: expected array value"))
        }
    }
}

pub fn decode<T: ?Sized>(bs: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    decode_ref(&bs)
}

//decode one type from array of maps
/// values = [{k:v},...]
pub fn try_decode_map<T>(datas: &Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    // Handle empty array case
    if datas.len() == 0 {
        return Err(Error::from("decode empty array value"));
    }
    //decode struct
    if datas.len() > 1 {
        return Err(Error::from(format!(
            "[rb] rows.rows_affected > 1,but decode one type ({})!",
            std::any::type_name::<T>()
        )));
    }
    let values = datas.index(0);
    match values {
        Value::Map(arr) => {
            if arr.len() == 1 {
                // 尝试直接解码单个元素，失败则继续 fallback 到 Vec 方式
                if let Some((_key, value)) = arr.into_iter().next() {
                    if let Ok(result) = rbs::from_value_ref::<T>(value) {
                        return Ok(result);
                    }
                }
            }
        }
        _ => {}
    }
    //convert to map (for struct types or when direct decode fails)
    let arr: Vec<T> = rbs::from_value_ref(datas)?;
    arr.into_iter()
        .next()
        .ok_or_else(|| {
            Error::from(format!(
                "[rb] decode fail: cannot decode into type {} from empty result",
                std::any::type_name::<T>()
            ))
        })
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
