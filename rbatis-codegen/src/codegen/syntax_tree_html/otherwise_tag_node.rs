use super::{HtmlAstNode, NodeContext};
use crate::codegen::diagnostic::Diagnostic;
use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

/// Represents an `<otherwise>` tag node (child of `<choose>`) in the HTML AST.
#[derive(Debug, Clone)]
pub struct OtherwiseTagNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for OtherwiseTagNode {
    fn node_tag_name() -> &'static str {
        "otherwise"
    }

    fn try_from_element(element: &Element, _span: Span) -> Result<Self, Diagnostic> {
        // No specific attributes to extract for <otherwise> itself beyond common ones.
        Ok(Self {
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
        // The <choose> tag will call this for its <otherwise> branch.
        // Replicates `impl_otherwise` and its usage in `handle_choose_element`.
        let child_body = context.parse_children(&self.childs, ignore)?;
        Ok(quote! {
            #child_body
            // Unlike <when>, <otherwise> doesn't `return sql;` here because it's the last part of the choose block.
            // The `return sql;` is implicit at the end of the choose block closure.
        })
    }
}
