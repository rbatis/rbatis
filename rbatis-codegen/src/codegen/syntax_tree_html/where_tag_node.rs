use super::{HtmlAstNode, NodeContext, TrimTagNode};
use crate::codegen::loader_html::Element;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

/// Represents a `<where>` tag node in the HTML AST.
///
/// `<where>` is a `<trim>` with fixed settings: it starts the fragment with `" where "`
/// and drops a leading `and`/`or` (and the trailing ones) so that a condition list that
/// begins with `and ...` becomes valid SQL.
#[derive(Debug, Clone)]
pub struct WhereTagNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for WhereTagNode {
    fn node_tag_name() -> &'static str {
        "where"
    }

    fn from_element(element: &Element) -> Self {
        // No specific attributes to extract for <where> itself beyond common ones.
        Self {
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(
        &self,
        context: &mut NodeContext<FChildParser>,
        ignore: &mut Vec<String>,
    ) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        // Create a TrimTagNode with where-specific configurations
        let trim_node = TrimTagNode {
            prefix: " where ".to_string(),
            suffix: "".to_string(),
            prefix_overrides: " |and |or ".to_string(),
            suffix_overrides: " | and| or".to_string(),
            attrs: self.attrs.clone(),
            childs: self.childs.clone(),
        };

        // Generate the base trimmed SQL
        let trimmed_sql = trim_node.generate_tokens(context, ignore);

        // Additional where-specific cleanup
        quote! {
            {
                #trimmed_sql
                sql = sql.trim_end_matches(" ").trim_end_matches(" where").to_string();
            }
        }
    }
}
