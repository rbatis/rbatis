// suppress "正在创建库 xxx.lib 和对象 xxx.exp" noise from link.exe on Windows/MSVC
#![allow(linker_messages)]
#![allow(unused_assignments)]
//! Procedural macros of rbatis.
//!
//! These macros generate the Rust code that runs a statement, and are re-exported by
//! the `rbatis` crate, so they are normally written as `rbatis::py_sql!` or
//! `#[rbatis::py_sql("...")]`.
//!
//! * [`py_sql`] - python style dynamic SQL (`if`, `for`, `bind`, `trim`, `${}`, `#{}`)
//! * [`html_sql`] - Mybatis 3 compatible XML dynamic SQL (`<if>`, `<where>`, `<foreach>`, ...)
//! * [`sql`] - a raw SQL statement with `?` placeholders
//! * [`snake_name`] - `CamelCase` to `snake_case` at compile time, used to derive table names
//!
//! All of them work on the declaration of a function: the signature is kept and the
//! body is replaced by the generated code (`impled!()` is the customary placeholder).
//! See [`rbatis`](https://docs.rs/rbatis/latest/rbatis/) for how they are used.

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
///     use rbatis::plugin;
///     use rbatis::executor::Executor;
///     #[derive(serde::Serialize,serde::Deserialize)]
///     pub struct MockTable{}
///
///     #[sql("select * from biz_activity where id = ?")]
///     async fn select(rb:&dyn Executor, name: &str) -> MockTable {}
///```
#[proc_macro_attribute]
pub fn sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_sql(&target_fn, &args);
    #[cfg(feature = "println_gen")]
    if cfg!(debug_assertions) {
        use quote::ToTokens;
        let func_name_ident = target_fn.sig.ident.to_token_stream();
        println!(
            "............#[sql] '{}'...................\n {}",
            func_name_ident, stream
        );
        println!(".......................................................");
    }

    stream
}

/// py sql create macro,this macro use RB.py_query and RB.py_exec
///```log
/// use rbatis::executor::Executor;
/// use rbatis::py_sql;
/// #[derive(serde::Serialize,serde::Deserialize)]
/// pub struct MockTable{}
///
/// #[py_sql("select * from biz_activity where delete_flag = 0")]
/// async fn py_select_page(rb: &dyn Executor, name: &str) -> Vec<MockTable> { }
///```
///  or more example:
///```log
/// use rbatis::executor::Executor;
/// use rbatis::py_sql;
/// #[derive(serde::Serialize,serde::Deserialize)]
/// pub struct MockTable{}
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
///   pub async fn py_select_rb(rb: &dyn Executor, name: &str) -> Option<MockTable> {}
/// ```
/// or read from file
/// ```text
/// //#[rbatis::py_sql(r#"include!("C:/rs/rbatis/target/debug/xx.py_sql")"#)]
/// //pub async fn test_same_id(rb: &dyn Executor, id: &u64) -> Result<Value, Error> { impled!() }
/// ```
#[proc_macro_attribute]
pub fn py_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_py_sql(&target_fn, args);
    #[cfg(feature = "println_gen")]
    if cfg!(debug_assertions) {
        use quote::ToTokens;
        use rust_format::{Formatter, RustFmt};
        let func_name_ident = target_fn.sig.ident.to_token_stream();
        let stream_str = stream.to_string().replace("$crate", "rbatis");
        let code = RustFmt::default()
            .format_str(&stream_str)
            .unwrap_or_else(|_e| stream_str.to_string());
        println!(
            "............#[py_sql] '{}'............\n {}",
            func_name_ident, code
        );
        println!(".......................................................");
    }
    stream
}

/// html sql create macro,this macro use RB.py_query and RB.py_exec
/// for example:
/// ```log
/// use rbatis::executor::Executor;
/// use rbatis::html_sql;
/// #[derive(serde::Serialize,serde::Deserialize)]
/// pub struct MockTable{}
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
/// pub async fn select_by_name(rbatis: &dyn Executor, name: &str) -> Option<MockTable> {}
/// ```
/// or from file
/// ```log
/// #[html_sql("xxxx.html")]
/// pub async fn select_by_name(rbatis: &dyn Executor, name: &str) -> Option<MockTable> {}
/// ```
/// or impl block level
/// ```log
/// #[html_sql("xxxx.html")]
/// impl MockTable {
///     async fn select_1(rb: &impl Executor) -> rbatis::Result<Vec<MockTable>> {
///         impled!()
///     }
///     async fn select_2(rb: &impl Executor) -> rbatis::Result<Vec<MockTable>> {
///         impled!()
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn html_sql(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);

    // Parse as impl
    if let Ok(impl_block) = syn::parse::<syn::ItemImpl>(item.clone()) {
        return crate::macros::html_sql_impl::impl_macro_html_sql_impl(&impl_block, &args);
    }

    // Parse as function
    let target_fn: ItemFn = syn::parse(item).unwrap();
    let stream = impl_macro_html_sql(&target_fn, &args, false);
    #[cfg(feature = "println_gen")]
    if cfg!(debug_assertions) {
        use quote::ToTokens;
        use rust_format::{Formatter, RustFmt};
        let func_name_ident = target_fn.sig.ident.to_token_stream();
        let stream_str = stream.to_string().replace("$crate", "rbatis");
        let code = RustFmt::default()
            .format_str(&stream_str)
            .unwrap_or_else(|_e| stream_str.to_string());
        println!(
            "............#[html_sql] '{}'............\n {}",
            func_name_ident, code
        );
        println!(".......................................................");
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

/// turn a function declaration into a function that returns the `snake_case` name of
/// the type given as the first argument:
///
/// ```rust,ignore
/// #[rbatis::snake_name(BizActivity)]
/// fn table_name() {}
/// assert_eq!(table_name(), "biz_activity");
/// ```
///
/// `crud!` uses this to derive a table name from a struct name when no table name
/// was passed explicitly.
#[proc_macro_attribute]
pub fn snake_name(args: TokenStream, func: TokenStream) -> TokenStream {
    macros::snake_name::snake_name(args, func)
}
