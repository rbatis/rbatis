use std::collections::HashMap;
use crate::lib::RustExpressionEngine::node::NodeType::NString;
use serde_json::Value;

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
    fn Eval(&self, env: Value) -> (Value, String);
}

pub struct StringNode {
    pub value: String,
    pub t: NodeType,
}

impl Node for StringNode {
    fn Type(&self) -> NodeType {
        return NString;
    }

    fn Eval(&self, env: Value) -> (Value, String) {
        return (Value::String(self.value.to_string()), String::from(""));
    }
}