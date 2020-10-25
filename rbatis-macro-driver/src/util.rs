use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{ FnArg, ItemFn, ReturnType};

//find and check method return type
pub(crate) fn find_return_type(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    let mut return_ty = target_fn.sig.output.to_token_stream();
    match &target_fn.sig.output {
        ReturnType::Type(_, b) => {
            return_ty = b.to_token_stream();
        }
        _ => {}
    }
    let s = format!("{}", return_ty);
    if !s.starts_with("rbatis_core :: Result") && !s.starts_with("Result") && !s.starts_with("std :: result :: Result") {
        return_ty = quote! {
             rbatis_core :: Result <#return_ty>
        };
    }
    return_ty
}

pub(crate) fn get_fn_args(target_fn: &ItemFn) -> Vec<String> {
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

pub(crate) fn filter_fn_args(target_fn: &ItemFn, arg_name: &str, arg_type: &str) -> std::collections::HashMap<String, String> {
    let mut map = HashMap::new();
    for arg in &target_fn.sig.inputs {
        match arg {
            FnArg::Typed(t) => {
                let arg_name_value = format!("{}", t.pat.to_token_stream());
                if arg_name.eq(&arg_name_value) {
                    map.insert(arg_name.to_string(), arg_name_value.clone());
                }
                let arg_type_name = t.ty.to_token_stream().to_string();
                if arg_type.eq(&arg_type_name) {
                    map.insert(arg_type.to_string(), arg_name_value.clone());
                }
            }
            _ => {}
        }
    }
    map
}

pub(crate) fn get_page_req_ident(target_fn: &ItemFn, func_name: &str) -> Ident {
    let page_reqs = filter_fn_args(target_fn, "", "& PageRequest");
    if page_reqs.len() > 1 {
        panic!("[rbatis] {} only support on arg of '**:&PageRequest'!", func_name);
    }
    if page_reqs.len() == 0 {
        panic!("[rbatis] {} method arg must have arg Type '**:&PageRequest'!", func_name);
    }
    let req = page_reqs.get("& PageRequest").unwrap_or(&"".to_string()).to_owned();
    if req.eq("") {
        panic!("[rbatis] {} method arg must have arg Type '**:&PageRequest'!", func_name);
    }
    let req = Ident::new(&req, Span::call_site());
    req
}