use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents a <mapper> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct MapperTagNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for MapperTagNode {
    fn node_tag_name() -> &'static str { "mapper" }

    fn from_element(element: &Element) -> Self {
        Self {
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        // The <mapper> tag, similar to <sql>, is a container.
        // The original `parse_elements` function, when encountering a <mapper> tag,
        // immediately recurses on its children without adding any specific tokens for the <mapper> itself.
        let child_tokens = context.parse_children(&self.childs, ignore);
        quote! { #child_tokens }
    }
} 