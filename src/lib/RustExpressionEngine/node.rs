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
#[derive(Clone)]
pub struct Node {
    pub Data: Option<String>,
    pub NArg: Option<String>,
    pub  NString: Option<String>,
    pub  NNumber: Option<f64>,
    pub  NBool: Option<bool>,
    //bool节点
    pub NNull: Option<bool>,
    pub  NBinaryLeft: Option<Rc<Node>>,
    pub  NBinaryRight: Option<Rc<Node>>,
    pub  NOpt: Option<String>,
    pub t: Option<NodeType>,
}

impl Node {
    pub fn n_type(&self) -> NodeType {
        return self.t.clone().unwrap();
    }
    pub fn eval(&mut self, env: &Value) -> Node {
        let mut result = Node {
            Data: None,
            NArg: None,
            NString: None,
            NNumber: None,
            NBool: None,
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: None,
            t: Option::Some(NNull),
        };
        let leftV = self.NBinaryLeft.clone().unwrap().NNumber.unwrap();
        let rightV = self.NBinaryRight.clone().unwrap().NNumber.unwrap();
        result.NNumber = Option::Some(leftV + rightV);
        result.t = Option::Some(NNumber);
        //let nn=self.NBinaryLeft.unwrap() self.NBinaryRight.unwrap().Eval(env).NNumber.unwrap();
        match self.t.clone().unwrap() {
            NNumber => return result,
            NBinary => return result,
            _ => return result,
        }
    }

    pub fn opt(&self) ->  Option<String> {
        return self.NOpt.clone();
    }


    pub fn newNull() -> Self {
        Self {
            Data: None,
            NArg: None,
            NString: None,
            NNumber: None,
            NBool: None,
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: None,
            t: Option::Some(NNull),
        }
    }
    pub fn newArg(arg: String) -> Self {
        Self {
            Data: None,
            NArg: Option::Some(arg),
            NString: None,
            NNumber: None,
            NBool: None,
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: None,
            t: Option::Some(NArg),
        }
    }
    pub fn newString(arg: String) -> Self {
        Self {
            Data: None,
            NArg: None,
            NString: Option::Some(arg),
            NNumber: None,
            NBool: None,
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: None,
            t: Option::Some(NString),
        }
    }
    pub fn newNumber(arg: f64) -> Self {
        Self {
            Data: None,
            NArg: None,
            NString: None,
            NNumber: Option::Some(arg),
            NBool: None,
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: None,
            t: Option::Some(NNumber),
        }
    }
    pub fn newBool(arg: bool) -> Self {
        Self {
            Data: None,
            NArg: None,
            NString: None,
            NNumber: None,
            NBool: Option::Some(arg),
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: None,
            t: Option::Some(NBool),
        }
    }
    pub fn newBinary(argLef: Node, argRight: Node, opt: String) -> Self {
        Self {
            Data: None,
            NArg: None,
            NString: None,
            NNumber: None,
            NBool: None,
            NNull: None,
            NBinaryLeft: Option::Some(Rc::new(argLef)),
            NBinaryRight: Option::Some(Rc::new(argRight)),
            NOpt: Option::Some(opt),
            t: Option::Some(NBinary),
        }
    }
    pub fn newOpt(arg: String) -> Self {
        Self {
            Data: None,
            NArg: None,
            NString: None,
            NNumber: None,
            NBool: None,
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: Option::Some(arg),
            t: Option::Some(NOpt),
        }
    }

    //根据string 解析单个node
    pub fn parser(data: String) -> Self {
        let opt = OptMap::new();
        let firstIndex = data.find("'").unwrap_or_default();
        let lastIndex = data.rfind("'").unwrap_or_default();

        println!("{}",&data);

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
            let parsed=data.parse().unwrap();
            return Node::newNumber(parsed);
        } else {
            return Node::newArg(data);
        }
    }
}