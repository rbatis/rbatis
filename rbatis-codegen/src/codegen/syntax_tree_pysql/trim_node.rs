use crate::codegen::syntax_tree_pysql::{Name, NodeType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrimNode {
    pub childs: Vec<NodeType>,
    pub start: String,
    pub end: String,
}

impl Name for TrimNode {
    fn name() -> &'static str {
        "trim"
    }
}
