use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext, WhenTagNode, OtherwiseTagNode};

/// Represents a <choose> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct ChooseTagNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for ChooseTagNode {
    fn node_tag_name() -> &'static str where Self: Sized { "choose" }

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
        let mut inner_body = quote! {};

        for child_element in &self.childs {
            match child_element.tag.as_str() {
                tag_name if tag_name == WhenTagNode::node_tag_name() => {
                    let when_node = WhenTagNode::from_element(child_element);
                    let when_tokens = when_node.generate_tokens(context, ignore);
                    inner_body = quote! { #inner_body #when_tokens };
                }
                tag_name if tag_name == OtherwiseTagNode::node_tag_name() => {
                    let otherwise_node = OtherwiseTagNode::from_element(child_element);
                    let otherwise_tokens = otherwise_node.generate_tokens(context, ignore);
                    inner_body = quote! { #inner_body #otherwise_tokens };
                }
                _ => panic!("[rbatis-codegen] <choose> node's children must be <when> or <otherwise> nodes! Found: {}", child_element.tag),
            }
        }

        // TODO: Replace with a proper capacity estimation. 
        // The original code used element.child_string_cup(), which needs to be available
        // or re-implemented here or in a shared utility.
        let capacity = 1024usize; // Placeholder capacity

        quote! {
            sql.push_str(&|| -> String {
                let mut sql = String::with_capacity(#capacity);
                #inner_body
                return sql; // This sql is local to the closure
            }());
        }
    }
} 