use serde_json::json;
use serde_json::Value;

//深度取值。例如a.b.c 最终得到c.如果不存在返回Value::Null
pub fn get_deep_value(arg: &str, value: &Value) -> Value {
    let splits: Vec<&str> = arg.split(".").collect();

    let mut v = value;
    for item in splits {
        if item.is_empty() {
            continue;
        }
        v = v.get(item).unwrap_or(&Value::Null);
    }
    return v.clone();
}

#[test]
pub fn test_get_deep_value() {
    let john = json!({
        "a": {
           "name":"job",
        },
    });

    let v = get_deep_value("a.name", &john);
    println!("{}", v);
}
