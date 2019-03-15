extern crate serde_json;

use serde_json::json;
use serde_json::Value;

pub fn Parser() -> String {
    return String::from("ds");
}

#[test]
fn TestParser() {
    let john = json!({
        "name": "John Doe",
        "age": Value::Null,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });


    let age = &john["age"];
    println!("{}", *age);
}