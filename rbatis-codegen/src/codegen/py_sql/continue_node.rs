use crate::codegen::py_sql::{AsHtml, Name};

#[derive(Clone,Debug)]
pub struct ContinueNode{

}

impl AsHtml for ContinueNode {
    fn as_html(&self) -> String {
        format!("<continue />")
    }
}

impl Name for ContinueNode {
    fn name() -> &'static str {
        "continue"
    }
}