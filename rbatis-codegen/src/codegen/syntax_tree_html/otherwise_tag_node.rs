use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents an <otherwise> tag node (child of <choose>) in the HTML AST.
#[derive(Debug, Clone)]
pub struct OtherwiseTagNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for OtherwiseTagNode {
    fn node_tag_name() -> &'static str where Self: Sized { "otherwise" }

    fn from_element(element: &Element) -> Self {
        // No specific attributes to extract for <otherwise> itself beyond common ones.
        Self {
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        // This logic is primarily used within the context of a <choose> tag.
        // The <choose> tag will call this for its <otherwise> branch.
        // Replicates `impl_otherwise` and its usage in `handle_choose_element`.
        let child_body = context.parse_children(&self.childs, ignore);
        quote! {
            #child_body
            // Unlike <when>, <otherwise> doesn't `return sql;` here because it's the last part of the choose block.
            // The `return sql;` is implicit at the end of the choose block closure.
        }
    }
} 