use super::{HtmlAstNode, NodeContext, OtherwiseTagNode, WhenTagNode};
use crate::codegen::diagnostic::Diagnostic;
use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

/// Represents a `<choose>` tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct ChooseTagNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for ChooseTagNode {
    fn node_tag_name() -> &'static str {
        "choose"
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
        let mut inner_body = quote! {};

        for child_element in &self.childs {
            match child_element.tag.as_str() {
                tag_name if tag_name == WhenTagNode::node_tag_name() => {
                    let when_node = WhenTagNode::try_from_element(child_element, context.span)
                        .map_err(|d| d.with_source(child_element.source.clone()))?;
                    let when_tokens = when_node
                        .try_generate_tokens(context, ignore)
                        .map_err(|d| d.with_source(child_element.source.clone()))?;
                    inner_body = quote! { #inner_body #when_tokens };
                }
                tag_name if tag_name == OtherwiseTagNode::node_tag_name() => {
                    let otherwise_node =
                        OtherwiseTagNode::try_from_element(child_element, context.span)
                            .map_err(|d| d.with_source(child_element.source.clone()))?;
                    let otherwise_tokens = otherwise_node
                        .try_generate_tokens(context, ignore)
                        .map_err(|d| d.with_source(child_element.source.clone()))?;
                    inner_body = quote! { #inner_body #otherwise_tokens };
                }
                _ => {
                    return Err(Diagnostic::new(
                        context.span,
                        format!(
                            "[rbatis-codegen] <choose> children must be <when> or <otherwise>, found <{}>",
                            child_element.tag
                        ),
                        "expected <when> or <otherwise>",
                    )
                    .with_source(child_element.source.clone()))
                }
            }
        }

        // the sql of this branch is built in its own scope, so the text of the
        // children is a good estimate for it
        let capacity = self
            .childs
            .iter()
            .map(|c| c.child_string_cup())
            .sum::<usize>()
            + 100;

        Ok(quote! {
            sql.push_str(&|| -> String {
                let mut sql = String::with_capacity(#capacity);
                #inner_body
                return sql; // This sql is local to the closure
            }());
        })
    }
}
