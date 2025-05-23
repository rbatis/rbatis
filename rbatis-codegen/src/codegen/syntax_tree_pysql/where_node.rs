use crate::codegen::syntax_tree_pysql::{Name, NodeType};

/// Represents a `where` node in py_sql.
/// It's used to dynamically build the `WHERE` clause of a SQL query.
/// It will automatically prepend `WHERE` if needed and remove leading `AND` or `OR` keywords from its content.
///
/// # Example
///
/// PySQL syntax:
/// ```py
/// SELECT * FROM table
/// where:
///   if id != null:
///     AND id = #{id}
///   if name != null:
///     AND name = #{name}
/// ```
/// This would result in `SELECT * FROM table WHERE id = #{id} AND name = #{name}` (if both conditions are true),
/// or `SELECT * FROM table WHERE id = #{id}` (if only id is not null),
/// or `SELECT * FROM table WHERE name = #{name}` (if only name is not null).
/// If no conditions are met, the `WHERE` clause is omitted entirely.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhereNode {
    pub childs: Vec<NodeType>,
}

impl Name for WhereNode {
    fn name() -> &'static str {
        "where"
    }
}
