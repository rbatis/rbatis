use crate::codegen::syntax_tree_pysql::{ToHtml, Name, NodeType};

/// Represents a reusable SQL fragment node in py_sql, defined by a `<sql>` tag in XML or an equivalent in py_sql.
/// It allows defining a piece of SQL that can be included elsewhere.
///
/// # Example
///
/// PySQL syntax (conceptual, as direct py_sql for `<sql>` might be less common than XML):
/// ```py
/// # define a reusable sql fragment
/// sql id='columns':
///   column1, column2
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SqlNode {
    pub id: String,
    pub childs: Vec<NodeType>,
}

impl Name for SqlNode {
    fn name() -> &'static str {
        "sql"
    }
}

impl ToHtml for SqlNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!("<sql id=\"{}\">{}</sql>", self.id, childs)
    }
}
