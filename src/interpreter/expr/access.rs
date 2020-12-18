use serde_json::Value;

pub trait AccessField {
    fn access_field(&self, env: &Value) -> Result<Value, crate::core::Error>;
}

pub trait AccessMethod {
    fn access_method(&self, env: &Value) -> Result<Value, crate::core::Error>;
}

/// a[0]  or a.b
impl AccessField for Value {
    fn access_field(&self, env: &Value) -> Result<Value, crate::core::Error> {
        let arr = self.as_array().unwrap();
        let arr_len = arr.len() as i32;
        if arr_len == 0 {
            return Result::Ok(Value::Null);
        }
        let mut index = 0;
        let mut v = env;
        for item in arr {
            let item_str = item.as_str().unwrap();
            if v.is_object() {
                v = v.get(item_str).unwrap_or(&Value::Null);
            } else if v.is_array() {
                let item_index = item_str.parse::<usize>();
                if item_index.is_err() {
                    return Result::Ok(serde_json::Value::Null);
                }
                let item_index = item_index.unwrap();
                let arr_ref = v.as_array().unwrap();
                v = arr_ref.get(item_index).unwrap_or(&Value::Null);
            }
            if v.is_null() || index + 1 >= arr_len {
                return Result::Ok(v.clone());
            }
            index = index + 1;
        }
        return Result::Ok(Value::Null);
    }
}