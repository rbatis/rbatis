use std::rc::Rc;
use crate::ast::Node::Node;
use serde_json::Value;
use crate::ast::StringNode::StringNode;

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
            _ => String::new(),
        }
    }
}

pub struct BindNode {
    pub name: String,
    pub value: String,
}

pub struct ChooseNode {
    pub whenNodes: Vec<NodeType>,
    pub otherwiseNode: Rc<NodeType>,
}

pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub  index: String,
    pub item: String,
    pub open: String,
    pub close: String,
    pub separator: String,
}

pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

pub struct IncludeNode {
    pub childs: Vec<NodeType>,
}

pub struct OtherwiseNode {
    pub childs: Vec<NodeType>,
}

pub struct TrimNode {
    pub childs: Vec<NodeType>,
    pub prefix: String,
    pub suffix: String,
    pub suffixOverrides: String,
    pub prefixOverrides: String,
}

pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}



