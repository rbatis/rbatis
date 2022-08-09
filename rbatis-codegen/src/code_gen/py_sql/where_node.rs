use crate::code_gen::py_sql::{Name, NodeType};

#[derive(Clone, Debug)]
pub struct WhereNode {
    pub childs: Vec<NodeType>,
}

impl Name for WhereNode {
    fn name() -> &'static str {
        "where"
    }
}
