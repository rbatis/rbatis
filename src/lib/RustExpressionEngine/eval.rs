extern crate serde_json;

use serde_json::json;
use serde_json::Value;
use serde::{Serialize, Deserialize};


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

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn TestTakeValue(){
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}