use crate::codegen::syntax_tree_pysql::{Name, NodeType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl Name for WhenNode {
    fn name() -> &'static str {
        "when"
    }
}
