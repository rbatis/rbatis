use crate::codegen::syntax_tree::{Name, NodeType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhereNode {
    pub childs: Vec<NodeType>,
}

impl Name for WhereNode {
    fn name() -> &'static str {
        "where"
    }
}
