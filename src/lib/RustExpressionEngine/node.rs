use std::collections::HashMap;
use crate::lib::RustExpressionEngine::node::NodeType::{NString, NArg, NNumber, NBool, NNull, NBinary, NOpt};
use serde_json::{Value, Map};
use serde_json::value::Value::{Number, Null};
use serde_json;
use serde_json::de::ParserNumber;
use std::ptr::null;
use crate::lib::RustExpressionEngine::eval::Eval;
use std::fmt::{Display, Formatter, Error};
use crate::lib::RustExpressionEngine::runtime::{IsNumber, OptMap, ParserTokens};

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
pub trait Node: Clone {
    fn Type(&self) -> NodeType;
    fn Eval(&self, env: &Value) -> (Value, String);
    fn Value(&self) -> Value;
    fn New(data: String) -> Self;
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

    fn New(data: String) -> Self {
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
        return ArgNode::New(self.Value().as_str().unwrap().to_string());
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

    fn New(data: String) -> Self {
        let pars: Vec<&str> = data.split('.').collect();
        let mut pars2 = vec![];
        for item in &pars {
            pars2.push(item.to_string());
        }
        let len = &pars.len();
        return Self {
            value: data.to_string(),
            t: NArg,
            params: pars2,
            paramsLen: len.clone(),
        };
    }
}


//String节点，值节点
pub struct StringNode {
    pub value: String,
    pub t: NodeType,
}

impl Clone for StringNode {
    fn clone(&self) -> Self {
        return StringNode::New(self.value.clone());
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

    fn New(data: String) -> Self {
        Self {
            value: data,
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

    fn New(data: String) -> Self {
        if data.as_str() == "" {
            return Self {
                value: Value::Number(serde_json::Number::from(ParserNumber::I64(0))),
                t: NNumber,
            };
        }
        let index = data.find(".").unwrap_or_default();
        if index > 0 {
            //i64
            let r: f64 = data.parse().unwrap();
            return Self {
                value: Value::Number(serde_json::Number::from(ParserNumber::F64(r))),
                t: NNumber,
            };
        } else {
            let r: i64 = data.parse().unwrap();
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

    fn New(data: String) -> Self {
        let r: bool = data.parse().unwrap_or_default();
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

    fn New(data: String) -> Self {
        Self {
            value: Value::Null,
            t: NNull,
        }
    }
}


//计算节点
pub struct BinaryNode {
    left: NodeItem,
    right: NodeItem,
    opt: String,
    t: NodeType,
}

impl Clone for BinaryNode {
    fn clone(&self) -> BinaryNode {
        return BinaryNode {
            left: self.left.clone(),
            right: self.right.clone(),
            opt: self.opt.clone(),
            t: self.t.clone(),
        };
    }
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

    fn New(v: String) -> Self {
        unimplemented!();
    }
}

impl BinaryNode {
    pub fn New(v: String, v2: String, opt: String) -> Self {
        Self {
            left: NodeItem::New(v),
            right: NodeItem::New(v2),
            opt: opt,
            t: NNumber,
        }
    }
}

//节点
pub struct NodeItem {
    Data: String,

    NArg: ArgNode,
    //参数节点
    NString: StringNode,
    //string 节点
    NNumber: NumberNode,
    //number节点
    NBool: BoolNode,
    //bool节点
    NNull: NullNode,
    //二元计算节点
    NOpt: OptNode,

    t: NodeType,
}

impl Clone for NodeItem {
    fn clone(&self) -> Self {
        return NodeItem::New(self.Data.clone());
    }
}

impl Node for NodeItem {
    fn Type(&self) -> NodeType {
        return self.t.clone();
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        match self.t {
            NArg => return self.NArg.Eval(env),
            //参数节点
            NString => return self.NString.Eval(env),
            //string 节点
            NNumber => return self.NNumber.Eval(env),
            //number节点
            NBool => return self.NBool.Eval(env),
            //bool节点
            NNull => return self.NNull.Eval(env),
            //二元计算节点
            NOpt => return self.NOpt.Eval(env),
            _ => return self.NNull.Eval(env),
        }
    }

    fn Value(&self) -> Value {
        match self.t {
            NArg => return self.NArg.Value(),
            //参数节点
            NString => return self.NString.Value(),
            //string 节点
            NNumber => return self.NNumber.Value(),
            //number节点
            NBool => return self.NBool.Value(),
            //bool节点
            NNull => return self.NNull.Value(),
            //二元计算节点
            NOpt => return self.NOpt.Value(),
            _ => return self.NNull.Value(),
        }
    }

    fn New(data: String) -> Self {
        let opt = OptMap::new();
        let mut t;
        let firstIndex = data.find("'").unwrap_or_default();
        let lastIndex = data.rfind("'").unwrap_or_default();

        if data.as_str() == "" {
            t = NNull;
            return Self {
                Data: data.clone(),
                NArg: ArgNode::New("".to_string()),
                //参数节点
                NString: StringNode::New("".to_string()),
                //string 节点
                NNumber: NumberNode::New("0".to_string()),
                //number节点
                NBool: BoolNode::New("".to_string()),
                //bool节点
                NNull: NullNode::New("".to_string()),
                //空节点
                //NBinary: BinaryNode::new(NullNode::new(),NullNode::new(),"".to_string()),
                //二元计算节点
                NOpt: OptNode::New("".to_string()),
                t: t.clone(),
            };
        } else if data.as_str() == "true" || data.as_str() == "false" {
            t = NBool;

            return Self {
                Data: data.clone(),
                NArg: ArgNode::New("".to_string()),
                //参数节点
                NString: StringNode::New("".to_string()),
                //string 节点
                NNumber: NumberNode::New("0".to_string()),
                //number节点
                NBool: BoolNode::New(data.clone()),
                //bool节点
                NNull: NullNode::New("".to_string()),
                //空节点
                //NBinary: BinaryNode::new(NullNode::new(),NullNode::new(),"".to_string()),
                //二元计算节点
                NOpt: OptNode::New("".to_string()),
                t: t.clone(),
            };
        } else if opt.isOpt(data.clone()) {
            t = NOpt;

            return Self {
                Data: data.clone(),
                NArg: ArgNode::New("".to_string()),
                //参数节点
                NString: StringNode::New("".to_string()),
                //string 节点
                NNumber: NumberNode::New("0".to_string()),
                //number节点
                NBool: BoolNode::New("".to_string()),
                //bool节点
                NNull: NullNode::New("".to_string()),
                //空节点
                //NBinary: BinaryNode::new(NullNode::new(),NullNode::new(),"".to_string()),
                //二元计算节点
                NOpt: OptNode::New(data.clone()),
                t: t.clone(),
            };
        } else if firstIndex == 0 && lastIndex == (data.len() - 1) && firstIndex != lastIndex {
            t = NString;

            return Self {
                Data: data.clone(),
                NArg: ArgNode::New("".to_string()),
                //参数节点
                NString: StringNode::New("".to_string()),
                //string 节点
                NNumber: NumberNode::New("0".to_string()),
                //number节点
                NBool: BoolNode::New("".to_string()),
                //bool节点
                NNull: NullNode::New("".to_string()),
                //空节点
                //NBinary: BinaryNode::new(NullNode::new(),NullNode::new(),"".to_string()),
                //二元计算节点
                NOpt: OptNode::New("".to_string()),
                t: t.clone(),
            };
        } else if IsNumber(&data) {
            t = NNumber;

            return Self {
                Data: data.clone(),
                NArg: ArgNode::New("".to_string()),
                //参数节点
                NString: StringNode::New("".to_string()),
                //string 节点
                NNumber: NumberNode::New("0".to_string()),
                //number节点
                NBool: BoolNode::New("".to_string()),
                //bool节点
                NNull: NullNode::New("".to_string()),
                //空节点
                //NBinary: BinaryNode::new(NullNode::new(),NullNode::new(),"".to_string()),
                //二元计算节点
                NOpt: OptNode::New("".to_string()),
                t: t.clone(),
            };
        } else {
            t = NArg;

            return Self {
                Data: data.clone(),
                NArg: ArgNode::New("".to_string()),
                //参数节点
                NString: StringNode::New("".to_string()),
                //string 节点
                NNumber: NumberNode::New("0".to_string()),
                //number节点
                NBool: BoolNode::New("".to_string()),
                //bool节点
                NNull: NullNode::New("".to_string()),
                //空节点
                //NBinary: BinaryNode::new(NullNode::new(),NullNode::new(),"".to_string()),
                //二元计算节点
                NOpt: OptNode::New("".to_string()),
                t: t.clone(),
            };
        }
    }
}


