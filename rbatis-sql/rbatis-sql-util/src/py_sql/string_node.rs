use crate::py_sql::Name;

#[derive(Clone, Debug)]
pub struct StringNode {
    pub value: String,
}

impl Name for String {
    fn name() -> &'static str {
        "string"
    }
}
