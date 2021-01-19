#![allow(unused_assignments)]
extern crate proc_macro;

use std::collections::HashMap;

use syn::{parse_macro_input, AttributeArgs, ItemFn};

use crate::proc_macro::TokenStream;

mod crud_enable;
mod py_sql;
mod sql;
mod util;

#[proc_macro_derive(CRUDEnable)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let stream = crud_enable::impl_crud_driver(&ast, "", "", "", "", &HashMap::new());
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen impl CRUDEnable:\n {}", stream);
        println!("............gen impl CRUDEnable end............");
    }

    stream
}

/// auto create sql macro,this macro use RB.fetch_prepare and RB.exec_prepare
/// for example:
///     #[sql(RB, "select * from biz_activity where id = ?")]
///     async fn select(name: &str) -> BizActivity {}
#[proc_macro_attribute]
pub fn sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = sql::impl_macro_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen macro sql:\n {}", stream);
        println!("............gen macro sql end............");
    }

    stream
}

/// py sql create macro,this macro use RB.py_fetch and RB.py_exec
/// for example:
///
///
///  also,you can use arg  context_id:&str,RB:&Rbatis
///     #[py_sql(RB, "select * from biz_activity where id = #{name}
///                   if name != '':
///                     and name=#{name}")]
/// pub async fn py_select_rb(rbatis: &Rbatis, name: &str) -> Option<BizActivity> {}
#[proc_macro_attribute]
pub fn py_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = py_sql::impl_macro_py_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen macro py_sql :\n {}", stream);
        println!("............gen macro py_sql end............");
    }
    stream
}

#[proc_macro_attribute]
pub fn crud_enable(args: TokenStream, input: TokenStream) -> TokenStream {
    let stream = crud_enable::impl_crud(args, input);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen impl CRUDEnable:\n {}", stream);
        println!("............gen impl CRUDEnable end............");
    }

    return stream;
}
