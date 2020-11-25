use serde_json::map::Map;

use serde_json::{json, Value};

use crate::ast::ast::RbatisAST;
use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::engine;
use crate::engine::runtime::RbatisEngine;
use crate::utils::string_util;


///string抽象节点
#[derive(Clone, Debug)]
pub struct StringNode {
    pub value: String,
    //去重的，需要替换的要sql转换express map
    pub express_map: Map<String, Value>,
    //去重的，需要替换的免sql转换express map
    pub express_map_no_convert: Map<String, Value>,
}

impl StringNode {
    pub fn new(v: &str) -> Self {
        let mut express_map = Map::new();
        for item in &string_util::find_convert_string(v) {
            express_map.insert(item.clone(), Value::String("#{".to_owned() + item.as_str() + "}"));
        }
        let mut express_map_no_convert = Map::new();
        for item in &string_util::find_no_convert_string(v) {
            express_map_no_convert.insert(item.clone(), Value::String("${".to_owned() + item.as_str() + "}"));
        }
        Self {
            value: v.to_string(),
            express_map,
            express_map_no_convert,
        }
    }
}

impl RbatisAST for StringNode {
    fn name() -> &'static str {
        "string"
    }
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        let mut result = self.value.clone();
        for (item, value) in &self.express_map {
            let value = value.as_str().unwrap_or("");
            result = result.replace(value, convert.stmt_convert(arg_array.len()).as_str());
            let get_v = env.get(item);
            if get_v.is_none() {
                let v = engine.eval(item, env).unwrap();
                arg_array.push(v);
            } else {
                let v = get_v.unwrap().clone();
                arg_array.push(v);
            }
        }
        for (item, value) in &self.express_map_no_convert {
            let value = value.as_str().unwrap_or("");
            let v = env.get(item);
            match v {
                Some(v) => {
                    if v.is_string() {
                        result = result.replace(value, &v.as_str().unwrap());
                    } else {
                        result = result.replace(value, &v.to_string());
                    }
                }
                _ => {
                    result = result.replace(value, "");
                }
            }
        }
        return Result::Ok(result);
    }
}


#[test]
pub fn test_string_node() {
    let mut john = json!({
        "arg": 2,
    });
    let mut engine = RbatisEngine::new();
    let s_node = StringNode::new("arg+1=#{arg+1}");
    let mut arg_array = vec![];

    let r = s_node.eval(&DriverType::Mysql, &mut john, &mut engine, &mut arg_array).unwrap();
    println!("{}", r);
}