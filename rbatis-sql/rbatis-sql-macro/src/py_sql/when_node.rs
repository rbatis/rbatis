use crate::py_sql::{NodeType, Name};

#[derive(Clone, Debug)]
pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl Name for WhenNode{
    fn name() -> &'static str {
        "when"
    }
}