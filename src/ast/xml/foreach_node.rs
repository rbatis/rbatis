use std::collections::HashMap;

use serde_json::{json, Map, Value};

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::xml::node_type::NodeType;
use crate::ast::xml::string_node::StringNode;
use crate::utils;

#[derive(Clone)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
    pub open: String,
    pub close: String,
    pub separator: String,
}

impl Ast for ForEachNode {
    fn eval(&self, env: &mut Value, arg_array:&mut Vec<Value>,holder: &mut ConfigHolder) -> Result<String, String> {
        let mut result = String::new();

        //open
        result = result + self.open.as_str();

        let collection_value = utils::value_util::get_deep_value(self.collection.as_str(), env);
        if collection_value.is_null() {
            return Result::Err("[rbatis] collection name:".to_owned() + self.collection.as_str() + " is none value!");
        }
        if !collection_value.is_array() {
            return Result::Err("[rbatis] collection name:".to_owned() + self.collection.as_str() + " is not a array value!");
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
            let item_result = do_child_nodes(&self.childs, &mut temp_arg, arg_array,holder);
            if item_result.is_err() {
                return item_result;
            }
            result = result + item_result.unwrap().as_str();
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
    let mut holder = ConfigHolder::new();
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
    let mut arg_array=vec![];
    let r = n.eval(&mut john, &mut arg_array,&mut holder);
    println!("{}", r.unwrap_or("null".to_string()));
}