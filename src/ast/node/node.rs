use std::collections::HashMap;


use serde_json::{json, Value};


use crate::ast::node::bind_node::BindNode;
use crate::ast::node::choose_node::ChooseNode;
use crate::ast::node::delete_node::DeleteNode;
use crate::ast::node::foreach_node::ForEachNode;
use crate::ast::node::if_node::IfNode;
use crate::ast::node::include_node::IncludeNode;
use crate::ast::node::insert_node::InsertNode;
use crate::ast::node::node_type::NodeType::NWhen;
use crate::ast::node::otherwise_node::OtherwiseNode;
use crate::ast::node::result_map_id_node::ResultMapIdNode;
use crate::ast::node::result_map_node::ResultMapNode;
use crate::ast::node::result_map_result_node::ResultMapResultNode;
use crate::ast::node::select_node::SelectNode;
use crate::ast::node::set_node::SetNode;
use crate::ast::node::string_node::StringNode;
use crate::ast::node::trim_node::TrimNode;
use crate::ast::node::update_node::UpdateNode;
use crate::ast::node::when_node::WhenNode;
use crate::ast::node::where_node::WhereNode;
use crate::utils::xml_loader::Element;

use super::node_type::NodeType;
use crate::ast::ast::Ast;
use crate::engine::runtime::RbatisEngine;

pub trait SqlNodePrint {
    fn print(&self, deep: i32) -> String;
}


//执行子所有节点
pub fn do_child_nodes(child_nodes: &Vec<NodeType>, env: &mut Value,engine: &mut RbatisEngine,arg_array:&mut Vec<Value>) -> Result<String, String> {
    let mut s = String::new();
    for item in child_nodes {
        let item_result = item.eval(env,engine,arg_array);
        if item_result.is_err() {
            return item_result;
        }
        s = s + item_result.unwrap().as_str();
    }
    return Result::Ok(s);
}


pub fn print_child(arg: &Vec<impl SqlNodePrint>, deep: i32) -> String {
    let mut result = String::new();
    for x in arg {
        let item = x.print(deep);
        result = result + "" + item.as_str();
    }
    return result;
}

pub fn create_deep(deep: i32) -> String {
    let mut s = "\n".to_string();
    for index in 0..deep {
        s = s + "  ";
    }
    return s;
}


#[test]
fn test_string_node() {
    let mut engine = RbatisEngine::new();
    let mut john = json!({
        "name": "John Doe",
    });
    let str_node = NodeType::NString(StringNode::new("select * from ${name} where name = #{name}"));
    let mut arg_array=vec![];

    let result = str_node.eval(&mut john,&mut engine, &mut arg_array).unwrap();
    println!("{}", result);
}