use crate::codegen::syntax_tree::{Name, NodeType};

#[derive(Clone, Debug)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl Name for SetNode {
    fn name() -> &'static str {
        "set"
    }
}
