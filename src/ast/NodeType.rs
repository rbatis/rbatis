use std::rc::Rc;
use crate::ast::Node::Node;
use serde_json::Value;

pub enum NodeType {
    Null,
    NString(StringNode),
    NIf(If),
    NTrim(Trim),
    NForEach(ForEach),
    NChoose(Choose),
    NOtherwise(Otherwise),
    NWhen(When),
    NBind(Bind),
    NInclude(Include),
}

impl Node for NodeType{
    fn eval(&self, env: Value) -> String {
        match self {
            NodeType::NString(strNode)=> return strNode.value.clone() ,
            _=> String::new(),
        }
    }
}

pub struct Bind {
    pub name: String,
    pub value: String,
}

pub struct Choose {
    pub whenNodes: Vec<NodeType>,
    pub otherwiseNode: Rc<NodeType>,
}

pub struct ForEach {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub  index: String,
    pub item: String,
    pub open: String,
    pub close: String,
    pub separator: String,
}

pub struct If {
    pub childs: Vec<NodeType>,
    pub test: String,
}

pub struct Include {
    pub childs: Vec<NodeType>,
}

pub struct Otherwise {
    pub childs: Vec<NodeType>,
}

pub struct StringNode {
    pub value: String,
    //去重的，需要替换的express map
    pub expressMap: Vec<String>,
    //去重的，需要替换的express map
    pub noConvertExpressMap: Vec<String>,
}

pub struct Trim {
    pub childs: Vec<NodeType>,
    pub prefix: String,
    pub suffix: String,
    pub suffixOverrides: String,
    pub prefixOverrides: String,
}

pub struct When {
    pub childs: Vec<NodeType>,
    pub test: String,
}



