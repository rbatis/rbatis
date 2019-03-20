use std::collections::HashMap;
use crate::lib::RustExpressionEngine::node::NodeType::{NString, NArg, NNumber, NBool, NNull, NBinary, NOpt};
use serde_json::{Value, Map};
use serde_json::value::Value::{Number, Null};
use serde_json;
use serde_json::de::ParserNumber;
use std::ptr::null;
use crate::lib::RustExpressionEngine::eval::Eval;
use std::fmt::{Display, Formatter, Error};

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


impl Clone for NodeType {
    fn clone(&self) -> Self {
        match self {
            NArg => return NArg,
            NString => return NString,
            NNumber => return NNumber,
            NBool => return NBool,
            NNull => return NNull,
            NBinary => return NBinary,
            NOpt => return NOpt,
        }
    }
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

impl Clone for OptNode {
    fn clone(&self) -> Self {
        return OptNode {
            value: self.value.clone(),
            t: self.t.clone(),
        };
    }
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

impl Clone for ArgNode {
    fn clone(&self) -> Self {
        return ArgNode::new(&self.Value().as_str().unwrap().to_string());
    }
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

impl Clone for StringNode {
    fn clone(&self) -> Self {
        return StringNode::new(self.value.clone());
    }
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

impl Clone for NumberNode {
    fn clone(&self) -> Self {
        return NumberNode {
            t: NNumber,
            value: self.value.clone(),
        };
    }
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

impl Clone for BoolNode {
    fn clone(&self) -> Self {
        return BoolNode {
            t: NBool,
            value: self.value.clone(),
        };
    }
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

impl Clone for NullNode {
    fn clone(&self) -> Self {
        return NullNode {
            t: NNull,
            value: self.value.clone(),
        };
    }
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
pub struct BinaryNode<Left: Node, Right: Node> {
    left: Left,
    right: Right,
    opt: String,
    t: NodeType,
}

impl<Left: Node + Clone, Right: Node + Clone> Clone for BinaryNode<Left, Right> {
    fn clone(&self) -> BinaryNode<Left, Right> {
        return BinaryNode {
            left: self.left.clone(),
            right: self.right.clone(),
            opt: self.opt.clone(),
            t: self.t.clone(),
        };
    }
}

impl<Left: Node + Clone, Right: Node + Clone> Node for BinaryNode<Left, Right> {
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
impl<Left: Node + Clone, Right: Node + Clone> BinaryNode<Left, Right> {
    pub fn new(left: Left, right: Right, opt: String) -> Self {
        Self {
            left: left,
            right: right,
            opt: opt,
            t: NNumber,
        }
    }
}
//
////节点
//pub struct NodeItem {
//    NArg: ArgNode,
//    //参数节点
//    NString: StringNode,
//    //string 节点
//    NNumber: NumberNode,
//    //number节点
//    NBool: BoolNode,
//    //bool节点
//    NNull: NullNode,
//    //空节点
//    NBinary: BinaryNode,
//    //二元计算节点
//    NOpt: OptNode,
//
//    t: NodeType,
//}
//
//impl Node for NodeItem {
//    fn Type(&self) -> NodeType {
//        return self.t.clone();
//    }
//    fn Eval(&self, env: &Value) -> (Value, String) {
//        return self.node.Eval(env);
//    }
//    fn Value(&self) -> Value {
//        return self.node.Value();
//    }
//}
//
//impl Clone for NodeItem {
//    fn clone(&self) -> Self {
//        return NodeItem::new(self.node.clone());
//    }
//}
//
//impl NodeItem {
//    fn newNArg(NArg: ArgNode) -> Self {
//        return Self {
//            NArg: NArg,
//            //参数节点
//            NString: StringNode ::new("".to_string()),
//            //string 节点
//            NNumber: NumberNode::new(&"0".to_string()),
//            //number节点
//            NBool: BoolNode::new("".to_string()),
//            //bool节点
//            NNull: NullNode::new(),
//            //空节点
//            NBinary: BinaryNode::new(NullNode::new(),NullNode::new(),"".to_string()),
//            //二元计算节点
//            NOpt: OptNode,
//
//            t: NodeType,
//        };
//    }
//}
//
