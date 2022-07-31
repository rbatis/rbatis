use crate::py_sql::{NodeType, Name, DefName};

#[derive(Clone, Debug)]
pub struct OtherwiseNode {
    pub childs: Vec<NodeType>,
}

impl Name for OtherwiseNode{
    fn name() -> &'static str {
        "otherwise"
    }
}

impl DefName for OtherwiseNode{
    fn def_name() -> &'static str {
        "_"
    }
}