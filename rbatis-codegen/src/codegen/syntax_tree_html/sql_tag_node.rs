use super::{HtmlAstNode, NodeContext};
use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::Error;

/// Represents a `<sql>` tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct SqlTagNode {
    /// Extracted from the "id" attribute.
    pub id: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for SqlTagNode {
    fn node_tag_name() -> &'static str {
        "sql"
    }

    fn try_from_element(element: &Element, span: Span) -> syn::Result<Self> {
        let id = match element.attrs.get("id") {
            Some(id) => id.clone(),
            None => {
                return Err(Error::new(
                    span,
                    format!(
                        "[rbatis-codegen] <sql> is missing the required \"id\" attribute{}",
                        element.code_frame("add id=\"...\"")
                    ),
                ))
            }
        };
        if id.trim().is_empty() {
            return Err(Error::new(
                span,
                format!(
                    "[rbatis-codegen] <sql> has an empty \"id\" attribute{}",
                    element.code_frame("id must not be empty")
                ),
            ));
        }
        Ok(Self {
            id,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
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
        // The <sql> tag itself doesn't directly generate code into the main SQL string in parse_elements.
        // It's used as a definition, and its children are processed when it's included or if it's a root element.
        // The original parse_elements handles <sql> by just recursing on its children.
        // So, if generate_tokens is called on a SqlTagNode directly, it means its children should be parsed.
        let child_tokens = context.parse_children(&self.childs, ignore)?;
        Ok(quote! { #child_tokens })
    }
}
