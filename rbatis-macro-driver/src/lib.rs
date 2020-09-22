extern crate proc_macro;

use proc_macro2::{Ident, Span};
use quote::{quote, TokenStreamExt};
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, BareFnArg, Expr, FnArg, ForeignItemFn, ItemFn, NestedMeta, parse_macro_input, TypeBareFn, ReturnType, Type, Path};

use crate::proc_macro::TokenStream;
use std::process::Output;
use syn::parse::Parse;
use proc_macro2::TokenTree::Punct;

mod string_util;

#[proc_macro_derive(CRUDEnable)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_macro(&ast)
}

///filter id_type
fn find_id_type_ident(arg: &syn::Data) -> Ident {
    let mut id_type = Ident::new("String", Span::call_site());
    match &arg {
        syn::Data::Struct(ref data_struct) => match data_struct.fields {
            // field: (0) a: String
            syn::Fields::Named(ref fields_named) => {
                for (_, field) in fields_named.named.iter().enumerate() {
                    //println!("named struct field: ({}) {}: {}", index, field_name, field.ty.to_token_stream());
                    let field_name = format!("{}", field.ident.to_token_stream());
                    if field_name.eq("id") {
                        let ty = format!("{}", field.ty.to_token_stream());
                        let mut inner_type = ty.trim().replace(" ", "").to_string();
                        if inner_type.starts_with("Option<") {
                            inner_type = inner_type.trim_start_matches("Option<").trim_end_matches(">").to_string();
                        }
                        //println!("id_type from:{}", &inner_type);
                        id_type = Ident::new(inner_type.as_str(), Span::call_site());
                        //println!("id_type:{}", &id_type);
                        break;
                    }
                }
            }
            syn::Fields::Unnamed(_) => {}
            syn::Fields::Unit => {}
        },
        _ => (),
    }
    id_type
}


fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let id_type = find_id_type_ident(&ast.data);
    let gen = quote! {
        impl CRUDEnable for #name {
            type IdType = #id_type;
            fn table_name() -> String {
                 let mut name = stringify!(#name).to_string();
                 let names: Vec<&str> = name.split("::").collect();
                 name = names.get(names.len() - 1).unwrap().to_string();
                 return rbatis::utils::string_util::to_snake_name(&name);
            }
        }
    };
    gen.into()
}


/// sql create macro
#[proc_macro_attribute]
pub fn sql(args: TokenStream, this: TokenStream) -> TokenStream {
    println!("proc_macro_attribute sql start............");

    // this
    let args = parse_macro_input!(args as AttributeArgs);
    let func = syn::parse(this).unwrap();

    let stream = impl_macro_sql(&func, &args);

    println!("gen rust code:\n {}", format!("{}", stream));
    println!("proc_macro_attribute sql end............");

    stream
}

macro_rules! gen_macro_json_arg_array {
    () => {};
}

fn impl_macro_sql(func: &syn::ItemFn, args: &AttributeArgs) -> TokenStream {
    let mut return_ty = func.sig.output.to_token_stream();
    match &func.sig.output{
        ReturnType::Type(_,b)=>{
            return_ty = b.to_token_stream();
        }
        _ => {}
    }

    println!("return_ty:{:#?}",return_ty);

    let mut s=format!("{}", return_ty);
    if !s.starts_with("rbatis_core :: Result") && !s.starts_with("Result") && !s.starts_with("std :: result :: Result"){
        return_ty = quote! {
             rbatis_core :: Result <#return_ty>
        };
    }

    let func_name = format!("{}", func.sig.ident.to_token_stream());
    let rbatis_meta = args.get(0).unwrap();
    let field_name = format!("{}", rbatis_meta.to_token_stream());

    let sql_meta = args.get(1).unwrap();
    let sql = format!("{}", sql_meta.to_token_stream()).trim().to_string();

    //fetch fn arg names
    let mut fn_arg_name_vec = vec![];
    for arg in &func.sig.inputs {
        match arg {
            FnArg::Typed(t) => {
                let arg_name = format!("{}", t.pat.to_token_stream());
                fn_arg_name_vec.push(arg_name);
                //println!("arg_name {}", arg_name);
            }
            _ => {}
        }
    }

    //check sql
    let arg_num = string_util::find_all_sql_opt(&sql);
    if arg_num != fn_arg_name_vec.len() {
        panic!("[rbatis] fn arg len must equal to the sql's arg len!  fn: {}", func_name);
    }

    let sql_ident = sql_meta;
    let func_args_stream = func.sig.inputs.to_token_stream();
    let func_name_ident = Ident::new(&func_name, Span::call_site());
    let rbatis_ident = Ident::new(&field_name, Span::call_site());
    //append all args
    let mut args_gen = quote! {
         let mut args =vec![];
    };
    for item in fn_arg_name_vec {
        let item_ident = Ident::new(&item, Span::call_site());
        args_gen = quote! {
            #args_gen
            args.push(serde_json::to_value(#item_ident).unwrap_or(serde_json::Value::Null));
       };
    }

    let is_select = sql.starts_with("select ") || sql.starts_with("SELECT ") || sql.starts_with("\"select ") || sql.starts_with("\"SELECT ");

    if is_select {
        let gen = quote! {
        pub async fn #func_name_ident(#func_args_stream) -> #return_ty {
           #args_gen
              log::info!("[rbatis] [{}] Query ==> {}", "", #sql_ident);
              log::info!("[rbatis] [{}] Args  ==> {}", "", serde_json::to_string(&args).unwrap_or("".to_string()));
              return #rbatis_ident.fetch_prepare("",#sql_ident,&args).await;
        }
    };
        return gen.into();
    } else {
        let gen = quote! {
        pub async fn #func_name_ident(#func_args_stream) #return_ty {
           #args_gen
              log::info!("[rbatis] [{}] Exec ==> {}", "", #sql_ident);
              log::info!("[rbatis] [{}] Args  ==> {}", "", serde_json::to_string(&args).unwrap_or("".to_string()));
              return #rbatis_ident.exec_prepare("",#sql_ident,&args).await;
        }
    };
        return gen.into();
    }
}