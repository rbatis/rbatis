use serde::Serialize;
use serde_json::json;
use serde_json::Value;

pub fn json_join<T, JoinIn>(value: &T, key: &str, join_in: JoinIn) -> Result<Value, String>
    where
        T: Serialize, JoinIn: Serialize {
    let mut arg = json!(value);
    if !arg.is_object() {
        return Err("json_join value must be a object!".to_string());
    }
    arg[key] = json!(join_in);
    return Result::Ok(arg);
}