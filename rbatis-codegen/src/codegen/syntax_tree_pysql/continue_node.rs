use crate::codegen::syntax_tree_pysql::{AsHtml, Name};

/// Represents a `continue` node in py_sql.
/// It's used to skip the current iteration of a loop and proceed to the next one, typically within a `foreach` block.
///
/// # Example
///
/// PySQL syntax:
/// ```py
/// for item in collection:
///   if item == 'skip_this':
///     continue
///   # process item
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinueNode {}

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
