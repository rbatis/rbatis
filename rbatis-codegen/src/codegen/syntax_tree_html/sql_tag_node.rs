use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents a <sql> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct SqlTagNode {
    /// Extracted from the "id" attribute.
    pub id: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for SqlTagNode {
    fn node_tag_name() -> &'static str where Self: Sized { "sql" }

    fn from_element(element: &Element) -> Self {
        let id = element.attrs.get("id")
            .expect("[rbatis-codegen] <sql> element must have id!")
            .clone();
        Self {
            id,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        // The <sql> tag itself doesn't directly generate code into the main SQL string in parse_elements.
        // It's used as a definition, and its children are processed when it's included or if it's a root element.
        // The original parse_elements handles <sql> by just recursing on its children.
        // So, if generate_tokens is called on a SqlTagNode directly, it means its children should be parsed.
        let child_tokens = context.parse_children(&self.childs, ignore);
        quote! { #child_tokens }
    }
} 