extern crate serde_json;

use serde_json::json;
use serde_json::Value;
use serde::{Serialize, Deserialize};


pub fn Eval(left: &Value,
            right: &Value,
            opt: &String, ) -> (Value, String) {
    //+ - * / == >= <= !=
    println!("l:{}", left);
    println!("r:{}", right);
    println!("opt:{}", opt);

    let op = opt.as_str();
    if op == "+" {
        let mut s = String::new();
        s.push_str(left.as_str().unwrap());
        s.push_str(right.as_str().unwrap());
        return (Value::String(s), "".to_string());
    }

    return (Value::Null, "".to_string());
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

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn TestTakeValue() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    //create serde_json::Value
    let john = json!(point);
    println!("{}", john["x"]);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

