use crate::codegen::string_util::escape_attr_value;
use crate::codegen::syntax_tree_pysql::{DefaultName, Name, ToHtml};

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

impl ToHtml for BindNode {
    fn as_html(&self) -> String {
        format!(
            "<bind name=\"{}\" value=\"{}\"></bind>",
            escape_attr_value(&self.name),
            escape_attr_value(&self.value)
        )
    }
}
