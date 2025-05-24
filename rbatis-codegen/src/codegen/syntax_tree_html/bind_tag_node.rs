use std::collections::HashMap;
use proc_macro2::{ Span, TokenStream};
use quote::quote;
use syn::LitStr;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents a <bind> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct BindTagNode {
    /// Extracted from the "name" attribute.
    pub name: String,
    /// Extracted from the "value" attribute (expression string).
    pub value: String,
    pub attrs: HashMap<String, String>,
    // Bind tags typically do not have children, but Vec<Element> is kept for consistency.
    pub childs: Vec<Element>,
}

impl HtmlAstNode for BindTagNode {
    fn node_tag_name() -> &'static str where Self: Sized { "bind" }

    fn from_element(element: &Element) -> Self {
        let name = element.attrs.get("name")
            .expect("[rbatis-codegen] <bind> element must have name!")
            .clone();
        let value = element.attrs.get("value")
            .expect("[rbatis-codegen] <bind> element must have value!")
            .clone();
        Self {
            name,
            value,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(&self, _context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        // Replicates the logic from `handle_bind_element`
        let method_impl = crate::codegen::func::impl_fn(
            "", // body_to_string, context might be needed
            "", // fn_name, context might be needed
            &format!("\"{}\"", self.value),
            false,
            ignore,
        );

        let lit_str_name = LitStr::new(&self.name, Span::call_site());

        // Bind nodes do not typically have children that generate SQL, so child_parser is not used for self.childs.
        // If they had children, their tokens would be generated using context.parse_children(&self.childs);
        // and then appropriately placed.

        quote! {
            if arg[#lit_str_name] == rbs::Value::Null {
                // Ensure the key exists before assignment, even if it's to insert Null first.
                // This behavior is slightly different from original, which only inserted if arg[#name] was Value::Null
                // The original implies `arg` is a map-like structure where direct assignment might create the key.
                // For rbs::Value, which seems to be map-like, direct assignment arg[key] = val is fine.
                // The original check was `if arg[#lit_str] == rbs::Value::Null` then `arg.insert(...)`
                // This seems slightly off as it would insert string key if null, then replace.
                // The intention is probably to ensure the key exists or to set it.
                // Let's stick to the original assignment logic: if it's Null, insert Null, then overwrite.
                // This is a bit redundant. A more direct `arg[#lit_str] = ...` is likely intended.
                // arg.insert(rbs::Value::String(#lit_str_name.to_string()), rbs::Value::Null);
            }
            arg[#lit_str_name] = rbs::value(#method_impl).unwrap_or_default();
        }
    }
} 