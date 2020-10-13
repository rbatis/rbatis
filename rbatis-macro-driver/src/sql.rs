use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, Data, FnArg, ItemFn, parse_macro_input, ReturnType};

use crate::proc_macro::TokenStream;

//impl sql macro
pub(crate) fn impl_macro_sql(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let mut return_ty = find_return_type(target_fn);
    let func_name_ident = target_fn.sig.ident.to_token_stream();
    let func_name = format!("{}", target_fn.sig.ident.to_token_stream());
    let rbatis_ident = args.get(0).unwrap().to_token_stream();
    let rbatis_name = format!("{}", rbatis_ident);
    let sql_ident = args.get(1).unwrap().to_token_stream();
    let sql = format!("{}", sql_ident).trim().to_string();
    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    //append all args
    let (sql_args_gen, tx_id_ident) = filter_args_tx_id(&rbatis_name, &get_fn_args(target_fn));
    let mut call_method = quote! {};
    let is_select = sql.starts_with("select ") || sql.starts_with("SELECT ") || sql.starts_with("\"select ") || sql.starts_with("\"SELECT ");
    if is_select {
        call_method = quote! {fetch_prepare};
    } else {
        call_method = quote! {execute_parperd};
    }
    //gen rust code templete
    let gen_token_temple = quote! {
       pub async fn #func_name_ident(#func_args_stream) -> #return_ty{
           let mut args =vec![];
           #sql_args_gen
           return #rbatis_ident.#call_method(#tx_id_ident,#sql_ident,&args).await;
       }
    };
    return gen_token_temple.into();
}


//find and check method return type
fn find_return_type(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    let mut return_ty = target_fn.sig.output.to_token_stream();
    match &target_fn.sig.output {
        ReturnType::Type(_, b) => {
            return_ty = b.to_token_stream();
        }
        _ => {}
    }
    let mut s = format!("{}", return_ty);
    if !s.starts_with("rbatis_core :: Result") && !s.starts_with("Result") && !s.starts_with("std :: result :: Result") {
        return_ty = quote! {
             rbatis_core :: Result <#return_ty>
        };
    }
    return_ty
}

fn get_fn_args(target_fn: &ItemFn) -> Vec<String> {
    let mut fn_arg_name_vec = vec![];
    for arg in &target_fn.sig.inputs {
        match arg {
            FnArg::Typed(t) => {
                let arg_name = format!("{}", t.pat.to_token_stream());
                fn_arg_name_vec.push(arg_name);
                //println!("arg_name {}", arg_name);
            }
            _ => {}
        }
    }
    fn_arg_name_vec
}

fn filter_args_tx_id(rbatis_name: &str, fn_arg_name_vec: &Vec<String>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut sql_args_gen = quote! {};
    let mut tx_id_ident = quote! {""};
    for item in fn_arg_name_vec {
        let item_ident = Ident::new(&item, Span::call_site());
        if item.eq(&rbatis_name) {
            continue;
        }
        if item.eq("tx_id") {
            tx_id_ident = item_ident.to_token_stream();
            continue;
        }
        sql_args_gen = quote! {
            #sql_args_gen
            args.push(serde_json::to_value(#item_ident).unwrap_or(serde_json::Value::Null));
       };
    }
    (sql_args_gen, tx_id_ident)
}