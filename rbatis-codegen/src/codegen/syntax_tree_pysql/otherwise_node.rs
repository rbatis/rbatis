use crate::codegen::syntax_tree_pysql::{DefaultName, Name, NodeType, ToHtml};

/// Represents an `otherwise` node in py_sql.
/// It's used within a `choose` block to provide a default SQL block to execute if none of the `when` conditions are met.
/// It can also be represented by `_`.
///
/// # Example
///
/// PySQL syntax (inside a `choose` block):
/// ```py
/// choose:
///   when test="type == 'A'":
///     sql_block_A
///   otherwise:  // or _:
///     sql_block_default
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OtherwiseNode {
    pub childs: Vec<NodeType>,
}

impl Name for OtherwiseNode {
    fn name() -> &'static str {
        "otherwise"
    }
}

impl DefaultName for OtherwiseNode {
    fn default_name() -> &'static str {
        "_"
    }
}

impl ToHtml for OtherwiseNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!("<otherwise>{}</otherwise>", childs)
    }
}
