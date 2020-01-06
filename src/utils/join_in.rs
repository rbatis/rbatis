use serde_json::Value;
use serde::Serialize;

pub fn json_join<T,JOIN_IN>(value:T,key:&str,join_in:JOIN_IN) ->Result<Value,String>
    where
        T: Serialize,JOIN_IN:Serialize{
    let mut arg= serde_json::to_value(value).unwrap();
    if !arg.is_object(){
        return Err("json_join value must be a object!".to_string());
    }
    arg.as_object_mut().unwrap().insert(key.to_string(),serde_json::to_value(join_in).unwrap());
    return Result::Ok(arg);
}