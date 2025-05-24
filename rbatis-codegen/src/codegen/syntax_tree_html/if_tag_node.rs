use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents an <if> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct IfTagNode {
    /// Extracted from the "test" attribute.
    pub test: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for IfTagNode {
    fn node_tag_name() -> &'static str where Self: Sized { "if" }

    fn from_element(element: &Element) -> Self {
        let test = element.attrs.get("test")
            .unwrap_or_else(|| panic!("[rbatis-codegen] <if> element must have test field! Found: {:?}", element.attrs))
            .clone();
        Self {
            test,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        let if_tag_body = if !self.childs.is_empty() {
            context.parse_children(&self.childs, ignore)
        } else {
            quote! {}
        };

        // Replicates the logic from `impl_condition` and `handle_if_element`
        let method_impl = crate::codegen::func::impl_fn(
            "", // Placeholder for body_to_string, review if context is needed
            "", // Placeholder for fn_name, review if context is needed
            &format!("\"{}\"", self.test),
            false,
            ignore, // Pass the ignore vector here
        );
        
        // The original `impl_condition` had `appends` which was empty for `if` but `return sql;` for `when`.
        // For a direct `if` node, appends is empty.
        quote! {
            if #method_impl.to_owned().into() {
                #if_tag_body
            }
        }
    }
}