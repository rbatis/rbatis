use crate::py_sql::{NodeType, Name};

#[derive(Clone, Debug)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl Name for SetNode{
    fn name() -> &'static str {
        "set"
    }
}