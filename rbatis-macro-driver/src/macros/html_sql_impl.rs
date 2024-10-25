use std::env::current_dir;
use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::{FnArg, ItemFn};

use crate::macros::py_sql_impl;
use crate::proc_macro::TokenStream;
use crate::util::{find_fn_body, find_return_type, get_fn_args, is_query, is_rb_ref};
use crate::ParseArgs;

pub(crate) fn impl_macro_html_sql(target_fn: &ItemFn, args: &ParseArgs) -> TokenStream {
    let return_ty = find_return_type(target_fn);
    let func_name_ident = target_fn.sig.ident.to_token_stream();

    let mut rbatis_ident = "".to_token_stream();
    let mut rbatis_name = String::new();
    for x in &target_fn.sig.inputs {
        match x {
            FnArg::Receiver(_) => {}
            FnArg::Typed(t) => {
                let ty_stream = t.ty.to_token_stream().to_string();
                if is_rb_ref(&ty_stream) {
                    rbatis_ident = t.pat.to_token_stream();
                    rbatis_name = rbatis_ident
                        .to_string()
                        .trim_start_matches("mut ")
                        .to_string();
                    break;
                }
            }
        }
    }
    let mut sql_ident = quote!();
    if args.sqls.len() >= 1 {
        if rbatis_name.is_empty() {
            panic!("[rb] you should add rbatis ref param   `rb:&dyn Executor`  on '{}()'!", target_fn.sig.ident);
        }
        let mut s = "".to_string();
        for v in &args.sqls {
            s = s + v.value().as_str();
        }
        sql_ident = quote!(#s);
    } else {
        panic!("[rb] Incorrect macro parameter length!");
    }
    // sql_ident is html or file?
    let mut file_name = sql_ident.to_string().trim().to_string();
    if file_name.ends_with(".html\"") {
        file_name = file_name
            .trim_start_matches("\"")
            .trim_end_matches("\"")
            .to_string();
    }
    if file_name.ends_with(".html") {
        //relative path append realpath
        let file_path = PathBuf::from(file_name.clone());
        if file_path.is_relative() {
            let mut current = current_dir().unwrap_or_default();
            current.push(file_name.clone());
            file_name = current.to_str().unwrap_or_default().to_string();
        }
        let mut html_data = String::new();
        let mut f = File::open(file_name.as_str())
            .expect(&format!("File Name = '{}' does not exist", file_name));
        f.read_to_string(&mut html_data)
            .expect(&format!("{} read_to_string fail", file_name));
        let mut htmls = rbatis_codegen::codegen::parser_html::load_mapper_map(&html_data)
            .expect("load html content fail");
        let token = htmls.remove(&func_name_ident.to_string()).expect("");
        let token = format!("{}", token);
        sql_ident = token.to_token_stream();
    }
    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let fn_body = find_fn_body(target_fn);
    let is_async = target_fn.sig.asyncness.is_some();
    if !is_async {
        panic!(
            "[rbaits] fn {}({}) must be  async fn! ",
            func_name_ident, func_args_stream
        );
    }
    if rbatis_ident.to_string().starts_with("mut ") {
        rbatis_ident = Ident::new(
            &rbatis_ident.to_string().trim_start_matches("mut "),
            Span::call_site(),
        )
            .to_token_stream();
    }

    //append all args
    let sql_args_gen = py_sql_impl::filter_args_context_id(&rbatis_name, &get_fn_args(target_fn));
    let is_query = is_query(&return_ty.to_string());
    let mut call_method = quote! {};
    if is_query {
        call_method = quote! {
             use rbatis::executor::{Executor};
             let r=#rbatis_ident.query(&sql,rb_args).await?;
             rbatis::decode::decode(r)
        };
    } else {
        call_method = quote! {
             use rbatis::executor::{Executor};
             #rbatis_ident.exec(&sql,rb_args).await
        };
    }
    let gen_target_method = quote! {
        #[rbatis::rb_html(#sql_ident)]
        pub fn impl_html_sql(arg: &rbs::Value, _tag: char) {}
    };
    let gen_target_macro_arg = quote! {
        #sql_ident
    };
    let gen_func: proc_macro2::TokenStream =
        rbatis_codegen::rb_html(gen_target_macro_arg.into(), gen_target_method.into()).into();

    let mut include_data = quote!();
    include_data = quote! {
        //no-debug_mode
    };
    #[cfg(feature = "debug_mode")]
    if cfg!(debug_assertions) {
        if file_name.ends_with(".html") {
            use std::env::current_dir;
            use std::path::PathBuf;
            let current_dir = current_dir().unwrap();
            let mut html_file_name = file_name.clone();
            if !PathBuf::from(&file_name).is_absolute() {
                html_file_name =
                    format!("{}/{}", current_dir.to_str().unwrap_or_default(), file_name);
            }
            include_data = quote! {#include_data  let _ = include_bytes!(#html_file_name);};
        }
    }
    let generic = target_fn.sig.generics.clone();
    //gen rust code
    let push_count = sql_args_gen
        .to_string()
        .matches("rb_arg_map.insert")
        .count();

    return quote! {
       pub async fn #func_name_ident #generic(#func_args_stream) -> #return_ty {
         #include_data
         let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(#push_count);
         #sql_args_gen
         #fn_body
         use rbatis::executor::{RBatisRef};
         let driver_type = #rbatis_ident.rb_ref().driver_type()?;
         use rbatis::rbatis_codegen;
         #gen_func
         let (mut sql,rb_args) = impl_html_sql(rbs::Value::Map(rb_arg_map),'?');
         #call_method
       }
    }
        .into();
}
