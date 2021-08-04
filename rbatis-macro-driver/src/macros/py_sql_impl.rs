use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, ItemFn};

use crate::proc_macro::TokenStream;
use crate::util::{find_fn_body, find_return_type, get_fn_args, get_page_req_ident, is_fetch};
use crate::macros::html_loader::Element;
use std::io::Read;

///py_sql macro
///support args for RB:&Rbatis,page:&PageRequest
///support return for Page<*>
pub(crate) fn impl_macro_py_sql(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let return_ty = find_return_type(target_fn);
    let func_name_ident = target_fn.sig.ident.to_token_stream();
    let rbatis_ident = args.get(0).expect("[rbatis] miss rbatis ident param!").to_token_stream();
    let rbatis_name = format!("{}", rbatis_ident);
    let sql_ident = args.get(1).expect("[rbatis] miss pysql param!").to_token_stream();
    let sql = format!("{}", sql_ident).trim().to_string();
    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let fn_body = find_fn_body(target_fn);
    let is_async = target_fn.sig.asyncness.is_some();
    if !is_async {
        panic!(
            "[rbaits] #[crud_table] 'fn {}({})' must be  async fn! ",
            func_name_ident, func_args_stream
        );
    }
    //append all args
    let sql_args_gen = filter_args_context_id(&rbatis_name, &get_fn_args(target_fn));
    let is_fetch = is_fetch(&return_ty.to_string());
    let mut call_method = quote! {};
    if is_fetch {
        call_method = quote! {
             use rbatis::executor::{Executor,ExecutorMut};
             #rbatis_ident.fetch(&sql,&rb_args).await
        };
    } else {
        call_method = quote! {
             use rbatis::executor::{Executor,ExecutorMut};
             #rbatis_ident.exec(&sql,&rb_args).await
        };
    }
    if return_ty.to_string().contains("Page <")
        && func_args_stream.to_string().contains("& PageRequest")
    {
        let page_ident = get_page_req_ident(target_fn, &func_name_ident.to_string());
        call_method = quote! {
            use rbatis::crud::{CRUD,CRUDMut};
            #rbatis_ident.fetch_page(&sql,&rb_args,#page_ident).await
        };
        println!("gen return");
    }

    //gen rust code templete
    return quote! {
       pub async fn #func_name_ident(#func_args_stream) -> #return_ty {
         let mut sql = #sql_ident.to_string();
         let mut rb_arg_map = serde_json::Map::new();
         #sql_args_gen
         #fn_body
         use rbatis::executor::{RbatisRef};
         let driver_type = #rbatis_ident.get_rbatis().driver_type()?;
         use rbatis::rbatis_sql;
         match driver_type{
            rbatis::DriverType::Postgres => {
                #[rb_py(#sql_ident,'$')]
                pub fn #func_name_ident(arg: &mut serde_json::Value) {}
                let (mut sql,rb_args) = #func_name_ident(&mut serde_json::Value::Object(rb_arg_map));
                #call_method
            }
            rbatis::DriverType::Mssql => {
                #[rb_py(#sql_ident,'$')]
                pub fn #func_name_ident(arg: &mut serde_json::Value) {}
                let (mut sql,rb_args) = #func_name_ident(&mut serde_json::Value::Object(rb_arg_map));
                sql = sql.replace("$","@p");
                #call_method
            }
            _=> {
                #[rb_py(#sql_ident,'?')]
                pub fn #func_name_ident(arg: &mut serde_json::Value) {}
                let (mut sql,rb_args) = #func_name_ident(&mut serde_json::Value::Object(rb_arg_map));
                #call_method
            }
         }
       }
    }
        .into();
}


fn find_node(arg: &Vec<Element>, name: &str) -> Option<Element> {
    for x in arg {
        match x.tag.as_str() {
            "mapper" => {
                return find_node(&x.childs, name);
            }
            "insert" | "select" | "update" | "delete" => {
                match x.attributes.get("id") {
                    Some(v) => {
                        if v.eq(name) {
                            return Some(x.clone());
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    return None;
}

pub(crate) fn impl_macro_html_sql(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let return_ty = find_return_type(target_fn);
    let func_name_ident = target_fn.sig.ident.to_token_stream();
    let rbatis_ident = args.get(0).expect("[rbatis] miss rbatis ident param!").to_token_stream();
    let rbatis_name = format!("{}", rbatis_ident);
    let sql_ident = args.get(1).expect("[rbatis] miss html file name param!").to_token_stream();

    let mut html = String::new();
    let mut file_name = sql_ident.to_string();
    if file_name.starts_with("\"") && file_name.ends_with("\"") {
        file_name = file_name[1..file_name.len() - 1].to_string();
    }
    let mut f = std::fs::File::open(&file_name).expect(&format!("File:\"{}\" does not exist", file_name));
    f.read_to_string(&mut html).expect(&format!("File:\"{}\" read fail!", file_name));
    let nodes = crate::macros::html_loader::load_html(&html).expect(&format!("[rbatis] load html file:{} fail!", sql_ident));
    let sql = find_node(&nodes, func_name_ident.to_string().as_str()).expect(&format!("[rbatis] load html file:{},method:{} fail!", sql_ident, sql_ident));


    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let fn_body = find_fn_body(target_fn);
    let is_async = target_fn.sig.asyncness.is_some();
    if !is_async {
        panic!(
            "[rbaits] #[crud_table] 'fn {}({})' must be  async fn! ",
            func_name_ident, func_args_stream
        );
    }
    //append all args
    let sql_args_gen = filter_args_context_id(&rbatis_name, &get_fn_args(target_fn));
    let is_fetch = is_fetch(&return_ty.to_string());
    let mut call_method = quote! {};
    if is_fetch {
        call_method = quote! {
             use rbatis::executor::{Executor,ExecutorMut};
             #rbatis_ident.fetch(&sql,&rb_args).await
        };
    } else {
        call_method = quote! {
             use rbatis::executor::{Executor,ExecutorMut};
             #rbatis_ident.exec(&sql,&rb_args).await
        };
    }
    if return_ty.to_string().contains("Page <")
        && func_args_stream.to_string().contains("& PageRequest")
    {
        let page_ident = get_page_req_ident(target_fn, &func_name_ident.to_string());
        call_method = quote! {
            use rbatis::crud::{CRUD,CRUDMut};
            #rbatis_ident.fetch_page(&sql,&rb_args,#page_ident).await
        };
        println!("gen return");
    }

    //gen rust code templete
    return quote! {
       pub async fn #func_name_ident(#func_args_stream) -> #return_ty {
         let mut rb_arg_map = serde_json::Map::new();
         #sql_args_gen
         #fn_body

         use rbatis::executor::{RbatisRef};
         let driver_type = #rbatis_ident.get_rbatis().driver_type()?;
         use rbatis::rbatis_sql;
         match driver_type{
            rbatis::DriverType::Postgres => {
                #[rb_html(#sql_ident,'$')]
                pub fn #func_name_ident(arg: &mut serde_json::Value) {}
                let (mut sql,rb_args) = #func_name_ident(&mut serde_json::Value::Object(rb_arg_map));
                #call_method
            }
            rbatis::DriverType::Mssql => {
                #[rb_html(#sql_ident,'$')]
                pub fn #func_name_ident(arg: &mut serde_json::Value) {}
                let (mut sql,rb_args) = #func_name_ident(&mut serde_json::Value::Object(rb_arg_map));
                sql = sql.replace("$","@p");
                #call_method
            }
            _=> {
                #[rb_html(#sql_ident,'?')]
                pub fn #func_name_ident(arg: &mut serde_json::Value) {}
                let (mut sql,rb_args) = #func_name_ident(&mut serde_json::Value::Object(rb_arg_map));
                #call_method
            }
         }
       }
    }
        .into();
}

fn filter_args_context_id(
    rbatis_name: &str,
    fn_arg_name_vec: &Vec<String>,
) -> proc_macro2::TokenStream {
    let mut sql_args_gen = quote! {};
    for item in fn_arg_name_vec {
        let item_ident = Ident::new(&item, Span::call_site());
        let item_ident_name = item_ident.to_string();
        if item.eq(&rbatis_name) {
            continue;
        }
        sql_args_gen = quote! {
             #sql_args_gen
             rb_arg_map.insert(#item_ident_name.to_string(),serde_json::json!(#item_ident));
        };
    }
    sql_args_gen
}
