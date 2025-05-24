use std::collections::HashMap;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents a <foreach> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct ForeachTagNode {
    pub collection: String, // Expression string
    pub item: String,       // Variable name for item
    pub index: String,      // Variable name for index
    pub open: String,
    pub close: String,
    pub separator: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for ForeachTagNode {
    fn node_tag_name() -> &'static str where Self: Sized { "foreach" }

    fn from_element(element: &Element) -> Self {
        let empty = String::new();
        let def_item = "item".to_string();
        let def_index = "index".to_string();

        let collection = element.attrs.get("collection").cloned().unwrap_or_else(|| {
            panic!("[rbatis-codegen] <foreach> element must have a 'collection' attribute.")
        });
        
        let mut item = element.attrs.get("item").cloned().unwrap_or(def_item.clone());
        let mut index = element.attrs.get("index").cloned().unwrap_or(def_index.clone());
        let open = element.attrs.get("open").cloned().unwrap_or_else(|| empty.clone());
        let close = element.attrs.get("close").cloned().unwrap_or_else(|| empty.clone());
        let separator = element.attrs.get("separator").cloned().unwrap_or_else(|| empty.clone());

        if item.is_empty() || item == "_" {
            item = def_item;
        }
        if index.is_empty() || index == "_" {
            index = def_index;
        }

        Self {
            collection,
            item,
            index,
            open,
            close,
            separator,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        // Create a new ignore list for the children of this foreach loop,
        // including the item and index variables.
        let mut item_specific_ignores = ignore.clone();
        item_specific_ignores.push(self.index.clone());
        item_specific_ignores.push(self.item.clone());

        // Parse children using the new, extended ignore list.
        let foreach_body = context.parse_children(&self.childs, &mut item_specific_ignores);
        
        // The collection expression itself should be parsed using the original ignore list.
        let collection_method_impl = crate::codegen::func::impl_fn(
            "", // body_to_string context placeholder
            "", // fn_name context placeholder
            &format!("\"{}\"", self.collection),
            false,
            ignore, // Use original ignore list for the collection expression
        );

        let open_str = &self.open;
        let open_impl = if !self.open.is_empty() {
            quote! { sql.push_str(#open_str); }
        } else {
            quote! {}
        };

        let close_str = &self.close;
        let close_impl = if !self.close.is_empty() {
            quote! { sql.push_str(#close_str); }
        } else {
            quote! {}
        };

        let item_ident = Ident::new(&self.item, Span::call_site());
        let index_ident = Ident::new(&self.index, Span::call_site());

        let separator_str = &self.separator;

        let (split_code, split_code_trim) = if !separator_str.is_empty() {
            (
                quote! { sql.push_str(#separator_str); },
                quote! { sql = sql.trim_end_matches(#separator_str).to_string(); }
            )
        } else {
            (quote! {}, quote! {})
        };
        
        let loop_tokens = if !self.separator.is_empty() {
            quote! {
                for (ref #index_ident, #item_ident) in #collection_method_impl {
                    #foreach_body
                    #split_code
                }
                #split_code_trim
            }
        } else {
            quote! {
                for (ref #index_ident, #item_ident) in #collection_method_impl {
                    #foreach_body
                }
            }
        };

        quote! {
            #open_impl
            #loop_tokens
            #close_impl
        }
    }
} 