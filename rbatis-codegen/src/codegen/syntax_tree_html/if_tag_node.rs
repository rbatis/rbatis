use super::{HtmlAstNode, NodeContext};
use crate::codegen::loader_html::Element;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

/// Represents an `<if>` tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct IfTagNode {
    /// Extracted from the "test" attribute.
    pub test: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for IfTagNode {
    fn node_tag_name() -> &'static str {
        "if"
    }

    fn from_element(element: &Element) -> Self {
        let test = element
            .attrs
            .get("test")
            .unwrap_or_else(|| {
                panic!(
                    "[rbatis-codegen] <if> element must have test field! Found: {:?}",
                    element.attrs
                )
            })
            .clone();
        Self {
            test,
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
        let if_tag_body = if !self.childs.is_empty() {
            context.parse_children(&self.childs, ignore)
        } else {
            quote! {}
        };

        // an empty func_name means "generate an expression body" instead of a function,
        // `context` is not used by func::impl_fn
        let method_impl =
            crate::codegen::func::impl_fn("", "", &format!("\"{}\"", self.test), false, ignore);

        quote! {
            if #method_impl.to_owned().into() {
                #if_tag_body
            }
        }
    }
}
