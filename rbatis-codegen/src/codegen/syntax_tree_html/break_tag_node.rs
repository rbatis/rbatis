use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents a <break> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct BreakTagNode {
    pub attrs: HashMap<String, String>,
    // Break tags do not have children that generate SQL content.
    pub childs: Vec<Element>,
}

impl HtmlAstNode for BreakTagNode {
    fn node_tag_name() -> &'static str { "break" }

    fn from_element(element: &Element) -> Self {
        Self {
            attrs: element.attrs.clone(),
            childs: element.childs.clone(), // Should be empty
        }
    }

    fn generate_tokens<FChildParser>(&self, _context: &mut NodeContext<FChildParser>, _ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        // Replicates `impl_break`
        quote! { break; }
    }
} 