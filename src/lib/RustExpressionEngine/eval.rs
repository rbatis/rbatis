extern crate serde_json;

use serde_json::json;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use chrono::Local;
use crate::utils::time_util;
use std::iter::Map;
use std::any::Any;
use std::rc::Rc;

pub fn Eval(left: &Value,
            right: &Value,
            op: &str) -> Result<Value,String> {
    if op == "&&" {
        return Result::Ok(Value::Bool(left.as_bool().unwrap() && right.as_bool().unwrap()));
    }
    if op == "||" {
        return Result::Ok(Value::Bool(left.as_bool().unwrap() || right.as_bool().unwrap()));
    }

    if op == "==" {
        if left.is_number() && right.is_number() {
            return Result::Ok(Value::Bool(left.as_f64() == right.as_f64()));
        }
        return Result::Ok(Value::Bool(left.eq(right)));
    }
    if op == "!=" {
        if left.is_number() && right.is_number() {
            return Result::Ok(Value::Bool(!(left.as_f64() == right.as_f64())));
        }
        return Result::Ok(Value::Bool(!left.eq(right)));
    }
    if op == ">=" {
        let booll = left.is_number();
        let boolr = right.is_number();
        if booll && boolr {
            return Result::Ok(Value::Bool(left.as_f64() >= right.as_f64()));
        }
    }
    if op == "<=" {
        let booll = left.is_number();
        let boolr = right.is_number();
        if booll && boolr {
            return Result::Ok(Value::Bool(left.as_f64() <= right.as_f64()));
        }
    }
    if op == ">" {
        let booll = left.is_number();
        let boolr = right.is_number();
        if booll && boolr {
            return Result::Ok(Value::Bool(left.as_f64() > right.as_f64()));
        }
    }
    if op == "<" {
        let booll = left.is_number();
        let boolr = right.is_number();
        if booll && boolr {
            return Result::Ok(Value::Bool(left.as_f64() < right.as_f64()));
        }
    }
    if op == "+" {
        let booll = left.is_number();
        let boolr = right.is_number();
        if booll && boolr {
            return Result::Ok(Value::Number(serde_json::Number::from_f64(left.as_f64().unwrap() + right.as_f64().unwrap()).unwrap()));
        } else {
            return Result::Ok(Value::from(left.as_str().unwrap().to_owned() + right.as_str().unwrap()));
        }
    }
    if op == "-" {
        let booll = left.is_number();
        let boolr = right.is_number();
        if booll && boolr {
            return Result::Ok(Value::Number(serde_json::Number::from_f64(left.as_f64().unwrap() - right.as_f64().unwrap()).unwrap()));
        }
    }
    if op == "*" {
        let booll = left.is_number();
        let boolr = right.is_number();
        if booll && boolr {
            return Result::Ok(Value::Number(serde_json::Number::from_f64(left.as_f64().unwrap() * right.as_f64().unwrap()).unwrap()));
        }
    }
    if op == "/" {
        let booll = left.is_number();
        let boolr = right.is_number();
        if booll && boolr {
            return Result::Ok(Value::Number(serde_json::Number::from_f64(left.as_f64().unwrap() / right.as_f64().unwrap()).unwrap()));
        }
    }
    return Result::Err("un support opt = ".to_owned()+op);
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