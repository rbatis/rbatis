use crate::code_gen::py_sql::{Name, NodeType};

#[derive(Clone, Debug)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
}

impl Name for ForEachNode {
    fn name() -> &'static str {
        "for"
    }
}
