use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, ItemFn};

use crate::proc_macro::TokenStream;
use crate::util::{find_fn_body, find_return_type, get_fn_args, get_page_req_ident, is_fetch_sql};

///py_sql macro
///support args for RB:&Rbatis,page:&PageRequest
///support return for Page<*>
pub(crate) fn impl_macro_py_sql(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let return_ty = find_return_type(target_fn);
    let func_name_ident = target_fn.sig.ident.to_token_stream();
    let rbatis_ident = args.get(0).unwrap().to_token_stream();
    let rbatis_name = format!("{}", rbatis_ident);
    let sql_ident = args.get(1).unwrap().to_token_stream();
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
    let is_fetch = is_fetch_sql(&sql);
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


    let mut deal_with = quote! {};
    let mut scheme_ident = "?".to_token_stream();
    if args.len() >= 3 {
        let new_scheme_ident = args.get(2).unwrap().to_token_stream();
        match new_scheme_ident.to_string().as_str() {
            "pg" | "postgres" | "$" => {
                scheme_ident = "$".to_token_stream();
            }
            "mssql" | "sqlserver" | "@p" | "@" => {
                scheme_ident = "$".to_token_stream();
                deal_with = quote! {
                    sql = sql.replace("$","@p");
                };
            }
            //mssql,mysql,sqlite....
            _ => {}
        }
    }
    //gen rust code templete
    return quote! {
       pub async fn #func_name_ident(#func_args_stream) -> #return_ty {
         let mut sql = #sql_ident.to_string();
         let mut rb_arg_map = serde_json::Map::new();
         #sql_args_gen
         #fn_body
         // use rbatis::py::{PySqlConvert};
         // let (sql, rb_args) = #rbatis_ident.py_to_sql(&#sql_ident, &rb_arg_map)?;
         use rbatis::rbatis_sql;
         #[rb_py(#sql_ident,#scheme_ident)]
         pub fn #func_name_ident(arg: &serde_json::Value) {}
         let (mut sql,rb_args) = #func_name_ident(&serde_json::Value::Object(rb_arg_map));
         #deal_with
         #call_method
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
