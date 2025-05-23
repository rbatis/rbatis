use crate::codegen::syntax_tree_pysql::{AsHtml, Name};

/// Represents a `break` node in py_sql.
/// It's used to exit a loop, typically within a `foreach` block.
///
/// # Example
///
/// PySQL syntax:
/// ```py
/// for item in collection:
///   if item == 'something':
///     break
/// ```
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
