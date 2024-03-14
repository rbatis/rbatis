use crate::codegen::syntax_tree_pysql::{AsHtml, Name};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreakNode {}

impl AsHtml for BreakNode {
    fn as_html(&self) -> String {
        format!("<break/>")
    }
}

impl Name for BreakNode {
    fn name() -> &'static str {
        "break"
    }
}
