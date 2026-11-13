use super::{HtmlAstNode, NodeContext};
use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

/// Represents a `<continue>` tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct ContinueTagNode {
    pub attrs: HashMap<String, String>,
    // Continue tags do not have children that generate SQL content.
    pub childs: Vec<Element>,
}

impl HtmlAstNode for ContinueTagNode {
    fn node_tag_name() -> &'static str {
        "continue"
    }

    fn try_from_element(element: &Element, _span: Span) -> syn::Result<Self> {
        Ok(Self {
            attrs: element.attrs.clone(),
            childs: element.childs.clone(), // Should be empty
        })
    }

    fn try_generate_tokens<FChildParser>(
        &self,
        _context: &mut NodeContext<FChildParser>,
        _ignore: &mut Vec<String>,
    ) -> syn::Result<TokenStream>
    where
        FChildParser:
            FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> syn::Result<TokenStream>,
    {
        // Replicates `impl_continue`
        Ok(quote! { continue; })
    }
}
