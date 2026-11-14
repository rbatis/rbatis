use super::{HtmlAstNode, NodeContext};
use crate::codegen::diagnostic::Diagnostic;
use crate::codegen::loader_html::Element;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

/// Represents a `<foreach>` tag node in the HTML AST.
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
    fn node_tag_name() -> &'static str {
        "foreach"
    }

    fn try_from_element(element: &Element, span: Span) -> Result<Self, Diagnostic> {
        let empty = String::new();
        let def_item = "item".to_string();
        let def_index = "index".to_string();

        let collection =
            match element.attrs.get("collection") {
                Some(collection) => collection.clone(),
                None => return Err(Diagnostic::new(
                    span,
                    "[rbatis-codegen] <foreach> is missing the required \"collection\" attribute",
                    "add collection=\"...\", for example <foreach collection=\"ids\" item=\"id\">",
                )),
            };

        if collection.trim().is_empty() {
            return Err(Diagnostic::new(
                span,
                "[rbatis-codegen] <foreach> has an empty \"collection\" attribute",
                "collection must not be empty",
            ));
        }

        let mut item = element
            .attrs
            .get("item")
            .cloned()
            .unwrap_or(def_item.clone());
        let mut index = element
            .attrs
            .get("index")
            .cloned()
            .unwrap_or(def_index.clone());
        let open = element
            .attrs
            .get("open")
            .cloned()
            .unwrap_or_else(|| empty.clone());
        let close = element
            .attrs
            .get("close")
            .cloned()
            .unwrap_or_else(|| empty.clone());
        let separator = element
            .attrs
            .get("separator")
            .cloned()
            .unwrap_or_else(|| empty.clone());

        if item.is_empty() || item == "_" {
            item = def_item;
        }
        if index.is_empty() || index == "_" {
            index = def_index;
        }
        if syn::parse_str::<Ident>(&item).is_err() {
            return Err(Diagnostic::new(
                span,
                format!("[rbatis-codegen] <foreach> has an invalid item: {item}"),
                "item must be a valid Rust identifier",
            ));
        }
        if syn::parse_str::<Ident>(&index).is_err() {
            return Err(Diagnostic::new(
                span,
                format!("[rbatis-codegen] <foreach> has an invalid index: {index}"),
                "index must be a valid Rust identifier",
            ));
        }

        Ok(Self {
            collection,
            item,
            index,
            open,
            close,
            separator,
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
        // Create a new ignore list for the children of this foreach loop,
        // including the item and index variables.
        let mut item_specific_ignores = ignore.clone();
        item_specific_ignores.push(self.index.clone());
        item_specific_ignores.push(self.item.clone());

        // Parse children using the new, extended ignore list.
        let foreach_body = context.parse_children(&self.childs, &mut item_specific_ignores)?;

        // The collection expression itself should be parsed using the original ignore list.
        let collection_method_impl = crate::codegen::func::try_impl_fn_with_span(
            "", // body_to_string context placeholder
            "", // fn_name context placeholder
            &format!("\"{}\"", self.collection),
            false,
            ignore, // Use original ignore list for the collection expression
            context.span,
        )
        .map_err(|e| Diagnostic::from_error(e, "check the collection expression"))?;

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
                // drop the separator of the last item. Only the text this loop added
                // is looked at: trimming the whole `sql` would also remove a separator
                // that was part of the SQL written before the <foreach>.
                quote! {
                    if sql.len() > __foreach_sql_start && sql.ends_with(#separator_str) {
                        sql.truncate(sql.len() - #separator_str.len());
                    }
                },
            )
        } else {
            (quote! {}, quote! {})
        };

        let start_binding = if self.separator.is_empty() {
            quote! {}
        } else {
            quote! {
                // where the text of this loop starts, used by #split_code_trim
                let __foreach_sql_start = sql.len();
            }
        };

        Ok(quote! {
            #start_binding
            let mut __foreach_has_items = false;
            for (ref #index_ident, #item_ident) in #collection_method_impl {
                if !__foreach_has_items {
                    #open_impl
                    __foreach_has_items = true;
                }
                #foreach_body
                #split_code
            }
            if __foreach_has_items {
                #split_code_trim
                #close_impl
            }
        })
    }
}
