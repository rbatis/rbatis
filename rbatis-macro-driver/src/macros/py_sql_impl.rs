use std::env::current_dir;
use crate::ParseArgs;
use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::{FnArg, ItemFn, Pat};

use crate::proc_macro::TokenStream;
use crate::util::{find_fn_body, find_return_type, get_fn_args, is_query, is_rb_ref};

///py_sql macro
///support args for rb:&RBatis
pub(crate) fn impl_macro_py_sql(target_fn: &ItemFn, args: ParseArgs) -> TokenStream {
    let return_ty = find_return_type(target_fn);
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

    let mut include_data = quote::quote! {};
    let mut sql_ident = quote!();
    if args.sqls.len() >= 1 {
        if rbatis_name.is_empty() {
            panic!("[rb] you should add rbatis ref param  `rb:&dyn Executor`  on '{}()'!", target_fn.sig.ident);
        }
        let mut s = "".to_string();
        for v in &args.sqls {
            if args.sqls.len() == 1 {
                let v = args.sqls[0].value();
                if v.starts_with("include_str!(\"") && v.ends_with("\")") {
                    let mut file_name = v
                        .trim_start_matches(r#"include_str!(""#)
                        .trim_end_matches(r#"")"#)
                        .to_string();
                    let file_path = PathBuf::from(file_name.clone());
                    if file_path.is_relative() {
                        let mut manifest_dir =
                            std::env::var("CARGO_MANIFEST_DIR").expect("Failed to read CARGO_MANIFEST_DIR");
                        manifest_dir.push_str("/");
                        let mut current = PathBuf::from(manifest_dir);
                        current.push(file_name.clone());
                        if !current.exists() {
                            current = current_dir().unwrap_or_default();
                            current.push(file_name.clone());
                        }
                        file_name = current.to_str().unwrap_or_default().to_string();
                    }
                    let mut f = File::open(&file_name).expect(&format!("can't find file={}", file_name));
                    let mut data = String::new();
                    _ = f.read_to_string(&mut data);
                    data = data.replace("\r\n", "\n");

                    #[cfg(feature = "debug_mode")]
                    if cfg!(debug_assertions) {
                        use std::env::current_dir;
                        use std::path::PathBuf;
                        let current_dir = current_dir().unwrap();
                        let mut include_file_name = file_name.to_string();
                        if !PathBuf::from(&file_name).is_absolute() {
                            include_file_name = format!(
                                "{}/{}",
                                current_dir.to_str().unwrap_or_default(),
                                file_name
                            );
                        }
                        include_data =
                            quote! {#include_data  let _ = include_bytes!(#include_file_name);};
                    }
                    s.push_str(&data);
                    continue;
                }
            }
            s = s + v.value().as_str();
        }
        sql_ident = quote!(#s);
    } else {
        panic!("[rb] Incorrect macro parameter length!");
    }
    include_data = include_data.clone();

    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let fn_body = find_fn_body(target_fn);
    let is_async = target_fn.sig.asyncness.is_some();
    let func_name_ident = target_fn.sig.ident.to_token_stream();
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
    let sql_args_gen = filter_args_context_id(&rbatis_name, &get_fn_args(target_fn));
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
        #[rbatis::rb_py(#sql_ident)]
        pub fn do_py_sql(arg: &rbs::Value, _tag: char) {}
    };
    let gen_target_macro_arg = quote! {
        #sql_ident
    };
    let gen_func: proc_macro2::TokenStream =
        rbatis_codegen::rb_py(gen_target_macro_arg.into(), gen_target_method.into()).into();

    let generic = target_fn.sig.generics.clone();

    let push_count = sql_args_gen
        .to_string()
        .matches("rb_arg_map.insert")
        .count();
    //gen rust code templete
    return quote! {
       pub async fn #func_name_ident #generic(#func_args_stream) -> #return_ty {
         let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(#push_count);
         #sql_args_gen
         #fn_body
         use rbatis::executor::{RBatisRef};
         let driver_type = #rbatis_ident.rb_ref().driver_type()?;
         use rbatis::rbatis_codegen;
         #include_data
         #gen_func
         let (mut sql,rb_args) = do_py_sql(rbs::Value::Map(rb_arg_map), '?');
         #call_method
       }
    }
    .into();
}

pub(crate) fn filter_args_context_id(
    rbatis_name: &str,
    fn_arg_name_vec: &Vec<Box<Pat>>,
) -> proc_macro2::TokenStream {
    let mut sql_args_gen = quote! {};
    for item in fn_arg_name_vec {
        let item_name = item
            .to_token_stream()
            .to_string()
            .trim()
            .trim_start_matches("mut ")
            .to_string();
        if item_name.eq(rbatis_name) {
            continue;
        }
        let mut item = item.to_token_stream();
        if item.to_string().starts_with("mut ") {
            item = proc_macro2::Ident::new(
                item.to_string().trim_start_matches("mut "),
                Span::call_site(),
            )
            .to_token_stream();
        }
        sql_args_gen = quote! {
             #sql_args_gen
             rb_arg_map.insert(#item_name.to_string().into(),rbs::to_value(#item)?);
        };
    }
    sql_args_gen
}
