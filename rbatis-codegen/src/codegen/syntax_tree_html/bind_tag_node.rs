use super::{HtmlAstNode, NodeContext};
use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::LitStr;

/// Represents a `<bind>` tag node in the HTML AST.
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
    fn node_tag_name() -> &'static str {
        "bind"
    }

    fn from_element(element: &Element) -> Self {
        let name = element
            .attrs
            .get("name")
            .expect("[rbatis-codegen] <bind> element must have name!")
            .clone();
        let value = element
            .attrs
            .get("value")
            .expect("[rbatis-codegen] <bind> element must have value!")
            .clone();
        Self {
            name,
            value,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(
        &self,
        _context: &mut NodeContext<FChildParser>,
        ignore: &mut Vec<String>,
    ) -> TokenStream
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

        // `<bind>` has no children that could carry SQL, so child_parser is not used.
        //
        // the assignment writes the value into the argument map under `name` (a missing
        // key is created by `IndexMut<&str> for ValueMap`), which is what makes it
        // reachable for `#{name}`/`${name}` of the following tags
        quote! {
            arg[#lit_str_name] = rbs::value(#method_impl).unwrap_or_default();
        }
    }
}
