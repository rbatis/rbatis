use serde_json::{json, Value};

use crate::ast::ast::RbatisAST;
use crate::ast::node::bind_node::BindNode;
use crate::ast::node::choose_node::ChooseNode;
use crate::ast::node::delete_node::DeleteNode;
use crate::ast::node::foreach_node::ForEachNode;
use crate::ast::node::if_node::IfNode;
use crate::ast::node::include_node::IncludeNode;
use crate::ast::node::insert_node::InsertNode;
use crate::ast::node::node::SqlNodePrint;
use crate::ast::node::otherwise_node::OtherwiseNode;
use crate::ast::node::result_map_id_node::ResultMapIdNode;
use crate::ast::node::result_map_node::ResultMapNode;
use crate::ast::node::result_map_result_node::ResultMapResultNode;
use crate::ast::node::select_node::SelectNode;
use crate::ast::node::set_node::SetNode;
use crate::ast::node::sql_node::SqlNode;
use crate::ast::node::string_node::StringNode;
use crate::ast::node::trim_node::TrimNode;
use crate::ast::node::update_node::UpdateNode;
use crate::ast::node::when_node::WhenNode;
use crate::ast::node::where_node::WhereNode;
use crate::core::convert::StmtConvert;
use crate::engine::node::Node;
use crate::engine::runtime::RbatisEngine;
use crate::ast::node::custom_node::CustomNode;

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
    NInclude(IncludeNode),
    NSet(SetNode),
    NWhere(WhereNode),
    NCustomNode(CustomNode),

    NSqlNode(SqlNode),
    //CRUD
    NInsertNode(InsertNode),
    NUpdateNode(UpdateNode),
    NDeleteNode(DeleteNode),
    NSelectNode(SelectNode),
    //ResultMap
    NResultMapNode(ResultMapNode),
    NResultMapIdNode(ResultMapIdNode),
    NResultMapResultNode(ResultMapResultNode),
}

impl NodeType {
    pub fn print_node(&self) -> String {
        return self.print(0);
    }

    pub fn to_result_map_node(&self) -> Option<ResultMapNode> {
        match self {
            NodeType::NResultMapNode(node) => {
                return Option::Some(node.clone());
            }
            _ => {}
        }
        return Option::None;
    }

    pub fn childs(&self) -> Option<&Vec<NodeType>> {
        match self {
            NodeType::NResultMapIdNode(node) => return None,
            NodeType::NResultMapResultNode(node) => return None,
            NodeType::NResultMapNode(node) => return None,

            NodeType::NSelectNode(node) => return Some(&node.childs),
            NodeType::NDeleteNode(node) => return Some(&node.childs),
            NodeType::NUpdateNode(node) => return Some(&node.childs),
            NodeType::NInsertNode(node) => return Some(&node.childs),

            NodeType::Null => return None,
            NodeType::NString(node) => return None,
            NodeType::NIf(node) => return Some(&node.childs),
            NodeType::NTrim(node) => return Some(&node.childs),
            NodeType::NForEach(node) => return Some(&node.childs),
            NodeType::NChoose(node) => return None,
            NodeType::NOtherwise(node) => return Some(&node.childs),
            NodeType::NWhen(node) => return Some(&node.childs),
            NodeType::NBind(node) => return None,
            NodeType::NInclude(node) => return Some(&node.childs),
            NodeType::NSet(node) => return Some(&node.childs),
            NodeType::NWhere(node) => return Some(&node.childs),
            NodeType::NSqlNode(node) => return Some(&node.childs),
            NodeType::NCustomNode(node) => return Some(&node.childs),
        }
    }
    pub fn childs_mut(&mut self) -> Option<&mut Vec<NodeType>> {
        match self {
            NodeType::NResultMapIdNode(node) => return None,
            NodeType::NResultMapResultNode(node) => return None,
            NodeType::NResultMapNode(node) => return None,

            NodeType::NSelectNode(node) => return Some(&mut node.childs),
            NodeType::NDeleteNode(node) => return Some(&mut node.childs),
            NodeType::NUpdateNode(node) => return Some(&mut node.childs),
            NodeType::NInsertNode(node) => return Some(&mut node.childs),

            NodeType::Null => return None,
            NodeType::NString(node) => return None,
            NodeType::NIf(node) => return Some(&mut node.childs),
            NodeType::NTrim(node) => return Some(&mut node.childs),
            NodeType::NForEach(node) => return Some(&mut node.childs),
            NodeType::NChoose(node) => return None,
            NodeType::NOtherwise(node) => return Some(&mut node.childs),
            NodeType::NWhen(node) => return Some(&mut node.childs),
            NodeType::NBind(node) => return None,
            NodeType::NInclude(node) => return Some(&mut node.childs),
            NodeType::NSet(node) => return Some(&mut node.childs),
            NodeType::NWhere(node) => return Some(&mut node.childs),

            NodeType::NSqlNode(node) => return Some(&mut node.childs),
            NodeType::NCustomNode(node) => return Some(&mut node.childs),
        }
    }
}

impl<'a> RbatisAST for NodeType {
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        match self {
            NodeType::NResultMapIdNode(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NResultMapResultNode(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NResultMapNode(node) => return node.eval(convert, env, engine, arg_array),

            NodeType::NSelectNode(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NDeleteNode(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NUpdateNode(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NInsertNode(node) => return node.eval(convert, env, engine, arg_array),

            NodeType::Null => return Result::Ok(String::new()),
            NodeType::NString(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NIf(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NTrim(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NForEach(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NChoose(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NOtherwise(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NWhen(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NBind(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NInclude(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NSet(node) => return node.eval(convert, env, engine, arg_array),
            NodeType::NWhere(node) => return node.eval(convert, env, engine, arg_array),

            NodeType::NSqlNode(node) => return node.eval(convert, env, engine, arg_array),

            NodeType::NCustomNode(node) => return  node.eval(convert, env, engine, arg_array),
        }
    }
}

impl SqlNodePrint for NodeType {
    fn print(&self, deep: i32) -> String {
        match self {
            NodeType::NResultMapIdNode(node) => return node.print(deep),
            NodeType::NResultMapResultNode(node) => return node.print(deep),
            NodeType::NResultMapNode(node) => return node.print(deep),

            NodeType::NSelectNode(node) => return node.print(deep),
            NodeType::NUpdateNode(node) => return node.print(deep),
            NodeType::NInsertNode(node) => return node.print(deep),
            NodeType::NDeleteNode(node) => return node.print(deep),

            NodeType::Null => return "null".to_string(),
            NodeType::NString(node) => return node.print(deep),
            NodeType::NIf(node) => return node.print(deep),
            NodeType::NTrim(node) => return node.print(deep),
            NodeType::NForEach(node) => return node.print(deep),
            NodeType::NChoose(node) => return node.print(deep),
            NodeType::NOtherwise(node) => return node.print(deep),
            NodeType::NWhen(node) => return node.print(deep),
            NodeType::NBind(node) => return node.print(deep),
            NodeType::NInclude(node) => return node.print(deep),
            NodeType::NSet(node) => return node.print(deep),
            NodeType::NWhere(node) => return node.print(deep),

            NodeType::NSqlNode(node) => return node.print(deep),
            NodeType::NCustomNode(node) => return node.print(deep),
        }
    }
}



