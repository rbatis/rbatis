use super::{HtmlAstNode, NodeContext};
use crate::codegen::diagnostic::Diagnostic;
use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

/// Represents a `<mapper>` tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct MapperTagNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for MapperTagNode {
    fn node_tag_name() -> &'static str {
        "mapper"
    }

    fn try_from_element(element: &Element, _span: Span) -> Result<Self, Diagnostic> {
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
        // The <mapper> tag, similar to <sql>, is a container.
        // The original `parse_elements` function, when encountering a <mapper> tag,
        // immediately recurses on its children without adding any specific tokens for the <mapper> itself.
        let child_tokens = context.parse_children(&self.childs, ignore)?;
        Ok(quote! { #child_tokens })
    }
}
