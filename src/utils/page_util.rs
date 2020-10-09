pub fn get_count(arg: &serde_json::Value) -> Option<u64> {
    if arg.is_array() {
        let arr = arg.as_array().unwrap();
        if arr.len() == 0 {
            return Some(0);
        }
        let arg = arr.get(0)?;
        let map = arg.as_object()?;
        let v = map.get("count(1)")?;
        return Some(v.as_u64().unwrap_or(0));
    }
    return Some(0);
}