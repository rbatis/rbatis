use crate::codegen::syntax_tree::{Name, NodeType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl Name for IfNode {
    fn name() -> &'static str {
        "if"
    }
}
