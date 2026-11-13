use super::{HtmlAstNode, NodeContext};
use crate::codegen::diagnostic::Diagnostic;
use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

/// Represents a `<when>` tag node (child of `<choose>`) in the HTML AST.
#[derive(Debug, Clone)]
pub struct WhenTagNode {
    /// Extracted from the "test" attribute.
    pub test: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for WhenTagNode {
    fn node_tag_name() -> &'static str {
        "when"
    }

    fn try_from_element(element: &Element, span: Span) -> Result<Self, Diagnostic> {
        let test = match element.attrs.get("test") {
            Some(test) => test.clone(),
            None => {
                return Err(Diagnostic::new(
                    span,
                    "[rbatis-codegen] <when> is missing the required \"test\" attribute",
                    "add test=\"...\", for example <when test=\"name != ''\">",
                ))
            }
        };
        if test.trim().is_empty() {
            return Err(Diagnostic::new(
                span,
                "[rbatis-codegen] <when> has an empty \"test\" attribute",
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
        // This logic is primarily used within the context of a <choose> tag.
        // The <choose> tag will call this and integrate it into its conditional structure.
        // Replicates parts of `impl_condition` used in `handle_choose_element` for a `when` tag.

        let condition_body = if !self.childs.is_empty() {
            context.parse_children(&self.childs, ignore)?
        } else {
            quote! {}
        };

        let method_impl = crate::codegen::func::try_impl_fn_with_span(
            "", // body_to_string - review context needs
            "", // fn_name - review context needs
            &format!("\"{}\"", self.test),
            false,
            ignore,
            context.span,
        )
        .map_err(|e| Diagnostic::from_error(e, "check the test expression"))?;

        // For <when> inside <choose>, if the condition is true, its body is evaluated,
        // and then the <choose> block should terminate for that path.
        // The original `handle_choose_element` used `appends: quote! { return sql; }` for `impl_condition`.
        Ok(quote! {
            if #method_impl.to_owned().into() {
                #condition_body
                return sql; // Exit the <choose> block's closure
            }
        })
    }
}
