use crate::ast::node_type::NodeType;
use crate::ast::node::{SqlNode, do_child_nodes, print_child, create_deep};
use serde_json::{Value, Map};
use crate::utils;
use std::collections::HashMap;
use crate::ast::config_holder::ConfigHolder;

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

impl SqlNode for ForEachNode {
    fn eval(&self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
        let mut result = String::new();

        //open
        result=result+self.open.as_str();

        let collection_value = utils::value_util::get_deep_value(self.collection.as_str(), env);
        if collection_value.is_null() {
            return Result::Err("[rbatis] collection name:".to_owned() + self.collection.as_str() + " is none value!");
        }
        if !collection_value.is_array() {
            return Result::Err("[rbatis] collection name:".to_owned() + self.collection.as_str() + " is not a array value!");
        }
        let collection = collection_value.as_array().unwrap();

        let collection_len =collection.len() as i32;
        let mut index = -1;
        let have_separator = !self.separator.is_empty();
        for item in collection {
            index = index + 1;
            //build temp arg
            let mut obj_map = serde_json::Map::new();
            obj_map.insert("item".to_string(), item.clone());
            obj_map.insert("index".to_string(), Value::Number(serde_json::Number::from_f64(index as f64).unwrap()));
            let mut temp_arg: Value = Value::Object(obj_map);
            let item_result = do_child_nodes(&self.childs, &mut temp_arg, holder);
            if item_result.is_err() {
                return item_result;
            }
            result = result + item_result.unwrap().as_str();
            if have_separator && (index+1)< collection_len {
                result = result + self.separator.as_str();
            }
        }
        //close
        result=result+self.close.as_str();
        return Result::Ok(result);
    }

    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<foreach";
        result=result+" collection=\""+self.collection.as_str()+"\"";
        result=result+" index=\""+self.index.as_str()+"\"";
        result=result+" item=\""+self.item.as_str()+"\"";
        result=result+" open=\""+self.open.as_str()+"\"";
        result=result+" close=\""+self.close.as_str()+"\"";
        result=result+" separator=\""+self.separator.as_str()+"\"";
        result=result+" >";

        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result=result+create_deep(deep).as_str()+"</foreach>";
        return result;
    }
}