use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents a <when> tag node (child of <choose>) in the HTML AST.
#[derive(Debug, Clone)]
pub struct WhenTagNode {
    /// Extracted from the "test" attribute.
    pub test: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for WhenTagNode {
    fn node_tag_name() -> &'static str where Self: Sized { "when" }

    fn from_element(element: &Element) -> Self {
        let test = element.attrs.get("test")
            .unwrap_or_else(|| panic!("[rbatis-codegen] <when> element must have test field! Found: {:?}", element.attrs))
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
        // This logic is primarily used within the context of a <choose> tag.
        // The <choose> tag will call this and integrate it into its conditional structure.
        // Replicates parts of `impl_condition` used in `handle_choose_element` for a `when` tag.

        let condition_body = if !self.childs.is_empty() {
            context.parse_children(&self.childs, ignore)
        } else {
            quote! {}
        };

        let method_impl = crate::codegen::func::impl_fn(
            "", // body_to_string - review context needs
            "", // fn_name - review context needs
            &format!("\"{}\"", self.test),
            false,
            ignore,
        );
        
        // For <when> inside <choose>, if the condition is true, its body is evaluated,
        // and then the <choose> block should terminate for that path.
        // The original `handle_choose_element` used `appends: quote! { return sql; }` for `impl_condition`.
        quote! {
            if #method_impl.to_owned().into() {
                #condition_body
                return sql; // Exit the <choose> block's closure
            }
        }
    }
} 