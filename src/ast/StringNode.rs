use crate::utils::string_util;
use crate::ast::Node::Node;
use serde_json::Value;
use std::collections::HashMap;

/**
*  string抽象节点
**/
pub struct StringNode {
    pub value: String,
    //去重的，需要替换的要sql转换express map
    pub expressMap: HashMap<String,String>,
    //去重的，需要替换的免sql转换express map
    pub noConvertExpressMap: HashMap<String,String>,
}

impl StringNode {
   pub fn new(v: &str) -> Self {
        //TODO find v #[] and find v$[]
       let mut  expressMap=HashMap::new();
       for item in &string_util::findConvertString(v.to_string()){
           expressMap.insert(item.clone(),"#{".to_owned()+item.as_str()+"}");
       }
       let mut  noConvertExpressMap=HashMap::new();
       for item in &string_util::findNoConvertString(v.to_string()){
           noConvertExpressMap.insert(item.clone(),"${".to_owned()+item.as_str()+"}");
       }
        Self {
            value: v.to_string(),
            expressMap: expressMap,
            noConvertExpressMap: noConvertExpressMap,
        }
    }
}

impl Node for StringNode {
    fn eval(&self, env: &Value) -> String {
        let mut result = self.value.clone();
        for (item,value) in &self.expressMap {
            result = result.replace(value, env.get(item).unwrap_or(&Value::String(String::new())).as_str().unwrap_or(""));
        }
        return result;
    }
}