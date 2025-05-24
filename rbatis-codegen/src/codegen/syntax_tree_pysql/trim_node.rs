use crate::codegen::syntax_tree_pysql::{Name, NodeType, ToHtml};

/// Represents a `trim` node in py_sql.
/// It's used to remove leading and/or trailing characters from a string.
/// 
/// # Examples
/// 
/// PySQL syntax:
/// ```py
/// trim ',' 
/// trim start=',' 
/// trim end=',' 
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrimNode {
    pub childs: Vec<NodeType>,
    pub start: String,
    pub end: String,
}

impl Name for TrimNode {
    fn name() -> &'static str {
        "trim"
    }
}

impl ToHtml for TrimNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!(
            "<trim prefixOverrides=\"{}\" suffixOverrides=\"{}\">{}</trim>",
            self.start, self.end, childs
        )
    }
}
