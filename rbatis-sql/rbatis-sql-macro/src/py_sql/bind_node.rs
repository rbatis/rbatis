use crate::py_sql::{DefName, Name};

#[derive(Clone, Debug)]
pub struct BindNode {
    pub name: String,
    pub value: String,
}

impl DefName for BindNode {
    fn def_name() -> &'static str {
        "let"
    }
}

impl Name for BindNode {
    fn name() -> &'static str {
        "bind"
    }
}
