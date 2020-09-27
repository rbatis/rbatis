extern crate proc_macro;

use syn::{AttributeArgs, Data, FnArg, ItemFn, parse_macro_input, ReturnType};

use crate::proc_macro::TokenStream;

mod table;
mod sql;
mod string_util;
mod py_sql;

#[proc_macro_derive(CRUDEnable)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    table::impl_macro(&ast)
}


/// sql create macro
#[proc_macro_attribute]
pub fn sql(args: TokenStream, this: TokenStream) -> TokenStream {
    println!("............proc_macro_attribute sql start............");

    // this
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(this).unwrap();

    let stream = sql::impl_macro_sql(&target_fn, &args);

    println!("............gen rust code:\n {}", format!("{}", stream));
    println!("............proc_macro_attribute sql end............");

    stream
}

/// py sql create macro
#[proc_macro_attribute]
pub fn py_sql(args: TokenStream, this: TokenStream) -> TokenStream {
    println!("............proc_macro_attribute py sql start............");

    // this
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(this).unwrap();

    let stream = py_sql::impl_macro_py_sql(&target_fn, &args);

    println!("............gen rust code:\n {}", format!("{}", stream));
    println!("............proc_macro_attribute py sql end............");

    stream
}
