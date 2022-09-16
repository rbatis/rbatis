use crate::codegen::syntax_tree::{DefaultName, Name, NodeType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OtherwiseNode {
    pub childs: Vec<NodeType>,
}

impl Name for OtherwiseNode {
    fn name() -> &'static str {
        "otherwise"
    }
}

impl DefaultName for OtherwiseNode {
    fn default_name() -> &'static str {
        "_"
    }
}
