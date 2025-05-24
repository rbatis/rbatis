use std::collections::HashMap;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents an <insert> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct InsertTagNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for InsertTagNode {
    fn node_tag_name() -> &'static str where Self: Sized { "insert" }

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
        // Replicates logic from `handle_crud_element` for <insert>
        let method_name = Ident::new(context.fn_name, Span::call_site());
        let child_body = context.parse_children(&self.childs, ignore);
        
        let capacity = 1024usize; // Placeholder
        let push_count = 10usize;   // Placeholder

        let function_token = quote! {
            pub fn #method_name(mut arg: rbs::Value, _tag: char) -> (String, Vec<rbs::Value>) {
                use rbatis_codegen::ops::*;
                let mut sql = String::with_capacity(#capacity);
                let mut args = Vec::with_capacity(#push_count);
                #child_body
                (sql, args)
            }
        };
        
        context.methods.extend(function_token);
        quote! { /* <insert> defines a method, no direct SQL output here */ }
    }
} 