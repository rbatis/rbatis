use crate::codegen::syntax_tree_pysql::Name;

/// Represents a plain string, a SQL text segment, or a string with preserved whitespace in py_sql.
/// This node holds parts of the SQL query that are not dynamic tags or raw text.
///
/// # Examples
///
/// PySQL syntax for simple text segments:
/// ```py
/// SELECT * FROM users WHERE
/// if name != null:
///   name = #{name}
/// // In the above, "SELECT * FROM users WHERE " is a StringNode.
/// ```
///
/// PySQL syntax for strings with preserved whitespace (using backticks - single line only):
/// ```py
/// ` SELECT   column1,    column2   FROM my_table  `
/// ```
///
/// It also handles simple quoted strings if they are part of the py_sql structure:
/// ```py
/// // Example within a more complex structure (e.g., an expression):
/// // if status == 'active':
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringNode {
    pub value: String,
}

impl Name for String {
    fn name() -> &'static str {
        "string"
    }
}
