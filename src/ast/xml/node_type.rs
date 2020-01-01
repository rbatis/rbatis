use std::rc::Rc;
use crate::ast::xml::node::{SqlNode, SqlNodePrint};
use serde_json::{Value,json};
use crate::ast::xml::string_node::StringNode;
use crate::ast::xml::if_node::IfNode;
use crate::ast::xml::trim_node::TrimNode;
use crate::ast::xml::foreach_node::ForEachNode;
use crate::ast::xml::choose_node::ChooseNode;
use crate::ast::xml::otherwise_node::OtherwiseNode;
use crate::ast::xml::when_node::WhenNode;
use crate::ast::xml::bind_node::BindNode;
use crate::ast::xml::include_node::IncludeNode;
use crate::ast::xml::set_node::SetNode;

use crate::ast::xml::select_node::SelectNode;
use crate::ast::xml::delete_node::DeleteNode;
use crate::ast::xml::update_node::UpdateNode;
use crate::ast::xml::insert_node::InsertNode;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::where_node::WhereNode;
use crate::ast::xml::result_map_node::ResultMapNode;
use crate::ast::xml::result_map_id_node::ResultMapIdNode;
use crate::ast::xml::result_map_result_node::ResultMapResultNode;
use crate::engine::node::Node;

#[derive(Clone)]
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

impl NodeType{
    pub fn print_node(&self)-> String {
        return self.print(0);
    }

    pub fn to_result_map_node(&self)->Option<ResultMapNode>{
        match self {
            NodeType::NResultMapNode(node)=>{
                   return Option::Some(node.clone());
            }
            _=>{}
        }
        return Option::None;
    }
}

impl <'a>SqlNode for NodeType {
    fn eval(&self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
        match self {
            NodeType::NResultMapIdNode(node) => return node.eval(env,holder),
            NodeType::NResultMapResultNode(node) => return node.eval(env,holder),
            NodeType::NResultMapNode(node) => return node.eval(env,holder),

            NodeType::NSelectNode(node) => return node.eval(env,holder),
            NodeType::NDeleteNode(node) => return node.eval(env,holder),
            NodeType::NUpdateNode(node) => return node.eval(env,holder),
            NodeType::NInsertNode(node) => return node.eval(env,holder),

            NodeType::Null => return Result::Ok(String::new()),
            NodeType::NString(node) => return node.eval(env,holder),
            NodeType::NIf(node) => return node.eval(env,holder),
            NodeType::NTrim(node) => return node.eval(env,holder),
            NodeType::NForEach(node) => return node.eval(env,holder),
            NodeType::NChoose(node) => return node.eval(env,holder),
            NodeType::NOtherwise(node) => return node.eval(env,holder),
            NodeType::NWhen(node) => return node.eval(env,holder),
            NodeType::NBind(node) => return node.eval(env,holder),
            NodeType::NInclude(node) => return node.eval(env,holder),
            NodeType::NSet(node) => return node.eval(env,holder),
            NodeType::NWhere(node) => return node.eval(env,holder),
            _ => Result::Err(String::from("eval NodeType not exist!")),
        }
    }
}

impl SqlNodePrint for NodeType{
    fn print(&self,deep:i32) -> String {
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
            _ => String::from("print NodeType not exist!"),
        }
    }
}



