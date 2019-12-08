pub struct ArgFilter {}

impl ArgFilter {

    ///  select * from table where name = #{'hello' OR 'A'!=''}
    ///过滤替换 参数中可能包含的影响sql安全的因素
    pub fn filter(arg: &mut serde_json::Value) {
        if arg.is_object() {
            let mut m = arg.as_object_mut().unwrap();
            for (k, v) in m {
                if v.is_string() {
                    let s = v.as_str().unwrap().replace("'", r#"\'"#);
                    *v = serde_json::Value::String(s);
                }
            }
        }
    }
}

#[test]
fn test_filter(){
    let  mut js:serde_json::Value=serde_json::from_str(r#"{"id":"","name":"hello' OR 'A'!='","version":0}"#).unwrap();
    ArgFilter::filter(&mut js);
    println!("{}",js.to_string());
}