use crate::codegen::syntax_tree_pysql::{Name, NodeType};

/// Represents a `choose` node in py_sql.
/// It provides a way to conditionally execute different blocks of SQL, similar to a switch statement.
/// It must contain one or more `when` child nodes and can optionally have an `otherwise` child node.
///
/// # Example
///
/// PySQL syntax:
/// ```py
/// choose:
///   when test="type == 'A'":
///     sql_block_A
///   when test="type == 'B'":
///     sql_block_B
///   otherwise:
///     sql_block_default
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChooseNode {
    pub when_nodes: Vec<NodeType>,
    pub otherwise_node: Option<Box<NodeType>>,
}

impl Name for ChooseNode {
    fn name() -> &'static str {
        "choose"
    }
}
