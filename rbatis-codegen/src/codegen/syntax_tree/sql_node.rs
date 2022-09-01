use crate::codegen::syntax_tree::{AsHtml, Name, NodeType};

/// the SqlNode
#[derive(Clone, Debug)]
pub struct SqlNode {
    pub childs: Vec<NodeType>,
}

impl Name for SqlNode {
    fn name() -> &'static str {
        "sql"
    }
}

impl AsHtml for SqlNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!("<sql>{}</sql>", childs)
    }
}
