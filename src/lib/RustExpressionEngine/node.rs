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
#[derive(Clone)]
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




//抽象语法树节点
pub trait Node: Clone {
    fn Type(&self) -> NodeType;
    fn Eval(&self, env: &Value) -> (Value, String);
    fn Value(&self) -> Value;
    fn New(data: String) -> Self;
}

#[derive(Clone)]
pub struct OptNode {
    pub  value: Value,
    t: NodeType,
}



impl Node for OptNode {
    fn Type(&self) -> NodeType {
        return NOpt;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        return ((&self).value.clone(), String::new());
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
#[derive(Clone)]
pub struct ArgNode {
    //参数源
    pub value: String,
    //缓存分割包含有"."后的参数
    params: Vec<String>,
    //参数长度
    paramsLen: usize,
    pub t: NodeType,
}




impl Node for ArgNode {
    fn Type(&self) -> NodeType {
        return NArg;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        if self.params.len() == 1 {
            return (env[self.value.as_str()].clone(), String::new());
        } else {
            let paramsLen = self.params.len();
            let mut result = env;
            for i in 0..paramsLen {
                result = &result[&self.params[i]];
                if i == (paramsLen - 1) {
                    return (result.clone(), String::new());
                }
            }
            return (Value::Null, String::new());
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
#[derive(Clone)]
pub struct StringNode {
    pub value: String,
    pub t: NodeType,
}


impl Node for StringNode {
    fn Type(&self) -> NodeType {
        return NString;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        return (Value::String(self.value.to_string()), String::new());
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
#[derive(Clone)]
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
        return ((&self.value).clone(), String::new());
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
#[derive(Clone)]
pub struct BoolNode {
    value: Value,
    t: NodeType,
}



impl Node for BoolNode {
    fn Type(&self) -> NodeType {
        return NBool;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        return ((&self.value).clone(), String::new());
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

#[derive(Clone)]
pub struct NullNode {
    value: Value,
    t: NodeType,
}



impl Node for NullNode {
    fn Type(&self) -> NodeType {
        return NNull;
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        return ((&self.value).clone(), String::new());
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
#[derive(Clone)]
pub struct BinaryNode {
    left: NodeItem,
    right: NodeItem,
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
    pub fn NewItem(v: NodeItem, v2: NodeItem, opt: String) -> Self {
        Self {
            left: v,
            right: v2,
            opt: opt,
            t: NNumber,
        }
    }
}

//节点
#[derive(Clone)]
pub struct NodeItem {
    pub Data: Option<String>,

    NArg: Option<ArgNode>,
    //参数节点
    NString: Option<StringNode>,
    //string 节点
    pub  NNumber: Option<NumberNode>,
    //number节点
    NBool: Option<BoolNode>,
    //bool节点
    NNull: Option<NullNode>,
    //二元计算节点
    NOpt: Option<OptNode>,

    pub  NBinary: Option<Box<BinaryNode>>,

    t: NodeType,
}


impl Node for NodeItem {
    fn Type(&self) -> NodeType {
        return self.t.clone();
    }

    fn Eval(&self, env: &Value) -> (Value, String) {
        match self.t {
            NArg => return self.NArg.clone().unwrap().Eval(env),
            //参数节点
            NString => return self.NString.clone().unwrap().Eval(env),
            //string 节点
            NNumber => return self.NNumber.clone().unwrap().Eval(env),
            //number节点
            NBool => return self.NBool.clone().unwrap().Eval(env),
            //bool节点
            NNull => return self.NNull.clone().unwrap().Eval(env),
            //二元计算节点
            NOpt => return self.NOpt.clone().unwrap().Eval(env),

            NBinary => return self.NBinary.clone().unwrap().Eval(env),
            _ => return (Value::Null, String::new()),
        }
    }

    fn Value(&self) -> Value {
        match self.t {
            NArg => return self.NArg.clone().unwrap().Value(),
            //参数节点
            NString => return self.NString.clone().unwrap().Value(),
            //string 节点
            NNumber => return self.NNumber.clone().unwrap().Value(),
            //number节点
            NBool => return self.NBool.clone().unwrap().Value(),
            //bool节点
            NNull => return self.NNull.clone().unwrap().Value(),
            //二元计算节点
            NOpt => return self.NOpt.clone().unwrap().Value(),

            NBinary => return self.NBinary.clone().unwrap().Value(),
            _ => return Value::Null,
        }
    }

    fn New(data: String) -> Self {
        let opt = OptMap::new();
        let mut t;
        let firstIndex = data.find("'").unwrap_or_default();
        let lastIndex = data.rfind("'").unwrap_or_default();

        if data.as_str() == "" || data.as_str() == "null" {
            t = NNull;
            return Self {
                Data: Option::Some(data.clone()),
                NArg: Option::None,
                //参数节点
                NString: Option::None,
                //string 节点
                NNumber: Option::None,
                //number节点
                NBool: Option::None,
                //bool节点
                NNull: Option::Some(NullNode { value: Value::Null, t: NodeType::NArg }),
                //空节点
                NBinary: Option::None,
                //二元计算节点
                NOpt: Option::None,
                t: t.clone(),
            };
        } else if data.as_str() == "true" || data.as_str() == "false" {
            t = NBool;

            return Self {
                Data: Option::Some(data.clone()),
                NArg: Option::None,
                //参数节点
                NString: Option::None,
                //string 节点
                NNumber: Option::None,
                //number节点
                NBool: Option::Some(BoolNode::New(data)),
                //bool节点
                NNull: Option::None,
                //空节点
                NBinary: Option::None,
                //二元计算节点
                NOpt: Option::None,
                t: t.clone(),
            };
        } else if opt.isOpt(data.clone()) {
            t = NOpt;

            return Self {
                Data: Option::Some(data.clone()),
                NArg: Option::None,
                //参数节点
                NString: Option::None,
                //string 节点
                NNumber: Option::None,
                //number节点
                NBool: Option::None,
                //bool节点
                NNull: Option::None,
                //空节点
                NBinary: Option::None,
                //二元计算节点
                NOpt: Option::Some(OptNode::New(data)),
                t: t.clone(),
            };
        } else if firstIndex == 0 && lastIndex == (data.len() - 1) && firstIndex != lastIndex {
            t = NString;

            return Self {
                Data: Option::Some(data.clone()),
                NArg: Option::None,
                //参数节点
                NString: Option::Some(StringNode::New(data)),
                //string 节点
                NNumber: Option::None,
                //number节点
                NBool: Option::None,
                //bool节点
                NNull: Option::None,
                //空节点
                NBinary: Option::None,
                //二元计算节点
                NOpt: Option::None,
                t: t.clone(),
            };
        } else if IsNumber(&data) {
            t = NNumber;

            return Self {
                Data: Option::Some(data.clone()),
                NArg: Option::None,
                //参数节点
                NString: Option::None,
                //string 节点
                NNumber: Option::Some(NumberNode::New(data)),
                //number节点
                NBool: Option::None,
                //bool节点
                NNull: Option::None,
                //空节点
                NBinary: Option::None,
                //二元计算节点
                NOpt: Option::None,
                t: t.clone(),
            };
        } else {
            t = NArg;

            return Self {
                Data: Option::Some(data.clone()),
                NArg: Option::Some(ArgNode::New(data)),
                //参数节点
                NString: Option::None,
                //string 节点
                NNumber: Option::None,
                //number节点
                NBool: Option::None,
                //bool节点
                NNull: Option::None,
                //空节点
                NBinary: Option::None,
                //二元计算节点
                NOpt: Option::None,
                t: t.clone(),
            };
        }
    }
}

impl NodeItem {
    pub fn NewNBinaryNode(node: BinaryNode) -> Self {
        return Self {
            Data: Option::None,
            NArg: Option::None,
            //参数节点
            NString: Option::None,
            //string 节点
            NNumber: Option::None,
            //number节点
            NBool: Option::None,
            //bool节点
            NNull: Option::None,
            //空节点
            NBinary: Option::Some(Box::new(node)),
            //二元计算节点
            NOpt: Option::None,
            t: NBinary,
        };
    }
    pub fn NewNBinary(left: NodeItem, right: NodeItem, opt: String) -> Self {
        return Self {
            Data: Option::None,
            NArg: Option::None,
            //参数节点
            NString: Option::None,
            //string 节点
            NNumber: Option::None,
            //number节点
            NBool: Option::None,
            //bool节点
            NNull: Option::None,
            //空节点
            NBinary: Option::Some(Box::new(BinaryNode::NewItem(left, right, opt))),
            //二元计算节点
            NOpt: Option::None,
            t: NBinary,
        };
    }
}

