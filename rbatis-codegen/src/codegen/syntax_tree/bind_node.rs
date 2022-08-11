use crate::codegen::syntax_tree::{DefaultName, Name};

#[derive(Clone, Debug)]
pub struct BindNode {
    pub name: String,
    pub value: String,
}

impl DefaultName for BindNode {
    fn default_name() -> &'static str {
        "let"
    }
}

impl Name for BindNode {
    fn name() -> &'static str {
        "bind"
    }
}
