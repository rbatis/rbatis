use crate::py_sql::{NodeType, Name};

#[derive(Clone, Debug)]
pub struct PrintNode {
    pub value: String,
    pub format: String,
}

impl Name for PrintNode{
    fn name() -> &'static str {
        "println"
    }
}