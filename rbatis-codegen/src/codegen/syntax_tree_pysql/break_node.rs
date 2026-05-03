use crate::codegen::syntax_tree_pysql::{Name, ToHtml};

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

impl ToHtml for BreakNode {
    fn as_html(&self) -> String {
        "<break></break>".to_string()
    }
}

impl Name for BreakNode {
    fn name() -> &'static str {
        "break"
    }
}
