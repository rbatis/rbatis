use super::{HtmlAstNode, NodeContext};
use crate::codegen::loader_html::Element;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

/// Represents an `<update>` tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct UpdateTagNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for UpdateTagNode {
    fn node_tag_name() -> &'static str {
        "update"
    }

    fn try_from_element(element: &Element, _span: Span) -> syn::Result<Self> {
        Ok(Self {
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
        // Replicates logic from `handle_crud_element` for <update>
        let method_name = Ident::new(context.fn_name, Span::call_site());
        let child_body = context.parse_children(&self.childs, ignore)?;

        // the estimates of the previous implementation: the text of the whole statement
        // plus the `args.push(..)` calls the generated body contains
        let capacity = self
            .childs
            .iter()
            .map(|c| c.child_string_cup())
            .sum::<usize>()
            + 1000;
        let push_count = child_body.to_string().matches("push").count();

        let function_token = quote! {
            pub fn #method_name(mut arg: rbs::Value, _tag: char) -> (String, Vec<rbs::Value>) {
                use rbatis_codegen::ops::*;
                let mut sql = String::with_capacity(#capacity);
                let mut args = Vec::with_capacity(#push_count);
                #child_body
                (sql, args)
            }
        };

        context.methods.extend(function_token);
        Ok(quote! { /* <update> defines a method, no direct SQL output here */ })
    }
}
