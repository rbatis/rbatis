use crate::py_sql::{NodeType, Name};

#[derive(Clone, Debug)]
pub struct ChooseNode {
    pub when_nodes: Vec<NodeType>,
    pub otherwise_node: Option<Box<NodeType>>,
}

impl Name for ChooseNode{
    fn name() -> &'static str {
        "choose"
    }
}