use crate::ParseArgs;
use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn::{FnArg, ItemFn, Pat};

use crate::proc_macro::TokenStream;
use crate::util::{find_fn_body, find_return_type, get_fn_args, is_query, is_rb_ref};

//impl sql macro
pub(crate) fn impl_macro_sql(target_fn: &ItemFn, args: &ParseArgs) -> TokenStream {
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
            panic!("[rb] you should add rbatis ref param  `rb:&dyn Executor`  on '{}()'!", target_fn.sig.ident);
        }
        let mut s = "".to_string();
        for v in &args.sqls {
            s = s + v.value().as_str();
        }
        sql_ident = quote!(#s);
    } else {
        panic!("[rb] Incorrect macro parameter length!");
    }

    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let fn_body = find_fn_body(target_fn);
    let is_async = target_fn.sig.asyncness.is_some();
    if !is_async {
        panic!(
            "[rbaits] 'fn {}({})' must be  async fn! ",
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
    let mut decode = quote! {};
    let mut call_method = quote! {};
    let is_query = is_query(&return_ty.to_string());
    if is_query {
        call_method = quote! {query};
        decode = quote! { Ok(rbatis::decode::decode(r)?)}
    } else {
        call_method = quote! {exec};
        decode = quote! { Ok(r)}
    }
    //check use page method
    let page_req_str = String::new();
    let page_req = quote! {};
    //append all args
    let sql_args_gen =
        filter_args_context_id(&rbatis_name, &get_fn_args(target_fn), &[page_req_str]);
    let generic = target_fn.sig.generics.clone();
    //gen rust code templete
    let gen_token_temple = quote! {
       pub async fn #func_name_ident #generic(#func_args_stream) -> #return_ty{
           let mut rb_args =vec![];
           #sql_args_gen
           #fn_body
           use rbatis::executor::{Executor};
           let r= #rbatis_ident.#call_method(&#sql_ident,rb_args #page_req).await?;
           #decode
       }
    };
    return gen_token_temple.into();
}

fn filter_args_context_id(
    rbatis_name: &str,
    fn_arg_name_vec: &Vec<Box<Pat>>,
    skip_names: &[String],
) -> proc_macro2::TokenStream {
    let mut sql_args_gen = quote! {};
    for item in fn_arg_name_vec {
        let item_ident_name = item
            .to_token_stream()
            .to_string()
            .trim()
            .trim_start_matches("mut ")
            .to_string();
        if item_ident_name.eq(rbatis_name) {
            continue;
        }
        let mut do_continue = false;
        for x in skip_names {
            if x.eq(&item_ident_name) {
                do_continue = true;
                break;
            }
        }
        if do_continue {
            continue;
        }
        sql_args_gen = quote! {
             #sql_args_gen
             rb_args.push(rbs::to_value(#item)?);
        };
    }
    sql_args_gen
}
