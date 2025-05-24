use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext, TrimTagNode}; // Import TrimTagNode for reuse

/// Represents a <where> tag node in the HTML AST.
/// This is a specialized form of the <trim> tag.
#[derive(Debug, Clone)]
pub struct WhereTagNode {
    // Where inherits behavior from trim but with specific prefixes/suffixes.
    // We can either embed a TrimTagNode or replicate its fields if they are simple.
    // For now, let's keep it simple, assuming its children are parsed and then specific where cleanup is applied.
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for WhereTagNode {
    fn node_tag_name() -> &'static str { "where" }

    fn from_element(element: &Element) -> Self {
        // No specific attributes to extract for <where> itself beyond common ones.
        Self {
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
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