extern crate serde_json;

use std::any::Any;
use std::iter::Map;
use std::time::SystemTime;

use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

use crate::utils::time_util;

pub fn eval(left: &Value,
            right: &Value,
            op: &str) -> Result<Value, rbatis_core::Error> {
    if op == "&&" {
        return Result::Ok(Value::Bool(left.as_bool().unwrap() && right.as_bool().unwrap()));
    }
    if op == "||" {
        return Result::Ok(Value::Bool(left.as_bool().unwrap() || right.as_bool().unwrap()));
    }

    if op == "==" {
        return Result::Ok(Value::Bool(eq(left, right)));
    }
    if op == "!=" {
        return Result::Ok(Value::Bool(!eq(left, right)));
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
        if left.is_number() && right.is_number() {
            return Result::Ok(Value::Number(serde_json::Number::from_f64(left.as_f64().unwrap() + right.as_f64().unwrap()).unwrap()));
        } else if left.is_string() && right.is_string() {
            return Result::Ok(Value::from(left.as_str().unwrap().to_owned() + right.as_str().unwrap()));
        } else {
            return Result::Err(rbatis_core::Error::from("[rbatis] un support diffrent type '+' opt"));
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
    return Result::Err(rbatis_core::Error::from("[rbatis] un support opt = ".to_owned() + op));
}


fn eq(left: &Value, right: &Value) -> bool {
    if left.is_null() && right.is_null() {// all null
        return true;
    } else if left.is_null() || right.is_null() {// on null
        return false;
    } else if left.is_number() && right.is_number() {
        return left.as_f64() == right.as_f64();
    } else if left.is_string() && right.is_string() {
        return left.as_str().unwrap().eq(right.as_str().unwrap());
    } else if left.is_boolean() && right.is_boolean() {
        return left.as_bool() == right.as_bool();
    } else {
        return false;
    }
}

#[test]
fn test_parser() {
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
fn test_take_value() {
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
fn benchmark_fromstr() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let total = 100000;
    let now = std::time::Instant::now();
    for i in 0..total {
        let deserialized: Point = serde_json::from_str(&serialized).unwrap();
        // println!("deserialized = {:?}", deserialized);
    }
    time_util::count_time_qps("benchmark_fromstr", total, now);
}

#[test]
fn benchmark_to_string() {
    let point = Point { x: 1, y: 2 };


    let total = 100000;
    let now = std::time::Instant::now();
    for i in 0..total {
        let serialized = serde_json::to_string(&point).unwrap();
        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
    }
    time_util::count_time_qps("benchmark_to_string", total, now);
}