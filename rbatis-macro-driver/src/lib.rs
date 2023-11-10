#![allow(unused_assignments)]
extern crate proc_macro;
extern crate rbatis_codegen;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, ItemFn, Token};

use crate::macros::html_sql_impl::impl_macro_html_sql;
use crate::macros::py_sql_impl::impl_macro_py_sql;
use crate::macros::sql_impl::impl_macro_sql;
use crate::proc_macro::TokenStream;

mod macros;
mod util;

struct ParseArgs {
    pub sqls: Vec<syn::LitStr>,
}

impl Parse for ParseArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let r = Punctuated::<syn::LitStr, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            sqls: r.into_iter().collect(),
        })
    }
}

/// auto create sql macro,this macro use RB.query_prepare and RB.exec_prepare
/// for example:
///```log
///     use rbatis::sql;
///     use rbatis::executor::Executor;
///     #[derive(serde::Serialize,serde::Deserialize)]
///     pub struct BizActivity{}
///
///     #[sql("select * from biz_activity where id = ?")]
///     async fn select(rb:&dyn Executor, name: &str) -> BizActivity {}
///```
#[proc_macro_attribute]
pub fn sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
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
///```log
/// use rbatis::executor::Executor;
/// use rbatis::py_sql;
/// #[derive(serde::Serialize,serde::Deserialize)]
/// pub struct BizActivity{}
///
/// #[py_sql("select * from biz_activity where delete_flag = 0")]
/// async fn py_select_page(rb: &dyn Executor, name: &str) -> Vec<BizActivity> { }
///```
///  or more example:
///```log
/// use rbatis::executor::Executor;
/// use rbatis::py_sql;
/// #[derive(serde::Serialize,serde::Deserialize)]
/// pub struct BizActivity{}
///
/// #[py_sql("
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
///   pub async fn py_select_rb(rb: &dyn Executor, name: &str) -> Option<BizActivity> {}
/// ```
/// or read from file
/// ```rust
/// //#[rbatis::py_sql(r#"include!("C:/rs/rbatis/target/debug/xx.py_sql")"#)]
/// //pub async fn test_same_id(rb: &dyn Executor, id: &u64) -> Result<Value, Error> { impled!() }
/// ```
#[proc_macro_attribute]
pub fn py_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_py_sql(&target_fn, args);
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
/// ```log
/// use rbatis::executor::Executor;
/// use rbatis::html_sql;
/// #[derive(serde::Serialize,serde::Deserialize)]
/// pub struct BizActivity{}
///
/// #[html_sql(r#"
/// <select id="select_by_condition">
///         `select * from activity`
///         <where>
///             <if test="name != ''">
///                 ` and name like #{name}`
///             </if>
///             <if test="dt >= '2023-11-03T21:13:09.9357266+08:00'">
///                 ` and create_time < #{dt}`
///             </if>
///             <choose>
///                 <when test="true">
///                     ` and id != '-1'`
///                 </when>
///                 <otherwise>and id != -2</otherwise>
///             </choose>
///             ` and `
///             <trim prefixOverrides=" and">
///                 ` and name != '' `
///             </trim>
///         </where>
///   </select>"#)]
/// pub async fn select_by_name(rbatis: &dyn Executor, name: &str) -> Option<BizActivity> {}
/// ```
/// or from file
/// ```log
/// #[html_sql("xxxx.html")]
/// pub async fn select_by_name(rbatis: &dyn Executor, name: &str) -> Option<BizActivity> {}
/// ```
#[proc_macro_attribute]
pub fn html_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
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