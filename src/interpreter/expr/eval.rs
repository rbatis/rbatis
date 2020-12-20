extern crate serde_json;

use std::time::SystemTime;

use chrono::Local;
use serde_json::{json, Map};
use serde_json::Value;

use crate::core::Error;
use crate::core::Result;

pub fn eval(left: &Value,
            right: &Value,
            op: &str) -> Result<Value> {
    match op {
        "&&" => {
            if left.is_boolean() && right.is_boolean() {
                return Result::Ok(Value::Bool(left.as_bool().unwrap_or(false) && right.as_bool().unwrap_or(false)));
            }
        }
        "||" => {
            if left.is_boolean() && right.is_boolean() {
                return Result::Ok(Value::Bool(left.as_bool().unwrap_or(false) || right.as_bool().unwrap_or(false)));
            }
        }
        "==" => {
            return Result::Ok(Value::Bool(eq(left, right)));
        }
        "!=" => {
            return Result::Ok(Value::Bool(!eq(left, right)));
        }
        "+" => {
            //allow null,string,number
            let left_is_string = left.is_string();
            let right_is_string = right.is_string();
            if !(left.is_null() || left_is_string || left.is_number()) || !(right.is_null() || right_is_string || right.is_number()) {
                return Result::Err(crate::core::Error::from(format!("[rbatis] eval error express:{} {} {}", left, op, right)));
            }
            if left_is_string || right_is_string {
                let left_v = left.as_str().unwrap_or("");
                let right_v = right.as_str().unwrap_or("");
                return Result::Ok(json!(left_v.to_string()+right_v));
            } else {
                let left_v = left.as_f64().unwrap_or(0.0);
                let right_v = right.as_f64().unwrap_or(0.0);
                let left_i64 = left.is_i64() || left.is_null();
                let right_i64 = right.is_i64() || right.is_null();
                if left_i64 && right_i64 {
                    return Result::Ok(json!(left_v as i64 + right_v as i64));
                }
                return Result::Ok(json!(left_v + right_v));
            }
        }
        _ => {
            //allow number,null
            if !(left.is_number() || left.is_null()) || !(right.is_number() || right.is_null()) {
                return Result::Err(crate::core::Error::from(format!("[rbatis] eval error express:{} {} {}", left, op, right)));
            }
            match op {
                ">=" => {
                    let left_v = left.as_f64().unwrap_or(0.0);
                    let right_v = right.as_f64().unwrap_or(0.0);
                    return Result::Ok(json!(left_v >= right_v));
                }
                "<=" => {
                    let left_v = left.as_f64().unwrap_or(0.0);
                    let right_v = right.as_f64().unwrap_or(0.0);
                    return Result::Ok(json!(left_v <= right_v));
                }
                ">" => {
                    let left_v = left.as_f64().unwrap_or(0.0);
                    let right_v = right.as_f64().unwrap_or(0.0);
                    return Result::Ok(json!(left_v > right_v));
                }
                "<" => {
                    let left_v = left.as_f64().unwrap_or(0.0);
                    let right_v = right.as_f64().unwrap_or(0.0);
                    return Result::Ok(json!(left_v < right_v));
                }
                "*" => {
                    let left_v = left.as_f64().unwrap_or(0.0);
                    let right_v = right.as_f64().unwrap_or(0.0);
                    if left.is_i64() && right.is_i64() {
                        return Result::Ok(json!(left_v as i64 * right_v as i64));
                    }
                    return Result::Ok(json!(left_v * right_v));
                }
                "/" => {
                    let left_v = left.as_f64().unwrap_or(0.0);
                    let right_v = right.as_f64();
                    if right_v.is_some() {
                        if left.is_i64() && right.is_i64() {
                            return Result::Ok(json!(left_v as i64 / right_v.unwrap() as i64));
                        }
                        return Result::Ok(json!(left_v / right_v.unwrap()));
                    } else {
                        return Result::Err(Error::from(format!("[rbatis] express '{} / null' Infinity!", left_v)));
                    }
                }
                "%" => {
                    let left_v = left.as_f64().unwrap_or(0.0);
                    let right_v = right.as_f64();
                    if right_v.is_some() {
                        if left.is_i64() && right.is_i64() {
                            return Result::Ok(json!(left_v as i64 % right_v.unwrap() as i64));
                        }
                        return Result::Ok(json!(left_v % right_v.unwrap()));
                    } else {
                        return Result::Err(Error::from(format!("[rbatis] express '{} % null' Infinity!", left_v)));
                    }
                }
                "^" => {
                    if !(left.is_i64() || left.is_null()) || !(right.is_i64() || right.is_null()) {
                        return Result::Err(crate::core::Error::from(format!("[rbatis] only support 'int ^ int'! express:{}{}{}", left, op, right)));
                    }
                    let left_v = left.as_i64().unwrap_or(0);
                    let right_v = right.as_i64().unwrap_or(0);
                    if left.is_i64() && right.is_i64() {
                        return Result::Ok(json!(left_v as i64 ^ right_v as i64));
                    }
                    return Result::Ok(json!(left_v ^ right_v));
                }
                "**" => {
                    if right.is_u64() == false {
                        return Result::Err(crate::core::Error::from(format!("[rbatis] only support 'number ** uint'! express:{}{}{}", left, op, right)));
                    }
                    let left_v = left.as_f64().unwrap_or(0.0);
                    let right_v = right.as_f64().unwrap();
                    return Result::Ok(json!(left_v.powf(right_v)));
                }
                "-" => {
                    let left_v = left.as_f64().unwrap_or(0.0);
                    let right_v = right.as_f64().unwrap_or(0.0);
                    let left_i64 = left.is_i64() || left.is_null();
                    let right_i64 = right.is_i64() || right.is_null();
                    if left_i64 && right_i64 {
                        return Result::Ok(json!(left_v as i64 - right_v as i64));
                    }
                    return Result::Ok(json!(left_v - right_v));
                }
                _ => {}
            }
        }
    }
    return Result::Err(crate::core::Error::from(format!("[rbatis] eval error express:{} {} {}", left, op, right)));
}


fn eq(left: &Value, right: &Value) -> bool {
    if left.is_number() && right.is_number() {
        return left.as_f64().eq(&right.as_f64());
    }
    return left.eq(right);
}


#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use crate::interpreter::expr::eval::eval;
    use crate::utils::time_util;

    #[test]
    fn test_lexer() {
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
    fn test_array_eq() {
        assert_eq!(eval(&json!([{"a":1}]), &json!([]), "==").unwrap(), false);
        assert_eq!(eval(&json!([{"a":1}]), &json!([{"a":2}]), "==").unwrap(), false);
        assert_eq!(eval(&json!([{"a":1}]), &json!([{"a":1}]), "==").unwrap(), true);
        assert_eq!(eval(&json!([{"a":1}]), &json!([{}]), "==").unwrap(), false);
    }

    #[test]
    fn test_object_eq() {
        assert_eq!(eval(&json!({"a":1}), &json!({"b":1}), "==").unwrap(), false);
        assert_eq!(eval(&json!({"a":"1"}), &json!({"a":"1"}), "==").unwrap(), true);
        assert_eq!(eval(&json!({"a":"1"}), &json!({"a":"2"}), "==").unwrap(), false);
        assert_eq!(eval(&json!(4), &json!(3), "%").unwrap().as_f64().unwrap(), 1.0);
        assert_eq!(eval(&json!(2), &json!(4), "^").unwrap().as_i64().unwrap(), 2 ^ 4);
        assert_eq!(eval(&json!(2), &json!(3), "**").unwrap().as_f64().unwrap(), 8.0);
    }
}