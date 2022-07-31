use crate::py_sql::{NodeType, Name};

#[derive(Clone, Debug)]
pub struct WhereNode {
    pub childs: Vec<NodeType>,
}

impl Name for WhereNode{
    fn name() -> &'static str {
        "where"
    }
}