use super::{HtmlAstNode, NodeContext};
use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{Error, LitStr};

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
    /// source line of the tag in the template, `0` when unknown
    pub line: usize,
    /// source text of that line, used in diagnostics
    pub line_text: String,
}

impl HtmlAstNode for BindTagNode {
    fn node_tag_name() -> &'static str {
        "bind"
    }

    fn try_from_element(element: &Element, span: Span) -> syn::Result<Self> {
        let name = match element.attrs.get("name") {
            Some(name) => name.clone(),
            None => {
                return Err(Error::new(
                    span,
                    format!(
                        "[rbatis-codegen] <bind> is missing the required \"name\" attribute{}",
                        element.code_frame(
                            "add name=\"...\", for example <bind name=\"x\" value=\"a + b\">"
                        )
                    ),
                ))
            }
        };
        let value = match element.attrs.get("value") {
            Some(value) => value.clone(),
            None => {
                return Err(Error::new(
                    span,
                    format!(
                        "[rbatis-codegen] <bind> is missing the required \"value\" attribute{}",
                        element.code_frame(
                            "add value=\"...\", for example <bind name=\"x\" value=\"a + b\">"
                        )
                    ),
                ))
            }
        };
        if name.trim().is_empty() {
            return Err(Error::new(
                span,
                format!(
                    "[rbatis-codegen] <bind> has an empty \"name\" attribute{}",
                    element.code_frame("name must not be empty")
                ),
            ));
        }
        if value.trim().is_empty() {
            return Err(Error::new(
                span,
                format!(
                    "[rbatis-codegen] <bind> has an empty \"value\" attribute{}",
                    element.code_frame("value must not be empty")
                ),
            ));
        }
        Ok(Self {
            name,
            value,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
            line: element.line,
            line_text: element.line_text.clone(),
        })
    }

    fn try_generate_tokens<FChildParser>(
        &self,
        context: &mut NodeContext<FChildParser>,
        ignore: &mut Vec<String>,
    ) -> syn::Result<TokenStream>
    where
        FChildParser:
            FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> syn::Result<TokenStream>,
    {
        // Replicates the logic from `handle_bind_element`
        let method_impl = crate::codegen::func::try_impl_fn_with_span(
            "", // body_to_string, context might be needed
            "", // fn_name, context might be needed
            &format!("\"{}\"", self.value),
            false,
            ignore,
            context.span,
        )
        .map_err(|e| {
            syn::Error::new(
                e.span(),
                format!(
                    "{}{}",
                    e,
                    crate::codegen::loader_html::code_frame(
                        self.line,
                        &self.line_text,
                        "check the value expression"
                    )
                ),
            )
        })?;

        let lit_str_name = LitStr::new(&self.name, Span::call_site());

        // `<bind>` has no children that could carry SQL, so child_parser is not used.
        //
        // the assignment writes the value into the argument map under `name` (a missing
        // key is created by `IndexMut<&str> for ValueMap`), which is what makes it
        // reachable for `#{name}`/`${name}` of the following tags
        Ok(quote! {
            arg[#lit_str_name] = rbs::value(#method_impl).unwrap_or_default();
        })
    }
}
