use crate::codegen::syntax_tree_pysql::{Name, NodeType};

/// Represents a `set` node in py_sql.
/// It's typically used in `UPDATE` statements to dynamically include `SET` clauses.
/// It will automatically remove trailing commas if present.
///
/// # Example
///
/// PySQL syntax:
/// ```py
/// UPDATE table
/// set:
///   if name != null:
///     name = #{name},
///   if age != null:
///     age = #{age},
/// WHERE id = #{id}
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl Name for SetNode {
    fn name() -> &'static str {
        "set"
    }
}
