use std::collections::HashMap;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents a <select> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct SelectTagNode {
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for SelectTagNode {
    fn node_tag_name() -> &'static str where Self: Sized { "select" }

    fn from_element(element: &Element) -> Self {
        Self {
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        // Replicates logic from `handle_crud_element` for <select>
        let method_name = Ident::new(context.fn_name, Span::call_site());
        let child_body = context.parse_children(&self.childs, ignore);
        
        // TODO: Accurately determine capacity and push_count as in original.
        // let capacity = element.child_string_cup() + 1000;
        // let push_count = child_body.to_string().matches("args.push").count();
        let capacity = 1024usize; // Placeholder with explicit i32 type
        let push_count = 10usize;   // Placeholder with explicit i32 type

        let function_token = quote! {
            pub fn #method_name(mut arg: rbs::Value, _tag: char) -> (String, Vec<rbs::Value>) {
                use rbatis_codegen::ops::*;
                let mut sql = String::with_capacity(#capacity);
                let mut args = Vec::with_capacity(#push_count);
                #child_body
                (sql, args)
            }
        };
        
        // Accumulate this function into methods if it's not already there.
        // The original code accumulated methods in the outer scope.
        // Here, we add it to context.methods. This needs careful handling to avoid duplicates
        // if multiple CRUD tags are present or if this is called multiple times.
        // For now, let's assume the caller of this whole parsing process handles method accumulation.
        // This generate_tokens should return the main body call, and the function itself 
        // should be collected by the `methods` accumulator in the context.
        
        // The original `handle_crud_element` would do: *body = quote! { #body #function };
        // This means the function itself is the token stream for this node.
        // The context.methods is where these should be stored by the main parser loop.
        context.methods.extend(function_token);

        // A CRUD element itself (like <select>) doesn't add to the current SQL string directly in the place it's defined.
        // It defines a function that will be called later. 
        // So, it should return an empty TokenStream or a marker if needed by the caller.
        // The original `parse_elements` for a CRUD tag just assigned the function to `*body` and then `*body` becomes the function.
        // This seems to imply the function *is* the result of parsing that tag.
        quote! { /* <select> defines a method, no direct SQL output here */ }
    }
} 