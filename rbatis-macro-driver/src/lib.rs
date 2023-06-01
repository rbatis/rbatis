#![allow(unused_assignments)]
extern crate proc_macro;
extern crate rbatis_codegen;

use syn::{parse_macro_input, AttributeArgs, ItemFn};

use crate::macros::html_sql_impl::impl_macro_html_sql;
use crate::macros::py_sql_impl::impl_macro_py_sql;
use crate::macros::sql_impl::impl_macro_sql;
use crate::proc_macro::TokenStream;

mod macros;
mod util;

/// auto create sql macro,this macro use RB.query_prepare and RB.exec_prepare
/// for example:
///     pub static RB:Lazy<RBatis> = Lazy::new(||RBatis::new());
///     #[sql(RB, "select * from biz_activity where id = ?")]
///     async fn select(name: &str) -> BizActivity {}
///
/// or:
///     #[sql("select * from biz_activity where id = ?")]
///     async fn select(rb:&RBatis, name: &str) -> BizActivity {}
///
#[proc_macro_attribute]
pub fn sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    if cfg!(debug_assertions) {
        println!("............gen macro sql:\n {}", stream);
        println!("............gen macro sql end............");
    }

    stream
}

/// py sql create macro,this macro use RB.py_query and RB.py_exec
///
///  pub static RB:Lazy<RBatis> = Lazy::new(||RBatis::new());
///  #[py_sql(RB,"select * from biz_activity where delete_flag = 0")]
///  async fn py_select_page(page_req: &PageRequest, name: &str) -> Page<BizActivity> { }
///  or:
///  #[py_sql("select * from biz_activity where delete_flag = 0")]
///  async fn py_select_page(rb: &mut Executor, page_req: &PageRequest, name: &str) -> Page<BizActivity> { }
///
///  or more example:
///  #[py_sql("
///     SELECT * FROM biz_activity
///     if  name != null:
///       AND delete_flag = #{del}
///       AND version = 1
///       if  age!=1:
///         AND version = 1
///       AND version = 1
///     AND a = 0
///       AND version = 1
///     and id in (
///     trim ',': for item in ids:
///       #{item},
///     )
///     and id in (
///     trim ',': for index,item in ids:
///       #{item},
///     )
///     trim 'AND':
///       AND delete_flag = #{del2}
///     choose:
///         when age==27:
///           AND age = 27
///         otherwise:
///           AND age = 0
///     WHERE id  = '2'")]
///   pub async fn py_select_rb(rbatis: &RBatis, name: &str) -> Option<BizActivity> {}
#[proc_macro_attribute]
pub fn py_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_py_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    if cfg!(debug_assertions) {
        use rust_format::{Formatter, RustFmt};
        let stream_str = stream.to_string().replace("$crate", "rbatis");
        let code = RustFmt::default()
            .format_str(&stream_str)
            .unwrap_or_else(|_e| stream_str.to_string());
        println!("............gen macro py_sql :\n {}", code);
        println!("............gen macro py_sql end............");
    }
    stream
}

/// html sql create macro,this macro use RB.py_query and RB.py_exec
/// for example:
///
/// pub static RB:Lazy<RBatis> = Lazy::new(||RBatis::new());
/// #[py_sql(RB,"example/example.html")]
/// pub async fn py_select_rb(name: &str) -> Option<BizActivity> {}
///
/// or:
///
/// #[py_sql("example/example.html")]
/// pub async fn py_select_rb(rbatis: &RBatis, name: &str) -> Option<BizActivity> {}
///
#[proc_macro_attribute]
pub fn html_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_html_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    if cfg!(debug_assertions) {
        use rust_format::{Formatter, RustFmt};
        let stream_str = stream.to_string().replace("$crate", "rbatis");
        let code = RustFmt::default()
            .format_str(&stream_str)
            .unwrap_or_else(|_e| stream_str.to_string());
        println!("............gen macro html_sql :\n {}", code);
        println!("............gen macro html_sql end............");
    }
    stream
}

/// proxy rbatis_codegen rb_py
#[proc_macro_attribute]
pub fn rb_py(args: TokenStream, func: TokenStream) -> TokenStream {
    rbatis_codegen::rb_py(args, func)
}
/// proxy rbatis_codegen rb_html
#[proc_macro_attribute]
pub fn rb_html(args: TokenStream, func: TokenStream) -> TokenStream {
    rbatis_codegen::rb_html(args, func)
}
