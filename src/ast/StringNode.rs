use crate::utils::string_util;
use crate::ast::Node::SqlNode;
use serde_json::Value;
use std::collections::HashMap;
use crate::ast::SqlArgTypeConvert::SqlArgTypeConvert;
use std::rc::Rc;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::lib;
use crate::engines::ExpressionEngine::ExpressionEngine;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use crate::ast::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

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

    pub holder: NodeConfigHolder,
}

impl StringNode {

    pub fn new(v: &str, holder:NodeConfigHolder) -> Self {
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
            holder:holder,
        }
    }
}

impl SqlNode for StringNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        let mut result = self.value.clone();
        for (item, value) in &self.expressMap {
            let getV = env.get(item);
            if getV.is_none() {
                let v = self.holder.engine.LexerAndEval(item, env).unwrap();
                let vstr = self.holder.sqlConvert.convert(v);
                result = result.replace(value, vstr.as_str());
            } else {
                let v = getV.unwrap().clone();
                let vstr = self.holder.sqlConvert.convert(v);
                result = result.replace(value, vstr.as_str());
            }
        }
        for (item, value) in &self.noConvertExpressMap {
            result = result.replace(value, env.get(item).unwrap_or(&Value::String(String::new())).as_str().unwrap_or(""));
        }
        return Result::Ok(result);
    }

    fn print(&self) -> String {
        let mut result="<string> ".to_string();
        result=result+self.value.as_str();
        return result;
    }
}