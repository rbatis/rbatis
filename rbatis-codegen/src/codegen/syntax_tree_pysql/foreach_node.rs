use crate::codegen::syntax_tree_pysql::{Name, NodeType};

/// Represents a `for` loop node in py_sql.
/// It iterates over a collection and executes the nested SQL block for each item.
/// 
/// # Attributes
/// 
/// - `collection`: The expression providing the collection to iterate over.
/// - `item`: The name of the variable to hold the current item in each iteration.
/// - `index`: The name of the variable to hold the current key/index in each iteration.
/// 
/// # Example
/// 
/// PySQL syntax:
/// ```py
/// for item in ids:
///   AND id = #{item}
/// 
/// for key, item in user_map:
///   (#{key}, #{item.name})
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
}

impl Name for ForEachNode {
    fn name() -> &'static str {
        "for"
    }
}
