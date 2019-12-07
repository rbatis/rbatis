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
use crate::ast::config_holder::ConfigHolder;
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
}

impl <'a>SqlNode for NodeType {
    fn eval(&mut self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
        match self {
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

    fn print(&self,deep:i32) -> String {
        match self {
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



