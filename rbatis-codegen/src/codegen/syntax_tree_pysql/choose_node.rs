use crate::codegen::syntax_tree_pysql::{Name, NodeType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChooseNode {
    pub when_nodes: Vec<NodeType>,
    pub otherwise_node: Option<Box<NodeType>>,
}

impl Name for ChooseNode {
    fn name() -> &'static str {
        "choose"
    }
}
