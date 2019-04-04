use super::NodeType::NodeType;
use std::collections::HashMap;
use serde_json::Value;

/**
* Abstract syntax tree node
*/
pub trait SqlNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String>;
}


//执行子所有节点
pub fn DoChildNodes(childNodes: &mut Vec<NodeType>, env: &mut Value) -> Result<String, String> {
    let mut s = String::new();
    for item in childNodes {
        let itemResult = item.eval(env);
        if !itemResult.is_ok() {
            return itemResult;
        }
        s = s + itemResult.unwrap().as_str();
    }
    return Result::Ok(s);
}