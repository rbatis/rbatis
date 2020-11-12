use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::ext::IdentExt;

use crate::proc_macro::TokenStream;
use std::collections::HashMap;

///impl CRUDEnable
pub(crate) fn impl_crud_driver(ast: &syn::DeriveInput, arg_id_type: &str, arg_id_name: &str, arg_table_name: &str, arg_table_columns: &str) -> TokenStream {
    let name = &ast.ident;
    let id_name;
    if arg_id_name.is_empty() {
        id_name = "id".to_token_stream();
    } else {
        id_name = arg_id_name.to_token_stream();
    }
    let id_type;
    if arg_id_type.is_empty() {
        id_type = find_id_type_ident(&ast.data).to_token_stream();
    } else {
        id_type = Ident::new(arg_id_type, Span::call_site()).to_token_stream();
    }
    let table_name;
    if arg_table_name.is_empty() {
        table_name = gen_table_name(&ast.ident).to_token_stream();
    } else {
        table_name = quote! {#arg_table_name};
    }
    let fields;
    if arg_table_columns.is_empty() {
        fields = gen_fields(&ast.data);
    } else {
        fields = quote! {#arg_table_columns.to_string()};
    }

    let gen = quote! {
        impl CRUDEnable for #name {
            type IdType = #id_type;

            fn id_name() -> String {
                 #id_name.to_string()
            }

            fn table_name() -> String {
                 #table_name.to_string()
            }

            fn table_columns() -> String{
                 #fields
            }
        }
    };
    gen.into()
}

fn gen_table_name(data: &syn::Ident) -> String {
    let mut table_name = data.to_string();
    let names: Vec<&str> = table_name.split("::").collect();
    table_name = names.get(names.len() - 1).unwrap().to_string();
    table_name = to_snake_name(&table_name);
    table_name
}

fn gen_fields(data: &syn::Data) -> proc_macro2::TokenStream {
    let mut fields = quote! { String::new() };
    match &data {
        syn::Data::Struct(s) => {
            let mut index = 0;
            for field in &s.fields {
                let field_name = &field.ident.as_ref().map(|ele| ele.unraw()).to_token_stream().to_string();
                if index == 0 {
                    fields = quote! {
                       #fields+#field_name
                     };
                } else {
                    fields = quote! {
                       #fields+","+#field_name
                     };
                }
                index += 1;
            }
        }
        _ => {}
    }
    fields
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

fn to_snake_name(name: &String) -> String {
    let chs = name.chars();
    let mut new_name = String::new();
    let mut index = 0;
    let chs_len = name.len();
    for x in chs {
        if x.is_uppercase() {
            if index != 0 && (index + 1) != chs_len {
                new_name.push_str("_");
            }
            new_name.push_str(x.to_lowercase().to_string().as_str());
        } else {
            new_name.push(x);
        }
        index += 1;
    }
    return new_name;
}

#[derive(Debug)]
pub struct CrudEnableConfig {
    pub id_name: String,
    pub id_type: String,
    pub table_name: String,
    pub table_columns: String,
}

/// impl the crud macro
pub(crate) fn impl_crud(args: TokenStream, input: TokenStream) -> TokenStream {
    let arg_str = args.to_string();
    let config = read_config(&arg_str);
    let input_clone: proc_macro2::TokenStream = input.clone().into();
    let ast = syn::parse(input).unwrap();
    let stream = impl_crud_driver(&ast, &config.id_type, &config.id_name, &config.table_name, &config.table_columns);
    let s: proc_macro2::TokenStream = stream.into();
    let qt = quote! {
       #input_clone

       #s
    };
    if !cfg!(feature = "no_print") {
        println!("............gen impl crud_enable:\n {}", qt);
        println!("............gen impl crud_enable end............");
    }
    qt.into()
}

///read config
///     id_name:id|
///     id_type:String|
///     table_name:biz_activity|
///     table_columns:id,name,version,delete_flag
fn read_config(arg: &str) -> CrudEnableConfig {
    let keys: Vec<&str> = arg.split("|").collect();
    let mut map = HashMap::new();
    for item in keys {
        let item = item.trim().replace("\n", "");
        if item.is_empty(){
            continue;
        }
        let kvs: Vec<&str> = item.split(":").collect();
        if kvs.len() != 2usize {
            panic!("[rbaits] crud_enable must be key:\"value\"");
        }
        let key = kvs[0].trim();
        let value = kvs[1].trim();
        map.insert(key.to_string(), value.to_string());
    }
    return CrudEnableConfig {
        id_name: map.get("id_name").unwrap_or(&"".to_string()).to_string(),
        id_type: map.get("id_type").unwrap_or(&"".to_string()).to_string(),
        table_name: map.get("table_name").unwrap_or(&"".to_string()).to_string(),
        table_columns: map.get("table_columns").unwrap_or(&"".to_string()).to_string(),
    };
}