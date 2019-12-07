use std::rc::Rc;
use crate::ast::node::SqlNode;
use serde_json::Value;
use crate::ast::string_node::StringNode;
use crate::ast::if_node::IfNode;
use crate::ast::trim_node::TrimNode;
use crate::ast::foreach_node::ForEachNode;
use crate::ast::choose_node::ChooseNode;
use crate::ast::otherwise_node::OtherwiseNode;
use crate::ast::when_node::WhenNode;
use crate::ast::bind_node::BindNode;
use crate::ast::include_node::IncludeNode;
use crate::ast::set_node::SetNode;

use crate::ast::select_node::SelectNode;
use crate::ast::delete_node::DeleteNode;
use crate::ast::update_node::UpdateNode;
use crate::ast::insert_node::InsertNode;
use crate::ast::insert_templete_node::InsertTempleteNode;
use crate::ast::update_templete_node::UpdateTempleteNode;
use crate::ast::delete_templete_node::DeleteTempleteNode;
use crate::ast::select_templete_node::SelectTempleteNode;
use crate::ast::node_config_holder::NodeConfigHolder;
use crate::ast::where_node::WhereNode;

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

    NInsertTempleteNode(InsertTempleteNode),
    NUpdateTempleteNode(UpdateTempleteNode),
    NDeleteTempleteNode(DeleteTempleteNode),
    NSelectTempleteNode(SelectTempleteNode),
}

impl <'a>SqlNode for NodeType {
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        match self {
            NodeType::NSelectNode(node) => return node.eval(env,holder),
            NodeType::NDeleteNode(node) => return node.eval(env,holder),
            NodeType::NUpdateNode(node) => return node.eval(env,holder),
            NodeType::NInsertNode(node) => return node.eval(env,holder),

            NodeType::NSelectTempleteNode(node) => return node.eval(env,holder),
            NodeType::NDeleteTempleteNode(node) => return node.eval(env,holder),
            NodeType::NUpdateTempleteNode(node) => return node.eval(env,holder),
            NodeType::NInsertTempleteNode(node) => return node.eval(env,holder),

            NodeType::Null => return Result::Ok(String::new()),
            NodeType::NString(stringNode) => return stringNode.eval(env,holder),
            NodeType::NIf(ifNode) => return ifNode.eval(env,holder),
            NodeType::NTrim(trimNode) => return trimNode.eval(env,holder),
            NodeType::NForEach(forEachNode) => return forEachNode.eval(env,holder),
            NodeType::NChoose(chooseNode) => return chooseNode.eval(env,holder),
            NodeType::NOtherwise(otherwiseNode) => return otherwiseNode.eval(env,holder),
            NodeType::NWhen(whenNode) => return whenNode.eval(env,holder),
            NodeType::NBind(bindNode) => return bindNode.eval(env,holder),
            NodeType::NInclude(includeNode) => return includeNode.eval(env,holder),
            NodeType::NSet(setNode) => return setNode.eval(env,holder),
            NodeType::NWhere(node) => return node.eval(env,holder),
            _ => Result::Err(String::from("eval NodeType not exist!")),
        }
    }

    fn print(&self,deep:i32) -> String {
        match self {
            NodeType::NSelectNode(node) => return node.print(deep),
            NodeType::NUpdateNode(node) => return node.print(deep),
            NodeType::NInsertNode(node) => return node.print(deep),
            NodeType::NDeleteNode(node) => return node.print(deep),

            NodeType::NSelectTempleteNode(node) => return node.print(deep),
            NodeType::NUpdateTempleteNode(node) => return node.print(deep),
            NodeType::NInsertTempleteNode(node) => return node.print(deep),
            NodeType::NDeleteTempleteNode(node) => return node.print(deep),


            NodeType::Null => return "null".to_string(),
            NodeType::NString(stringNode) => return stringNode.print(deep),
            NodeType::NIf(ifNode) => return ifNode.print(deep),
            NodeType::NTrim(trimNode) => return trimNode.print(deep),
            NodeType::NForEach(forEachNode) => return forEachNode.print(deep),
            NodeType::NChoose(chooseNode) => return chooseNode.print(deep),
            NodeType::NOtherwise(otherwiseNode) => return otherwiseNode.print(deep),
            NodeType::NWhen(whenNode) => return whenNode.print(deep),
            NodeType::NBind(bindNode) => return bindNode.print(deep),
            NodeType::NInclude(includeNode) => return includeNode.print(deep),
            NodeType::NSet(setNode) => return setNode.print(deep),
            NodeType::NWhere(node) => return node.print(deep),
            _ => String::from("print NodeType not exist!"),
        }
    }
}



