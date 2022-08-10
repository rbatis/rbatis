use crate::codegen::py_sql::{Name, NodeType};

#[derive(Clone, Debug)]
pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl Name for WhenNode {
    fn name() -> &'static str {
        "when"
    }
}
