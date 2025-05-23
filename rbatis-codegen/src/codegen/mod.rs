/// The core logic of code generation is written in func.rs
/// The syntax tree we use is the html tag structure loader_html.rs Element struct
extern crate proc_macro;

use syn::{parse_macro_input, ItemFn, Token};

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

pub mod func;
pub mod loader_html;
pub mod parser_html;
pub mod parser_pysql;
pub mod parser_pysql_pest;
pub mod string_util;
pub mod syntax_tree_pysql;

pub struct ParseArgs {
    pub sqls: Vec<syn::LitStr>,
}

// 实现Clone特性
impl Clone for ParseArgs {
    fn clone(&self) -> Self {
        let mut new_sqls = Vec::new();
        for sql in &self.sqls {
            let content = sql.value();
            let new_sql = syn::LitStr::new(&content, sql.span());
            new_sqls.push(new_sql);
        }
        ParseArgs { sqls: new_sqls }
    }
}

impl Parse for ParseArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let r = Punctuated::<syn::LitStr, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            sqls: r.into_iter().collect(),
        })
    }
}

pub fn expr(args: TokenStream, func: TokenStream) -> TokenStream {
    //let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = func::impl_fn(
        "",
        &target_fn.sig.ident.to_string(),
        &args.to_string(),
        true,
        &[],
    )
    .into();
    stream
}

pub fn rb_html(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let target_fn = syn::parse(func).unwrap();
    let stream = parser_html::impl_fn_html(&target_fn, &args);
    stream
}

/// support py_sql fn convert
pub fn rb_py(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let target_fn = syn::parse(func).unwrap();
    
    // 使用Pest解析器
    #[cfg(feature = "use_pest")]
    {
        let stream = parser_pysql_pest::impl_fn_py(&target_fn, &args);
        return stream;
    }
    
    // 默认使用原始解析器
    #[cfg(not(feature = "use_pest"))]
    {
        let stream = parser_pysql::impl_fn_py(&target_fn, &args);
        return stream;
    }
}
