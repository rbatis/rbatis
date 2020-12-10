use std::collections::HashMap;

use serde_json::{json, Map, Value};

use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;
use crate::interpreter::sql::node::string_node::StringNode;
use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::interpreter::expr::runtime::ExprRuntime;
use crate::utils;

#[derive(Clone, Debug)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
}

impl ForEachNode {
    pub fn from(source: &str, express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        if !express.contains("in ") {
            return Err(crate::core::Error::from("[rbatis] parser express fail:".to_string() + source));
        }
        let express = express["for ".len()..].trim();
        let in_index = express.find("in ").unwrap();
        let col = express[in_index + "in ".len()..].trim();
        let mut item = express[..in_index].trim();
        let mut index = "";
        if item.contains(",") {
            let items: Vec<&str> = item.split(",").collect();
            if items.len() != 2 {
                return Err(crate::core::Error::from(format!("[rbatis][py] parse fail 'for ,' must be 'for arg1,arg2 in ...',value:'{}'", source)));
            }
            index = items[0];
            item = items[1];
        }
        return Ok(ForEachNode {
            childs: childs,
            collection: col.to_string(),
            index: index.to_string(),
            item: item.to_string(),
        });
    }
}

impl RbatisAST for ForEachNode {
    fn name() -> &'static str {
        "for"
    }
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &ExprRuntime, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        let mut result = String::new();
        let collection_value = utils::value_util::get_deep_value(self.collection.as_str(), env);
        if collection_value.is_null() {
            return Result::Err(crate::core::Error::from("[rbatis] collection name:".to_owned() + self.collection.as_str() + " is none value!"));
        }
        if collection_value.is_array() {
            let collection = collection_value.as_array().unwrap();
            let collection_len = collection.len() as i32;
            let mut index = -1;
            for item in collection {
                index = index + 1;
                //build temp arg
                let mut obj_map = serde_json::Map::new();
                obj_map.insert(self.item.to_string(), item.clone());
                obj_map.insert(self.index.to_string(), json!(index));
                let mut temp_arg: Value = Value::Object(obj_map);
                let item_result = do_child_nodes(convert, &self.childs, &mut temp_arg, engine, arg_array)?;
                result = result + item_result.as_str();
            }
            return Result::Ok(result);
        } else if collection_value.is_object() {
            let collection = collection_value.as_object().unwrap();
            let collection_len = collection.len() as i32;
            let mut index = -1;
            for (key, item) in collection {
                index = index + 1;
                //build temp arg
                let mut obj_map = serde_json::Map::new();
                obj_map.insert(self.item.to_string(), item.clone());
                obj_map.insert(self.index.to_string(), json!(key));
                let mut temp_arg: Value = Value::Object(obj_map);
                let item_result = do_child_nodes(convert, &self.childs, &mut temp_arg, engine, arg_array)?;
                result = result + item_result.as_str();
            }
            return Result::Ok(result);
        } else {
            return Result::Err(crate::core::Error::from("[rbatis] collection name:".to_owned() + self.collection.as_str() + " is not a array or object/map value!"));
        }
    }
}


#[test]
pub fn test_for_each_node() {
    let mut engine = ExprRuntime::new();
    let n = ForEachNode {
        childs: vec![NodeType::NString(StringNode::new("index:#{index},item:#{item}"))],
        collection: "arg".to_string(),
        index: "index".to_string(),
        item: "item".to_string(),
    };
    let mut john = json!({
        "arg": [1,2,3],
    });
    let mut arg_array = vec![];
    let r = n.eval(&DriverType::Mysql, &mut john, &mut engine, &mut arg_array);
    println!("{}", r.unwrap_or("null".to_string()));
    println!("{}", json!(arg_array));
}

#[test]
pub fn test_for_each_object_node() {
    let mut engine = ExprRuntime::new();
    let n = ForEachNode {
        childs: vec![NodeType::NString(StringNode::new("index:#{index},item:#{item}"))],
        collection: "arg".to_string(),
        index: "index".to_string(),
        item: "item".to_string(),
    };
    let mut john = json!({
        "arg": {
           "id":1
        },
    });
    let mut arg_array = vec![];
    let r = n.eval(&DriverType::Mysql, &mut john, &mut engine, &mut arg_array);
    println!("{}", r.unwrap_or("null".to_string()));
    println!("{}", json!(arg_array));
}