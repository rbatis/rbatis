use std::rc::Rc;
use crate::ast::Node::Node;
use serde_json::Value;

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

impl Node for NodeType{
    fn eval(&self, env: Value) -> String {
        match self {
            NodeType::NString(strs)=> return strs.value.clone() ,
            _=> String::new(),
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

pub struct StringNode {
    pub value: String,
    //去重的，需要替换的express map
    pub expressMap: Vec<String>,
    //去重的，需要替换的express map
    pub noConvertExpressMap: Vec<String>,
}

impl StringNode{
    fn new(v:String)->Self{
        //TODO find v #[] and find v$[]
        Self{
            value: v,
            expressMap: vec![],
            noConvertExpressMap: vec![]
        }
    }
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



