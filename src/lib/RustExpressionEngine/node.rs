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
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, PartialEq)]
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
#[derive(Clone)]
pub struct Node {
    pub Data: Value,
    pub  NBinaryLeft: Option<Rc<Node>>,
    pub  NBinaryRight: Option<Rc<Node>>,
    pub t: NodeType,
}

impl Node {
    pub fn toNumber(&self) -> f64 {
        return self.Data.as_f64().unwrap();
    }
    pub fn toString(&self) -> &str {
        return self.Data.as_str().unwrap();
    }
    pub fn toArg(&self) -> &str {
        return self.Data.as_str().unwrap();
    }
    pub fn toBool(&self) -> bool {
        return self.Data.as_bool().unwrap();
    }
    pub fn toNull(&self) -> () {
        return self.Data.as_null().unwrap();
    }
    pub fn toOpt(&self) -> &str {
        return self.Data.as_str().unwrap();
    }
    pub fn nodeType(&self) -> NodeType {
        return self.t.clone();
    }

    pub fn equalNodeType(&self, arg: &NodeType) -> bool {
        return self.t == *arg;
    }

    pub fn eval(&self, env: &Value) -> Value {
        if self.equalNodeType(&NBinary) {
            let leftV = self.NBinaryLeft.clone().unwrap().eval(env);
            let rightV = self.NBinaryRight.clone().unwrap().eval(env);
            let opt = self.toString();
            let (v, _) = Eval(&leftV, &rightV, opt);
            return v;
        }else if self.equalNodeType(&NArg){
            let v=env.get(self.Data.clone().as_str().unwrap());
            return v.unwrap_or(&Value::Null).clone();
        }
        return self.Data.clone();
    }

    pub fn opt(&self) -> Option<&str> {
        return self.Data.as_str();
    }


    pub fn newNull() -> Self {
        Self {
            Data: Value::Null,
            NBinaryLeft: None,
            NBinaryRight: None,
            t: NNull,
        }
    }
    pub fn newArg(arg: String) -> Self {
        Self {
            Data: Value::String(arg),
            NBinaryLeft: None,
            NBinaryRight: None,
            t: NArg,
        }
    }
    pub fn newString(arg: String) -> Self {
        Self {
            Data: Value::String(arg),
            NBinaryLeft: None,
            NBinaryRight: None,
            t: NString,
        }
    }
    pub fn newNumber(arg: f64) -> Self {
        Self {
            Data: Value::Number(serde_json::Number::from_f64(arg).unwrap()),
            NBinaryLeft: None,
            NBinaryRight: None,
            t: NNumber,
        }
    }
    pub fn newBool(arg: bool) -> Self {
        Self {
            Data: Value::Bool(arg),
            NBinaryLeft: None,
            NBinaryRight: None,
            t: NBool,
        }
    }
    pub fn newBinary(argLef: Node, argRight: Node, opt: &str) -> Self {
        Self {
            Data: Value::from(opt),
            NBinaryLeft: Option::Some(Rc::new(argLef)),
            NBinaryRight: Option::Some(Rc::new(argRight)),
            t: NBinary,
        }
    }
    pub fn newOpt(arg: String) -> Self {
        Self {
            Data: Value::String(arg),
            NBinaryLeft: None,
            NBinaryRight: None,
            t: NOpt,
        }
    }

    //根据string 解析单个node
    pub fn parser(data: String) -> Self {
        let opt = OptMap::new();
        let firstIndex = data.find("'").unwrap_or_default();
        let lastIndex = data.rfind("'").unwrap_or_default();

        println!("{}", &data);

        if data.as_str() == "" || data.as_str() == "null" {
            return Node::newNull();
        } else if data.as_str() == "true" || data.as_str() == "false" {
            if data.as_str() == "true" {
                return Node::newBool(true);
            } else {
                return Node::newBool(false);
            }
        } else if opt.isOpt(data.clone()) {
            return Node::newOpt(data.clone());
        } else if firstIndex == 0 && lastIndex == (data.len() - 1) && firstIndex != lastIndex {
            let newStr = data.replace("'", "").replace("`", "");
            return Node::newString(newStr);
        } else if IsNumber(&data) {
            let parsed = data.parse().unwrap();
            return Node::newNumber(parsed);
        } else {
            return Node::newArg(data);
        }
    }
}