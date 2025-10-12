use crate::codegen::syntax_tree_pysql::{Name, NodeType, ToHtml};

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
    pub collection: String,
    pub skips: String,
    pub skip_null: bool,
}

impl Name for SetNode {
    fn name() -> &'static str {
        "set"
    }
}


impl ToHtml for SetNode {
    fn as_html(&self) -> String {
        let mut childs_html = String::new();
        for x in &self.childs {
            childs_html.push_str(&x.as_html());
        }

        let mut attrs_string = String::new();
        if !self.collection.is_empty() {
            attrs_string.push_str(&format!(" collection=\"{}\"", self.collection));
        }
        if !self.skips.is_empty() {
            attrs_string.push_str(&format!(" skips=\"{}\"", self.skips));
        }
        if self.skip_null {
            attrs_string.push_str(" skip_null=\"true\"");
        }else{
            attrs_string.push_str(" skip_null=\"false\"");
        }
        format!("<set{}>{}</set>", attrs_string, childs_html)
    }
}