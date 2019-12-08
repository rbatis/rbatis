pub struct ArgFilter {}

impl ArgFilter {
    ///过滤替换 参数中可能包含的影响sql安全的因素
    pub fn filter(arg: &mut serde_json::Value) {
        if arg.is_object() {
            let mut m = arg.as_object_mut().unwrap();
            for (k, v) in m {
                if v.is_string() {
                    let s = v.as_str().unwrap().replace("'", "\\'");
                    *v = serde_json::Value::String(s);
                }
            }
        }
    }
}