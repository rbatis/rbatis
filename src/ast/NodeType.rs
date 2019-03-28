use std::rc::Rc;
use crate::ast::Node::Node;
use serde_json::Value;
use crate::ast::StringNode::StringNode;
use crate::ast::IfNode::IfNode;
use crate::ast::TrimNode::TrimNode;
use crate::ast::ForEachNode::ForEachNode;
use crate::ast::ChooseNode::ChooseNode;
use crate::ast::OtherwiseNode::OtherwiseNode;
use crate::ast::WhenNode::WhenNode;
use crate::ast::BindNode::BindNode;
use crate::ast::IncludeNode::IncludeNode;

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
}

impl Node for NodeType {
    fn eval(&self, env: &Value) -> String {
        match self {
            NodeType::Null => return String::new(),
            NodeType::NString(stringNode) => return stringNode.eval(env),
            NodeType::NIf(ifNode) => return ifNode.eval(env),
            NodeType::NTrim(trimNode) => return trimNode.eval(env),
            NodeType::NForEach(forEachNode) => return forEachNode.eval(env),
            NodeType::NChoose(chooseNode) => return chooseNode.eval(env),
            NodeType::NOtherwise(otherwiseNode) => return otherwiseNode.eval(env),
            NodeType::NWhen(whenNode) => return whenNode.eval(env),
            NodeType::NBind(bindNode) => return bindNode.eval(env),
            NodeType::NInclude(includeNode) => return includeNode.eval(env),
            _ => String::new(),
        }
    }
}



