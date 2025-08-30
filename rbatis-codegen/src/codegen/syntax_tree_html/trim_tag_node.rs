use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext};

/// Represents a <trim> tag node in the HTML AST.
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
    fn node_tag_name() -> &'static str { "trim" }

    fn from_element(element: &Element) -> Self {
        let empty = String::new();
        let prefix = element.attrs.get("prefix").cloned().unwrap_or_else(|| empty.clone());
        let suffix = element.attrs.get("suffix").cloned().unwrap_or_else(|| empty.clone());
        let prefix_overrides = element.attrs.get("start")
            .or_else(|| element.attrs.get("prefixOverrides"))
            .cloned()
            .unwrap_or_else(|| empty.clone());
        let suffix_overrides = element.attrs.get("end")
            .or_else(|| element.attrs.get("suffixOverrides"))
            .cloned()
            .unwrap_or_else(|| empty.clone());

        Self {
            prefix,
            suffix,
            prefix_overrides,
            suffix_overrides,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        let trim_body = context.parse_children(&self.childs, ignore);

        let prefixes: Vec<String> = self.prefix_overrides.split('|')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let suffixes: Vec<String> = self.suffix_overrides.split('|')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let has_trim = !prefixes.is_empty() || !suffixes.is_empty();
        let capacity = trim_body.to_string().len() + self.prefix.len() + self.suffix.len() + 100;

        // 创建基础的trims表达式
        let mut trims = quote! {
            let mut sql = String::with_capacity(#capacity);
            #trim_body
            sql = sql
        };

        // 添加前缀去除
        for prefix in &prefixes {
            let p = prefix.as_str();
            trims = quote! { #trims .trim_start_matches(#p) };
        }

        // 添加后缀去除
        for suffix in &suffixes {
            let s = suffix.as_str();
            trims = quote! { #trims .trim_end_matches(#s) };
        }

        let mut final_tokens = quote! {};

        // 添加前缀（如果有）
        if !self.prefix.is_empty() {
            let prefix_str = &self.prefix;
            final_tokens.extend(quote! {
                sql.push_str(#prefix_str);
            });
        }

        // 添加内容（无论是否需要去除前后缀）
        if has_trim {
            final_tokens.extend(quote! {
                rbatis_codegen::codegen::string_util::concat_str(&mut sql, &{#trims.to_string(); sql});
            });
        } else {
            final_tokens.extend(quote! {
                {
                    let mut inner_sql = String::with_capacity(#capacity);
                    #trim_body
                    rbatis_codegen::codegen::string_util::concat_str(&mut sql, &inner_sql);
                }
            });
        }

        // 添加后缀（如果有）
        if !self.suffix.is_empty() {
            let suffix_str = &self.suffix;
            final_tokens.extend(quote! {
                sql.push_str(#suffix_str);
            });
        }

        final_tokens
    }
} 