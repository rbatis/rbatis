use super::{HtmlAstNode, NodeContext};
use crate::codegen::diagnostic::Diagnostic;
use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};
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

    fn try_from_element(element: &Element, span: Span) -> Result<Self, Diagnostic> {
        let test = match element.attrs.get("test") {
            Some(test) => test.clone(),
            None => {
                return Err(Diagnostic::new(
                    span,
                    "[rbatis-codegen] <if> is missing the required \"test\" attribute",
                    "add test=\"...\", for example <if test=\"name != ''\">",
                ))
            }
        };
        if test.trim().is_empty() {
            return Err(Diagnostic::new(
                span,
                "[rbatis-codegen] <if> has an empty \"test\" attribute",
                "test must not be empty",
            ));
        }
        Ok(Self {
            test,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        })
    }

    fn try_generate_tokens<FChildParser>(
        &self,
        context: &mut NodeContext<FChildParser>,
        ignore: &mut Vec<String>,
    ) -> Result<TokenStream, Diagnostic>
    where
        FChildParser:
            FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> Result<TokenStream, Diagnostic>,
    {
        let if_tag_body = if !self.childs.is_empty() {
            context.parse_children(&self.childs, ignore)?
        } else {
            quote! {}
        };

        // an empty func_name means "generate an expression body" instead of a function,
        // `context` is not used by func::impl_fn
        let method_impl = crate::codegen::func::try_impl_fn_with_span(
            "",
            "",
            &format!("\"{}\"", self.test),
            false,
            ignore,
            context.span,
        )
        .map_err(|e| Diagnostic::from_error(e, "check the test expression"))?;

        Ok(quote! {
            if #method_impl.to_owned().into() {
                #if_tag_body
            }
        })
    }
}
