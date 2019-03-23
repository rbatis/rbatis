extern crate serde_json;

use serde_json::json;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use chrono::Local;
use crate::utils::time_util;
use std::iter::Map;
use std::any::Any;

pub fn Eval(left: &Value,
            right: &Value,
            opt: &String) -> (Value, String) {
    //+ - * / == >= <= !=
//    println!("l:{}", left);
//    println!("r:{}", right);
//    println!("opt:{}", opt);

    let op = opt.as_str();
    if op == "==" {

    }
    if op == "!=" {

    }
    if op == ">=" {

    }
    if op == "<=" {
       if left.is_i64() && right.is_i64(){
           return (Value::Bool(left.as_i64() <= right.as_i64()),String::new());
       }
    }
    if op == "*" {

    }
    if op == "/" {

    }
    if op == "+" {
        let mut s = String::new();
        s.push_str(left.as_str().unwrap_or_default());
        s.push_str(right.as_str().unwrap_or_default());
        return (Value::String(s), String::new());
    }
    if op == "-" {

    }

//    match left {
//        Value::Null => println!("null"),
//        Value::Bool(v) => println!("null"),
//        Value::Number(v) => println!("null"),
//        Value::String(v) => println!("null"),
//        Value::Array(v) => println!("null"),
//        Value::Object(v) => println!("null"),
//    }


    return (Value::Null, String::new());
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

#[test]
fn BenchmarkFromStr() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let total = 100000;
    let now = Local::now();
    for i in 0..total {
        let deserialized: Point = serde_json::from_str(&serialized).unwrap();
       // println!("deserialized = {:?}", deserialized);
    }
    time_util::count_time(total, now);
    time_util::count_tps(total, now);
}

#[test]
fn BenchmarkToString() {
    let point = Point { x: 1, y: 2 };



    let total = 100000;
    let now = Local::now();
    for i in 0..total {
        let serialized = serde_json::to_string(&point).unwrap();
        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
    }
    time_util::count_time(total, now);
    time_util::count_tps(total, now);
}