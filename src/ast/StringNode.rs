use crate::utils::string_util;
use crate::ast::Node::Node;
use serde_json::Value;

/**
*  string抽象节点
**/
pub struct StringNode {
    pub value: String,
    //去重的，需要替换的要sql转换express map
    pub expressMap: Vec<String>,
    //去重的，需要替换的免sql转换express map
    pub noConvertExpressMap: Vec<String>,
}

impl StringNode {
   pub fn new(v: &str) -> Self {
        //TODO find v #[] and find v$[]
        Self {
            value: v.to_string(),
            expressMap: string_util::findConvertString(v.to_string()),
            noConvertExpressMap: string_util::findNoConvertString(v.to_string()),
        }
    }
}

impl Node for StringNode {
    fn eval(&self, env: &Value) -> String {
        let mut result = self.value.clone();
        for item in &self.expressMap {
            result = result.replace(("#{".to_owned()+item.as_str()+"}").as_str(), env.get(item).unwrap_or(&Value::String(String::new())).as_str().unwrap_or(""));
        }
        return result;
    }
}