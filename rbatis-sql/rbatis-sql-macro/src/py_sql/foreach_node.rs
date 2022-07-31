use crate::py_sql::{NodeType, Name};

#[derive(Clone, Debug)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
}

impl Name for ForEachNode{
    fn name() -> &'static str {
        "for"
    }
}