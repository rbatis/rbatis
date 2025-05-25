use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::loader_html::Element;
use crate::codegen::syntax_tree_html::{HtmlAstNode, NodeContext,};

#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}
impl HtmlAstNode for ReturnNode {
    fn node_tag_name() -> &'static str { "return" }

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

        let child_body = context.parse_children(&self.childs, ignore);
   
        quote! {
            #child_body
            return (sql, args);
        }
    }
} 