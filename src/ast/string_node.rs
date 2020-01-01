use crate::utils::string_util;
use crate::ast::node::{SqlNode, create_deep, SqlNodePrint};
use serde_json::Value;
use std::collections::HashMap;
use crate::ast::convert::sql_arg_type_convert::SqlArgTypeConvert;
use std::rc::Rc;
use crate::engine;

use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use crate::ast::config_holder::ConfigHolder;

/**
*  string抽象节点
**/
#[derive(Clone)]
pub struct StringNode {
    pub value: String,
    //去重的，需要替换的要sql转换express map
    pub express_map: HashMap<String, String>,
    //去重的，需要替换的免sql转换express map
    pub no_convert_express_map: HashMap<String, String>,
}

impl StringNode {

    pub fn new(v: &str) -> Self {
        let mut express_map = HashMap::new();
        for item in &string_util::find_convert_string(v) {
            express_map.insert(item.clone(), "#{".to_owned() + item.as_str() + "}");
        }
        let mut no_convert_express_map = HashMap::new();
        for item in &string_util::find_no_convert_string(v) {
            no_convert_express_map.insert(item.clone(), "${".to_owned() + item.as_str() + "}");
        }
        Self {
            value: v.to_string(),
            express_map: express_map,
            no_convert_express_map: no_convert_express_map,
        }
    }
}

impl SqlNode for StringNode {
    fn eval(&self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
        let mut result = self.value.clone();
        for (item, value) in &self.express_map {
            let get_v = env.get(item);
            if get_v.is_none() {
                let v = holder.engine.eval(item, env).unwrap();
                let vstr = holder.sql_convert.convert(v);
                result = result.replace(value, vstr.as_str());
            } else {
                let v = get_v.unwrap().clone();
                let vstr = holder.sql_convert.convert(v);
                result = result.replace(value, vstr.as_str());
            }
        }
        for (item, value) in &self.no_convert_express_map {
            result = result.replace(value, env.get(item).unwrap_or(&Value::String(String::new())).as_str().unwrap_or(""));
        }
        return Result::Ok(result);
    }
}

impl SqlNodePrint for StringNode{
    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep);
        result=result+self.value.as_str();
        return result;
    }
}