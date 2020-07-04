use std::collections::HashMap;

use serde_json::{json, Map, Value};

use crate::ast::ast::RbatisAST;
use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::ast::node::string_node::StringNode;
use rbatis_core::convert::StmtConvert;
use crate::engine::runtime::RbatisEngine;
use crate::utils;
use rbatis_core::db::DriverType;

#[derive(Clone, Debug)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
    pub open: String,
    pub close: String,
    pub separator: String,
}

impl RbatisAST for ForEachNode {
    fn eval(&self, convert: &impl StmtConvert, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, rbatis_core::Error> {
        let mut result = String::new();

        //open
        result = result + self.open.as_str();

        let collection_value = utils::value_util::get_deep_value(self.collection.as_str(), env);
        if collection_value.is_null() {
            return Result::Err(rbatis_core::Error::from("[rbatis] collection name:".to_owned() + self.collection.as_str() + " is none value!"));
        }
        if !collection_value.is_array() {
            return Result::Err(rbatis_core::Error::from("[rbatis] collection name:".to_owned() + self.collection.as_str() + " is not a array value!"));
        }
        let collection = collection_value.as_array().unwrap();

        let collection_len = collection.len() as i32;
        let mut index = -1;
        let have_separator = !self.separator.is_empty();
        for item in collection {
            index = index + 1;
            //build temp arg
            let mut obj_map = serde_json::Map::new();
            obj_map.insert("item".to_string(), item.clone());
            obj_map.insert("index".to_string(), Value::Number(serde_json::Number::from_f64(index as f64).unwrap()));
            let mut temp_arg: Value = Value::Object(obj_map);
            let item_result = do_child_nodes(convert, &self.childs, &mut temp_arg, engine, arg_array)?;
            result = result + item_result.as_str();
            if have_separator && (index + 1) < collection_len {
                result = result + self.separator.as_str();
            }
        }
        //close
        result = result + self.close.as_str();
        return Result::Ok(result);
    }
}

impl SqlNodePrint for ForEachNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<foreach";
        result = result + " collection=\"" + self.collection.as_str() + "\"";
        result = result + " index=\"" + self.index.as_str() + "\"";
        result = result + " item=\"" + self.item.as_str() + "\"";
        result = result + " open=\"" + self.open.as_str() + "\"";
        result = result + " close=\"" + self.close.as_str() + "\"";
        result = result + " separator=\"" + self.separator.as_str() + "\"";
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
        open: "(".to_string(),
        close: ")".to_string(),
        separator: ",".to_string(),
    };
    let mut john = json!({
        "arg": 1,
    });
    let mut arg_array = vec![];
    let r = n.eval(&DriverType::Mysql, &mut john, &mut engine, &mut arg_array);
    println!("{}", r.unwrap_or("null".to_string()));
}