use std::collections::HashMap;
use crate::lib::RustExpressionEngine::node::NodeType::{NString, NArg, NNumber, NBool, NNull, NBinary, NOpt};
use serde_json::{Value, Map};
use serde_json::value::Value::Number;
use serde_json;
use serde_json::de::ParserNumber;
use std::ptr::null;
use crate::lib::RustExpressionEngine::eval::Eval;

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


//抽象语法树节点
pub trait Node {
    fn Type(&self) -> NodeType;
    fn Eval(&self, env: &Value) -> (Value, String);
    fn Value(&self) -> Value;
}

pub struct OptNode {
    pub  value: Value,
    t: NodeType,
}

impl Node for OptNode {
    fn Type(&self) -> NodeType {
        return NOpt;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        return ((&self).value.clone(), "".to_string());
    }

    fn Value(&self) -> Value {
        return (&self).value.clone();
    }
}


impl OptNode {
    pub fn new(data: String) -> Self {
        Self {
            t: NOpt,
            value: Value::String(data),
        }
    }
}

//参数节点
pub struct ArgNode {
    //参数源
    pub value: String,
    //缓存分割包含有"."后的参数
    params: Vec<String>,
    //参数长度
    paramsLen: usize,
    pub t: NodeType,
}

impl ArgNode {
    pub fn new(v: &String) -> Self {
        let pars: Vec<&str> = v.split('.').collect();
        let mut pars2 = vec![];
        for item in &pars {
            pars2.push(item.to_string());
        }
        let len = &pars.len();
        return Self {
            value: v.to_string(),
            t: NArg,
            params: pars2,
            paramsLen: len.clone(),
        };
    }
}


impl Node for ArgNode {
    fn Type(&self) -> NodeType {
        return NArg;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        if self.params.len() == 1 {
            return (env[self.value.as_str()].clone(), "".to_string());
        } else {
            let paramsLen = self.params.len();
            let mut result = env;
            for i in 0..paramsLen {
                result = &result[&self.params[i]];
                if i == (paramsLen - 1) {
                    return (result.clone(), "".to_string());
                }
            }
            return (Value::Null, "".to_string());
        }
    }
    fn Value(&self) -> Value {
        return Value::String((&self).value.clone());
    }
}


//String节点，值节点
pub struct StringNode {
    pub value: String,
    pub t: NodeType,
}

impl Node for StringNode {
    fn Type(&self) -> NodeType {
        return NString;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        return (Value::String(self.value.to_string()), "".to_string());
    }

    fn Value(&self) -> Value {
        return Value::String((&self).value.clone());
    }
}


impl StringNode {
    pub fn new(s: String) -> Self {
        Self {
            value: s,
            t: NString,
        }
    }
}

//number节点,值节点
pub struct NumberNode {
    value: Value,
    //u64,i64,f64
    pub t: NodeType,
}


impl Node for NumberNode {
    fn Type(&self) -> NodeType {
        return NNumber;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        return ((&self.value).clone(), "".to_string());
    }

    fn Value(&self) -> Value {
        return (&self).value.clone();
    }
}

impl NumberNode {
    pub fn new(value: &String) -> Self {
        let index = value.find(".").unwrap_or_default();
        if index > 0 {
            //i64
            let r: f64 = value.parse().unwrap();
            return Self {
                value: Value::Number(serde_json::Number::from(ParserNumber::F64(r))),
                t: NNumber,
            };
        } else {
            let r: i64 = value.parse().unwrap();
            return Self {
                value: Value::Number(serde_json::Number::from(ParserNumber::I64(r))),
                t: NNumber,
            };
        }
    }
}

pub struct BoolNode {
    value: Value,
    t: NodeType,
}


impl Node for BoolNode {
    fn Type(&self) -> NodeType {
        return NBool;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        return ((&self.value).clone(), "".to_string());
    }
    fn Value(&self) -> Value {
        return (&self).value.clone();
    }
}

impl BoolNode {
    pub fn new(value: String) -> Self {
        let r: bool = value.parse().unwrap();
        Self {
            value: Value::Bool(r),
            t: NNumber,
        }
    }
}


pub struct NullNode {
    value: Value,
    t: NodeType,
}


impl Node for NullNode {
    fn Type(&self) -> NodeType {
        return NNull;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        return ((&self.value).clone(), "".to_string());
    }
    fn Value(&self) -> Value {
        return (&self).value.clone();
    }
}

impl NullNode {
    pub fn new() -> Self {
        Self {
            value: Value::Null,
            t: NNumber,
        }
    }
}


//计算节点
pub struct BinaryNode {
    left: Box<Node>,
    right: Box<Node>,
    opt: String,
    t: NodeType,
}

impl Node for BinaryNode {
    fn Type(&self) -> NodeType {
        return NBinary;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        let (l, e) = self.left.Eval(env);
        if e != "" {
            return (Value::Null, e);
        }
        let (r, e) = self.right.Eval(env);
        if e != "" {
            return (Value::Null, e);
        }
        return Eval(&l, &r, &self.opt);
    }
    fn Value(&self) -> Value {
        return Value::Null;
    }
}

//<Left: Node, Right: Node>
impl BinaryNode {
    pub fn new(left: Box<Node>, right: Box<Node>, opt: String) -> Self {
        Self {
            left: left,
            right: right,
            opt: opt,
            t: NNumber,
        }
    }
}