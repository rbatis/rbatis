use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Map, Value};
use serde_json;
use serde_json::json;
use serde_json::value::Value::{Null, Number};

use crate::interpreter::json::eval::eval;
use crate::interpreter::json::node::NodeType::{NArg, NBinary, NBool, NNull, NNumber, NOpt, NString};
use crate::interpreter::json::runtime::OptMap;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum NodeType {
    NArg = 1,
    //参数节点
    NString = 2,
    //string 节点
    NNumber = 3,
    //number节点
    NBool = 4,
    //bool节点
    NNull = 5,
    //空节点
    NBinary = 6,
    //二元计算节点
    NOpt = 7,           //操作符节点
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            NArg => f.write_str("NArg"),
            NString => f.write_str("NString"),
            NNumber => f.write_str("NNumber"),
            NBool => f.write_str("NBool"),
            NNull => f.write_str("NNull"),
            NBinary => f.write_str("NBinary"),
            NOpt => f.write_str("NOpt"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub left: Option<Box<Node>>,
    pub value: Value,
    pub right: Option<Box<Node>>,
    pub node_type: NodeType,
}

impl Node {
    pub fn to_number(&self) -> f64 {
        return self.value.as_f64().unwrap();
    }
    pub fn to_string(&self) -> &str {
        return self.value.as_str().unwrap();
    }
    pub fn to_arg(&self) -> &str {
        return self.value.as_str().unwrap();
    }
    pub fn to_bool(&self) -> bool {
        return self.value.as_bool().unwrap();
    }
    pub fn to_opt(&self) -> &str {
        return self.value.as_str().unwrap();
    }
    pub fn node_type(&self) -> NodeType {
        return self.node_type.clone();
    }

    pub fn equal_node_type(&self, arg: &NodeType) -> bool {
        return self.node_type == *arg;
    }

    pub fn is_value_node(&self) -> Option<Value> {
        if self.equal_node_type(&NBinary) {
            return Option::None;
        } else if self.equal_node_type(&NArg) {
            return Option::None;
        } else {
            return Option::Some(self.value.clone());
        }
    }

    pub fn eval(&self, env: &Value) -> Result<Value, crate::core::Error> {
        if self.equal_node_type(&NBinary) {
            let left_v = self.left.as_ref().unwrap().eval(env).unwrap_or(Value::Null);
            let right_v = self.right.as_ref().unwrap().eval(env).unwrap_or(Value::Null);
            let opt = self.to_string();
            return eval(&left_v, &right_v, opt);
        } else if self.equal_node_type(&NArg) {
            let arr = self.value.as_array().unwrap();
            let arr_len = arr.len() as i32;
            if arr_len == 0 {
                return Result::Ok(Value::Null);
            }
            let mut index = 0;
            let mut v = env;
            for item in arr {
                let item_str = item.as_str().unwrap();
                if v.is_object() {
                    v = v.get(item_str).unwrap_or(&Value::Null);
                } else if v.is_array() {
                    let item_index = item_str.parse::<usize>();
                    if item_index.is_err() {
                        return Result::Ok(serde_json::Value::Null);
                    }
                    let item_index = item_index.unwrap();
                    let arr_ref = v.as_array().unwrap();
                    v = arr_ref.get(item_index).unwrap_or(&Value::Null);
                }
                if v.is_null() || index + 1 >= arr_len {
                    return Result::Ok(v.clone());
                }
                index = index + 1;
            }
            return Result::Ok(Value::Null);
        }
        return Result::Ok(self.value.clone());
    }

    pub fn opt(&self) -> Option<&str> {
        return self.value.as_str();
    }


    pub fn new_null() -> Self {
        Self {
            value: Value::Null,
            left: None,
            right: None,
            node_type: NNull,
        }
    }
    pub fn new_arg(arg: &str) -> Self {
        let new_arg = arg.replace("]", "").replace("[", ".");
        let d: Vec<&str> = new_arg.split(".").collect();
        Self {
            value: json!(d),
            left: None,
            right: None,
            node_type: NArg,
        }
    }
    pub fn new_string(arg: &str) -> Self {
        Self {
            value: Value::String(arg.to_string()),
            left: None,
            right: None,
            node_type: NString,
        }
    }
    pub fn new_number_f64(arg: f64) -> Self {
        Self {
            value: json!(arg),
            left: None,
            right: None,
            node_type: NNumber,
        }
    }
    pub fn new_number_i64(arg: i64) -> Self {
        Self {
            value: json!(arg),
            left: None,
            right: None,
            node_type: NNumber,
        }
    }
    pub fn new_number_u64(arg: u64) -> Self {
        Self {
            value: json!(arg),
            left: None,
            right: None,
            node_type: NNumber,
        }
    }

    pub fn new_bool(arg: bool) -> Self {
        Self {
            value: Value::Bool(arg),
            left: None,
            right: None,
            node_type: NBool,
        }
    }
    pub fn new_binary(arg_lef: Node, arg_right: Node, opt: &str) -> Self {
        Self {
            value: Value::from(opt),
            left: Option::Some(Box::new(arg_lef)),
            right: Option::Some(Box::new(arg_right)),
            node_type: NBinary,
        }
    }
    pub fn new_opt(arg: &str) -> Self {
        Self {
            value: Value::String(arg.to_string()),
            left: None,
            right: None,
            node_type: NOpt,
        }
    }

    pub fn parse(data: &str, opt: &OptMap) -> Self {
        // println!("data={}", &data);
        let mut first_index = 0;
        let mut last_index = 0;
        if data.rfind("'").unwrap_or(0) != 0 {
            first_index = data.find("'").unwrap_or_default();
            last_index = data.rfind("'").unwrap_or_default();
        }
        if data.rfind("`").unwrap_or(0) != 0 {
            first_index = data.find("`").unwrap_or_default();
            last_index = data.rfind("`").unwrap_or_default();
        }
        if data == "" || data == "null" {
            return Node::new_null();
        } else if let Ok(n) = data.parse::<bool>() {
            return Node::new_bool(n);
        } else if opt.is_opt(data) {
            return Node::new_opt(data);
        } else if first_index == 0 && last_index == (data.len() - 1) && first_index != last_index {
            let new_str = data.replace("'", "").replace("`", "");
            return Node::new_string(new_str.as_str());
        } else if let Ok(n) = data.parse::<f64>() {
            if data.find(".").unwrap_or(0) != 0 {
                return Node::new_number_f64(n);
            } else {
                return Node::new_number_i64(n as i64);
            }
        } else {
            return Node::new_arg(data);
        }
    }
}