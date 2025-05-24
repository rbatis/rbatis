use crate::codegen::syntax_tree_pysql::{Name, NodeType, ToHtml};

/// Represents an `if` conditional node in py_sql.
/// It executes the nested SQL block if the `test` condition evaluates to true.
///
/// # Attributes
///
/// - `test`: The boolean expression to evaluate.
///
/// # Example
///
/// PySQL syntax:
/// ```py
/// if name != null:
///   AND name = #{name}
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl Name for IfNode {
    fn name() -> &'static str {
        "if"
    }
}


impl ToHtml for IfNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!("<if test=\"{}\">{}</if>", self.test, childs)
    }
}