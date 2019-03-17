use std::collections::HashMap;
use crate::lib::RustExpressionEngine::node::NodeType::{NString, NArg};
use serde_json::{Value, Map};

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
    fn Eval(&self, env: &Value) -> (Value, &str);
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

    fn Eval(&self, env: &Value) -> (Value, &str) {
        if self.params.len() == 1 {
            return (env[self.value.as_str()].clone(), "");
        } else {
            let paramsLen = self.params.len();
            let mut result = env;
            for i in 0..paramsLen {
                result = &result[self.params[i]];
                if i == (paramsLen - 1) {
                    return (result.clone(), "");
                }
            }
            return (Value::Null, "");
        }
    }
}


//String节点
pub struct StringNode {
    pub value: String,
    pub t: NodeType,
}

impl Node for StringNode {
    fn Type(&self) -> NodeType {
        return NString;
    }

    fn Eval(&self, env: &Value) -> (Value, &str) {
        return (Value::String(self.value.to_string()), "");
    }
}