use crate::py_sql::{NodeType, Name};

#[derive(Clone, Debug)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl Name for IfNode{
    fn name() -> &'static str {
        "if"
    }
}