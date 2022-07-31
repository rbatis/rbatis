#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_mut)]

extern crate proc_macro;

use syn::{AttributeArgs, DataStruct, ItemFn, parse_macro_input};

use crate::proc_macro::TokenStream;

mod func;
mod parser;
mod html_loader;
mod string_util;
mod py_sql;
mod element_from;

#[proc_macro_attribute]
pub fn expr(args: TokenStream, func: TokenStream) -> TokenStream {
    //let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = func::impl_fn("",&target_fn.sig.ident.to_string(), &args.to_string(),true,true,&[]).into();
    #[cfg(feature = "debug_mode")]
        {
            println!("............gen macro rexpr:\n {}", stream);
            println!("............gen macro rexpr end............");
        }
    stream
}


#[proc_macro_attribute]
pub fn rb_html(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn = syn::parse(func).unwrap();
    let stream = parser::impl_fn_html(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
        {
            println!("............gen macro xml:\n {}", stream);
            println!("............gen macro xml end............");
        }
    stream
}

/// support py_sql fn convert
#[proc_macro_attribute]
pub fn rb_py(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn = syn::parse(func).unwrap();
    let stream = parser::impl_fn_py(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
        {
            println!("............gen rb_pysql_fn:\n {}", stream);
            println!("............gen rb_pysql_fn end............");
        }
    stream
}