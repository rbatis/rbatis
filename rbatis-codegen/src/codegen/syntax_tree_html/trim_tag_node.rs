use super::{HtmlAstNode, NodeContext};
use crate::codegen::diagnostic::Diagnostic;
use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

/// Represents a `<trim>` tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct TrimTagNode {
    pub prefix: String,
    pub suffix: String,
    pub prefix_overrides: String, // Corresponds to "start" or "prefixOverrides"
    pub suffix_overrides: String, // Corresponds to "end" or "suffixOverrides"
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for TrimTagNode {
    fn node_tag_name() -> &'static str {
        "trim"
    }

    fn try_from_element(element: &Element, _span: Span) -> Result<Self, Diagnostic> {
        let empty = String::new();
        let prefix = element
            .attrs
            .get("prefix")
            .cloned()
            .unwrap_or_else(|| empty.clone());
        let suffix = element
            .attrs
            .get("suffix")
            .cloned()
            .unwrap_or_else(|| empty.clone());
        let prefix_overrides = element
            .attrs
            .get("start")
            .or_else(|| element.attrs.get("prefixOverrides"))
            .or_else(|| element.attrs.get("prefixoverrides"))
            .cloned()
            .unwrap_or_else(|| empty.clone());
        let suffix_overrides = element
            .attrs
            .get("end")
            .or_else(|| element.attrs.get("suffixOverrides"))
            .or_else(|| element.attrs.get("suffixoverrides"))
            .cloned()
            .unwrap_or_else(|| empty.clone());

        Ok(Self {
            prefix,
            suffix,
            prefix_overrides,
            suffix_overrides,
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
        let trim_body = context.parse_children(&self.childs, ignore)?;

        let prefixes: Vec<String> = self
            .prefix_overrides
            .split('|')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let suffixes: Vec<String> = self
            .suffix_overrides
            .split('|')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let capacity = trim_body.to_string().len() + self.prefix.len() + self.suffix.len() + 100;

        // Build the trim chain as an expression over the local `sql` value.
        let mut trim_expr = quote! { sql };

        // 添加前缀去除
        for prefix in &prefixes {
            let p = prefix.as_str();
            trim_expr = quote! { #trim_expr .trim_start_matches(#p) };
        }

        // 添加后缀去除
        for suffix in &suffixes {
            let s = suffix.as_str();
            trim_expr = quote! { #trim_expr .trim_end_matches(#s) };
        }

        let trims = quote! {
            let mut sql = String::with_capacity(#capacity);
            #trim_body
            (#trim_expr).to_string()
        };

        let prefix_str = &self.prefix;
        let suffix_str = &self.suffix;

        // Generate the body in its own `sql` scope, then apply prefix/suffix only
        // when that body produced SQL. This keeps empty conditions from emitting a
        // standalone `set`, parentheses, or other trim markers.
        Ok(quote! {
            let __trim_sql = {
                #trims
            };
            if !__trim_sql.is_empty() {
                sql.push_str(#prefix_str);
                sql.push_str(&__trim_sql);
                sql.push_str(#suffix_str);
            }
        })
    }
}
