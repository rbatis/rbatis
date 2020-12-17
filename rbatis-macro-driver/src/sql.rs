use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, ItemFn};

use crate::proc_macro::TokenStream;
use crate::util::{find_return_type, get_fn_args, get_page_req_ident};

//impl sql macro
pub(crate) fn impl_macro_sql(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let return_ty = find_return_type(target_fn);
    let func_name_ident = target_fn.sig.ident.to_token_stream();
    let rbatis_ident = args.get(0).unwrap().to_token_stream();
    let rbatis_name = format!("{}", rbatis_ident);
    let sql_ident = args.get(1).unwrap().to_token_stream();
    let sql = format!("{}", sql_ident).trim().to_string();
    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let is_async = target_fn.sig.asyncness.is_some();
    if !is_async {
        panic!(format!("[rbaits] 'fn {}({})' must be  async fn! ", func_name_ident, func_args_stream));
    }
    let mut call_method = quote! {};
    let is_select = sql.starts_with("select ") || sql.starts_with("SELECT ") || sql.starts_with("\"select ") || sql.starts_with("\"SELECT ");
    if is_select {
        call_method = quote! {fetch_prepare};
    } else {
        call_method = quote! {exec_prepare};
    }
    //check use page method
    let mut page_req_str = "".to_string();
    let mut page_req = quote! {};
    if return_ty.to_string().contains("Page <")
        && func_args_stream.to_string().contains("& PageRequest") {
        let req = get_page_req_ident(target_fn, &func_name_ident.to_string());
        page_req_str = req.to_string();
        page_req = quote! {,#req};
        call_method = quote! {fetch_page};
    }
    //append all args
    let (sql_args_gen, context_id_ident) = filter_args_context_id(&rbatis_name, &get_fn_args(target_fn), &[page_req_str]);
    //gen rust code templete
    let gen_token_temple = quote! {
       pub async fn #func_name_ident(#func_args_stream) -> #return_ty{
           let mut rb_args =vec![];
           #sql_args_gen
           return #rbatis_ident.#call_method(#context_id_ident,#sql_ident,&rb_args #page_req).await;
       }
    };
    return gen_token_temple.into();
}

fn filter_args_context_id(rbatis_name: &str, fn_arg_name_vec: &Vec<String>, skip_names: &[String]) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut sql_args_gen = quote! {};
    let mut context_id_ident = quote! {""};
    for item in fn_arg_name_vec {
        let item_ident = Ident::new(&item, Span::call_site());
        let item_ident_name = item_ident.to_string();
        if item.eq(&rbatis_name) {
            continue;
        }
        if item.eq("ctx_id") ||item.eq("context_id") || item.eq("tx_id") {
            context_id_ident = item_ident.to_token_stream();
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
            rb_args.push(serde_json::to_value(#item_ident).unwrap_or(serde_json::Value::Null));
       };
    }
    (sql_args_gen, context_id_ident)
}