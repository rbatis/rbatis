use crate::utils::string_util;
use crate::ast::node::{SqlNode, create_deep};
use serde_json::Value;
use std::collections::HashMap;
use crate::ast::convert::sql_arg_type_convert::SqlArgTypeConvert;
use std::rc::Rc;
use crate::engines;

use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use crate::ast::node_config_holder::NodeConfigHolder;

/**
*  string抽象节点
**/
#[derive(Clone)]
pub struct StringNode {
    pub value: String,
    //去重的，需要替换的要sql转换express map
    pub expressMap: HashMap<String, String>,
    //去重的，需要替换的免sql转换express map
    pub noConvertExpressMap: HashMap<String, String>,
}

impl StringNode {

    pub fn new(v: &str) -> Self {
        let mut expressMap = HashMap::new();
        for item in &string_util::findConvertString(v) {
            expressMap.insert(item.clone(), "#{".to_owned() + item.as_str() + "}");
        }
        let mut noConvertExpressMap = HashMap::new();
        for item in &string_util::findNoConvertString(v) {
            noConvertExpressMap.insert(item.clone(), "${".to_owned() + item.as_str() + "}");
        }
        Self {
            value: v.to_string(),
            expressMap:expressMap,
            noConvertExpressMap:noConvertExpressMap,
        }
    }
}

impl SqlNode for StringNode {
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        let mut result = self.value.clone();
        for (item, value) in &self.expressMap {
            let getV = env.get(item);
            if getV.is_none() {
                let v = holder.engine.Eval(item, env).unwrap();
                let vstr = holder.sqlConvert.convert(v);
                result = result.replace(value, vstr.as_str());
            } else {
                let v = getV.unwrap().clone();
                let vstr = holder.sqlConvert.convert(v);
                result = result.replace(value, vstr.as_str());
            }
        }
        for (item, value) in &self.noConvertExpressMap {
            result = result.replace(value, env.get(item).unwrap_or(&Value::String(String::new())).as_str().unwrap_or(""));
        }
        return Result::Ok(result);
    }

    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep);
        result=result+self.value.as_str();
        return result;
    }
}