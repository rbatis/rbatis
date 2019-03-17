use std::collections::HashMap;

pub enum NodeType {
  NArg,            //参数节点
  NString,          //string 节点
  NNumber,           //number节点
  NBool,           //bool节点
  NNull,           //空节点
  NBinary,         //二元计算节点
  NOpt,           //操作符节点
}

//抽象语法树节点
pub trait Node {
    fn Type() -> NodeType;
    fn Eval(env: serde_json::Value) -> (serde_json::Value, String);
}