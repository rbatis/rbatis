extern crate serde_json;

use std::time::SystemTime;
use chrono::Local;
use serde_json::{json, Map};
use serde_json::Value;

pub fn eval(left: &Value,
            right: &Value,
            op: &str) -> Result<Value, crate::core::Error> {
    match op {
        "&&" => {
            return Result::Ok(Value::Bool(left.as_bool().unwrap() && right.as_bool().unwrap()));
        }
        "||" => {
            return Result::Ok(Value::Bool(left.as_bool().unwrap() || right.as_bool().unwrap()));
        }
        "==" => {
            return Result::Ok(Value::Bool(eq(left, right)));
        }
        "!=" => {
            return Result::Ok(Value::Bool(!eq(left, right)));
        }
        ">=" => {
            let booll = left.is_number();
            let boolr = right.is_number();
            if booll && boolr {
                return Result::Ok(Value::Bool(left.as_f64() >= right.as_f64()));
            }
        }
        "<=" => {
            let booll = left.is_number();
            let boolr = right.is_number();
            if booll && boolr {
                return Result::Ok(Value::Bool(left.as_f64() <= right.as_f64()));
            }
        }
        ">" => {
            let booll = left.is_number();
            let boolr = right.is_number();
            if booll && boolr {
                return Result::Ok(Value::Bool(left.as_f64() > right.as_f64()));
            }
        }
        "<" => {
            let booll = left.is_number();
            let boolr = right.is_number();
            if booll && boolr {
                return Result::Ok(Value::Bool(left.as_f64() < right.as_f64()));
            }
        }
        "*" => {
            let booll = left.is_number();
            let boolr = right.is_number();
            if booll && boolr {
                if left.is_i64() && right.is_i64(){
                    return Result::Ok(json!(left.as_i64().unwrap() * right.as_i64().unwrap()));
                }
                return Result::Ok(json!(left.as_f64().unwrap() * right.as_f64().unwrap()));
            }
        }
        "/" => {
            let booll = left.is_number();
            let boolr = right.is_number();
            if booll && boolr {
                if left.is_i64() && right.is_i64(){
                    return Result::Ok(json!(left.as_i64().unwrap() / right.as_i64().unwrap()));
                }
                return Result::Ok(json!(left.as_f64().unwrap() / right.as_f64().unwrap()));
            }
        }
        "%" => {
            let booll = left.is_number();
            let boolr = right.is_number();
            if booll && boolr {
                if left.is_i64() && right.is_i64(){
                    return Result::Ok(json!(left.as_i64().unwrap() % right.as_i64().unwrap()));
                }
                let l = left.as_f64().unwrap();
                let r = right.as_f64().unwrap();
                let result = l % r;
                return Result::Ok(json!(result));
            }
        }
        "^" => {
            let booll = left.is_i64();
            let boolr = right.is_i64();
            if booll == false || boolr == false {
                return Result::Err(crate::core::Error::from(format!("[rbatis] only support 'int ** int'! express:{}{}{}", left, op, right)));
            }
            if booll && boolr {
                if left.is_i64() && right.is_i64(){
                    return Result::Ok(json!(left.as_i64().unwrap() ^ right.as_i64().unwrap()));
                }
                let l = left.as_i64().unwrap();
                let r = right.as_i64().unwrap();
                let result = l ^ r;
                return Result::Ok(json!(result));
            }
        }
        "**" => {
            let booll = left.is_number();
            let boolr = right.is_u64();
            if boolr == false {
                return Result::Err(crate::core::Error::from(format!("[rbatis] only support 'number ** uint'! express:{}{}{}", left, op, right)));
            }
            if booll && boolr {
                let left_v = left.as_i64().unwrap() as f64;
                let right_v = right.as_i64().unwrap();
                return Result::Ok(json!(left_v.powi(right_v as i32)));
            }
        }
        "+" => {
            if left.is_null() && right.is_number() {
                return Result::Ok(right.clone());
            }
            if right.is_null() && left.is_number() {
                return Result::Ok(left.clone());
            }
            if left.is_number() && right.is_number() {
                if left.is_i64() && right.is_i64(){
                    return Result::Ok(json!(left.as_i64().unwrap() + right.as_i64().unwrap()));
                }
                return Result::Ok(json!(left.as_f64().unwrap() + right.as_f64().unwrap()));
            } else if left.is_string() && right.is_string() {
                return Result::Ok(Value::from(left.as_str().unwrap().to_owned() + right.as_str().unwrap()));
            } else {
                return Result::Err(crate::core::Error::from("[rbatis] un support diffrent type '+' opt"));
            }
        }
        "-" => {
            if left.is_null() && right.is_number() {
                if right.is_i64(){
                    return Result::Ok(json!(0 - right.as_i64().unwrap()));
                }
                return Result::Ok(json!(0.0 - right.as_f64().unwrap()));
            }
            if right.is_null() && left.is_number() {
                return Result::Ok(left.clone());
            }
            if left.is_number() && right.is_number() {
                if left.is_i64() && right.is_i64(){
                    return Result::Ok(json!(left.as_i64().unwrap() - right.as_i64().unwrap()));
                }
                return Result::Ok(json!(left.as_f64().unwrap() - right.as_f64().unwrap()));
            }
        }
        _ => {}
    }
    return Result::Err(crate::core::Error::from(format!("[rbatis] eval error express:{} {} {}", left, op, right)));
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
    } else if left.is_array() && !right.is_array() {
        return false;
    } else if left.is_object() && !right.is_object() {
        return false;
    } else if left.is_array() && right.is_array() {
        return is_eq_array(left.as_array().unwrap(), right.as_array().unwrap());
    } else if left.is_object() && right.is_object() {
        return is_eq_object(left.as_object().unwrap(), right.as_object().unwrap());
    } else {
        return false;
    }
}

fn is_eq_array(lefts: &Vec<Value>, rights: &Vec<Value>) -> bool {
    if lefts.len() != rights.len() {
        return false;
    }
    for left in lefts {
        for right in rights {
            if eq(&left, &right) == false {
                return false;
            }
        }
    }
    return true;
}

fn is_eq_object(lefts: &Map<String, Value>, rights: &Map<String, Value>) -> bool {
    if lefts.len() != rights.len() {
        return false;
    }
    for (k, left) in lefts {
        let right = rights.get(k);
        if right.is_none() {
            return false;
        }
        let right = right.unwrap();
        if eq(left, right) == false {
            return false;
        }
    }
    return true;
}


#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use crate::engine::eval::eval;
    use crate::utils::time_util;

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
}