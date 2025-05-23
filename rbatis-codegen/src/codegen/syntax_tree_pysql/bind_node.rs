use crate::codegen::syntax_tree_pysql::{DefaultName, Name};

/// Represents a `bind` or `let` node in py_sql.
/// It's used to assign a value to a variable within the SQL query.
/// 
/// # Examples
/// 
/// PySQL syntax:
/// ```py
/// let name = 'value'
/// bind name = 'value'
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
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
