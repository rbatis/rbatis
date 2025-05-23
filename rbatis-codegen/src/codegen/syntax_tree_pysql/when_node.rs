use crate::codegen::syntax_tree_pysql::{Name, NodeType};

/// Represents a `when` node in py_sql.
/// It's used as a child of a `choose` node to define a conditional block of SQL.
/// The SQL block within `when` is executed if its `test` condition evaluates to true and no preceding `when` in the same `choose` was true.
///
/// # Attributes
///
/// - `test`: The boolean expression to evaluate.
///
/// # Example
///
/// PySQL syntax (inside a `choose` block):
/// ```py
/// choose:
///   when test="type == 'A'":
///     sql_for_type_A
///   when test="type == 'B'":
///     sql_for_type_B
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl Name for WhenNode {
    fn name() -> &'static str {
        "when"
    }
}
