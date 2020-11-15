use std::collections::HashMap;

use serde_json::{json, Map, Value};

use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;

use crate::ast::ast::RbatisAST;
use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::ast::node::string_node::StringNode;
use crate::engine::runtime::RbatisEngine;
use crate::utils;

#[derive(Clone, Debug)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
    // pub open: String,
    // pub close: String,
    // pub separator: String,
}

impl RbatisAST for ForEachNode {
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        let mut result = String::new();
        let collection_value = utils::value_util::get_deep_value(self.collection.as_str(), env);
        if collection_value.is_null() {
            return Result::Err(crate::core::Error::from("[rbatis] collection name:".to_owned() + self.collection.as_str() + " is none value!"));
        }
        if collection_value.is_array(){
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
        }else if collection_value.is_object(){
            let collection = collection_value.as_object().unwrap();
            let collection_len = collection.len() as i32;
            let mut index = -1;
            for (key,item) in collection {
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
        }else{
            return Result::Err(crate::core::Error::from("[rbatis] collection name:".to_owned() + self.collection.as_str() + " is not a array or object/map value!"));
        }
    }
}

impl SqlNodePrint for ForEachNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<foreach";
        result = result + " collection=\"" + self.collection.as_str() + "\"";
        result = result + " index=\"" + self.index.as_str() + "\"";
        result = result + " item=\"" + self.item.as_str() + "\"";
        result = result + " >";

        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</foreach>";
        return result;
    }
}

#[test]
pub fn test_for_each_node() {
    let mut engine = RbatisEngine::new();
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
    let mut engine = RbatisEngine::new();
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