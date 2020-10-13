extern crate proc_macro;

use syn::{AttributeArgs, Data, ItemFn, parse_macro_input, ReturnType};

use crate::proc_macro::TokenStream;

mod util;
mod crud_enable;
mod sql;
mod py_sql;

#[proc_macro_derive(CRUDEnable)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    println!("............gen impl CRUDEnable start............");
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    let stream = crud_enable::impl_macro(&ast);

    println!("............gen impl CRUDEnable:\n {}", format!("{}", stream));
    println!("............gen impl CRUDEnable end............");
    stream
}


/// sql create macro
#[proc_macro_attribute]
pub fn sql(args: TokenStream, func: TokenStream) -> TokenStream {
    println!("............gen macro sql start............");

    // this
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();

    let stream = sql::impl_macro_sql(&target_fn, &args);

    println!("............gen macro sql:\n {}", format!("{}", stream));
    println!("............gen macro sql end............");

    stream
}

/// py sql create macro
/// for example:
///
///
///  also,you can use arg  tx_id:&str,RB:&Rbatis
///     #[py_sql(RB, "select * from biz_activity where id = #{name}
///                   if name != '':
///                     and name=#{name}")]
/// pub async fn py_select_rb(rbatis: &Rbatis, name: &str) -> Option<BizActivity> {}
#[proc_macro_attribute]
pub fn py_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    println!("............gen macro py_sql start............");

    // this
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();

    let stream = py_sql::impl_macro_py_sql(&target_fn, &args);

    println!("............gen macro py_sql :\n {}", format!("{}", stream));
    println!("............gen macro py_sql end............");

    stream
}
