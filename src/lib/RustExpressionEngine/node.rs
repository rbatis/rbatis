use std::collections::HashMap;
use crate::lib::RustExpressionEngine::node::NodeType::{NString, NArg, NNumber, NBool, NNull, NBinary, NOpt};
use serde_json::{Value, Map};
use serde_json::value::Value::Number;
use serde_json;
use serde_json::de::ParserNumber;
use std::ptr::null;
use crate::lib::RustExpressionEngine::eval::Eval;

pub enum NodeType {
    NArg,
    //参数节点
    NString,
    //string 节点
    NNumber,
    //number节点
    NBool,
    //bool节点
    NNull,
    //空节点
    NBinary,
    //二元计算节点
    NOpt,           //操作符节点
}


//抽象语法树节点
pub trait Node {
    fn Type(&self) -> NodeType;
    fn Eval(&self, env: &Value) -> (Value, String);
}

pub struct OptNode {
    value: Value,
    t: NodeType,
}

impl Node for OptNode {
    fn Type(&self) -> NodeType {
        return NOpt;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        return ((&self).value.clone(), "".to_string());
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
pub struct ArgNode<'a> {
    //参数源
    pub value: String,
    //缓存分割包含有"."后的参数
    params: Vec<&'a str>,
    //参数长度
    paramsLen: usize,
    pub t: NodeType,
}

impl<'a> ArgNode<'a> {
    pub fn new(v: &'a str) -> Self {
        let pars: Vec<&'a str> = v.split('.').collect();
        let len = pars.len();
        return Self {
            value: v.to_string(),
            t: NArg,
            params: pars,
            paramsLen: len,
        };
    }
}


impl<'a> Node for ArgNode<'a> {
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
                result = &result[self.params[i]];
                if i == (paramsLen - 1) {
                    return (result.clone(), "".to_string());
                }
            }
            return (Value::Null, "".to_string());
        }
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
pub struct BinaryNode<Left: Node, Right: Node> {
    left: Left,
    right: Right,
    opt: String,
    t: NodeType,
}

impl<Left: Node, Right: Node> Node for BinaryNode<Left, Right> {
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
}

//<Left: Node, Right: Node>
impl<Left: Node, Right: Node> BinaryNode<Left, Right> {
    pub fn new(left: Left, right: Right, opt: String) -> Self {
        Self {
            left: left,
            right: right,
            opt: opt,
            t: NNumber,
        }
    }
}