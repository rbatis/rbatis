use crate::codegen::syntax_tree_pysql::{Name, NodeType, ToHtml};

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


impl ToHtml for ForEachNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!(
            "<foreach collection=\"{}\" index=\"{}\" item=\"{}\" >{}</foreach>",
            self.collection, self.index, self.item, childs
        )
    }
}
