use serde_json::{json, Value};

use crate::ast::ast::RbatisAST;
use crate::ast::node::bind_node::BindNode;
use crate::ast::node::choose_node::ChooseNode;
use crate::ast::node::foreach_node::ForEachNode;
use crate::ast::node::if_node::IfNode;
use crate::ast::node::node::SqlNodePrint;
use crate::ast::node::otherwise_node::OtherwiseNode;
use crate::ast::node::set_node::SetNode;
use crate::ast::node::string_node::StringNode;
use crate::ast::node::trim_node::TrimNode;
use crate::ast::node::when_node::WhenNode;
use crate::ast::node::where_node::WhereNode;
use crate::core::convert::StmtConvert;
use crate::engine::node::Node;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone, Debug)]
pub enum NodeType {
    Null,
    NString(StringNode),
    NIf(IfNode),
    NTrim(TrimNode),
    NForEach(ForEachNode),
    NChoose(ChooseNode),
    NOtherwise(OtherwiseNode),
    NWhen(WhenNode),
    NBind(BindNode),
    NSet(SetNode),
    NWhere(WhereNode)
}

impl NodeType {
    pub fn print_node(&self) -> String {
        return self.print(0);
    }

    pub fn childs(&self) -> Option<&Vec<NodeType>> {
        match self {

            NodeType::Null => return None,
            NodeType::NString(node) => return None,
            NodeType::NIf(node) => return Some(&node.childs),
            NodeType::NTrim(node) => return Some(&node.childs),
            NodeType::NForEach(node) => return Some(&node.childs),
            NodeType::NChoose(node) => return None,
            NodeType::NOtherwise(node) => return Some(&node.childs),
            NodeType::NWhen(node) => return Some(&node.childs),
            NodeType::NBind(node) => return None,
            NodeType::NSet(node) => return Some(&node.childs),
            NodeType::NWhere(node) => return Some(&node.childs),
        }
    }
    pub fn childs_mut(&mut self) -> Option<&mut Vec<NodeType>> {
        match self {

            NodeType::Null => return None,
            NodeType::NString(node) => return None,
            NodeType::NIf(node) => return Some(&mut node.childs),
            NodeType::NTrim(node) => return Some(&mut node.childs),
            NodeType::NForEach(node) => return Some(&mut node.childs),
            NodeType::NChoose(node) => return None,
            NodeType::NOtherwise(node) => return Some(&mut node.childs),
            NodeType::NWhen(node) => return Some(&mut node.childs),
            NodeType::NBind(node) => return None,
            NodeType::NSet(node) => return Some(&mut node.childs),
            NodeType::NWhere(node) => return Some(&mut node.childs),

        }
    }
}

impl<'a> RbatisAST for NodeType {
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        match self {
            NodeType::Null => return Result::Ok(String::new()),
            NodeType::NString(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NIf(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NTrim(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NForEach(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NChoose(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NOtherwise(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NWhen(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NBind(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NSet(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NWhere(node) => return node.eval(convert, env, engine, arg_array),
        }
    }
}

impl SqlNodePrint for NodeType {
    fn print(&self, deep: i32) -> String {
        match self {

            NodeType::Null => return "null".to_string(),
            NodeType::NString(node) => return node.print(deep),
            NodeType::NIf(node) => return node.print(deep),
            NodeType::NTrim(node) => return node.print(deep),
            NodeType::NForEach(node) => return node.print(deep),
            NodeType::NChoose(node) => return node.print(deep),
            NodeType::NOtherwise(node) => return node.print(deep),
            NodeType::NWhen(node) => return node.print(deep),
            NodeType::NBind(node) => return node.print(deep),
            NodeType::NSet(node) => return node.print(deep),
            NodeType::NWhere(node) => return node.print(deep),

        }
    }
}



