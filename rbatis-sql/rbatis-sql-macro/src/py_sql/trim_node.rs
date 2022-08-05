use crate::py_sql::{Name, NodeType};

#[derive(Clone, Debug)]
pub struct TrimNode {
    pub childs: Vec<NodeType>,
    pub trim: String,
}

impl Name for TrimNode {
    fn name() -> &'static str {
        "trim"
    }
}
