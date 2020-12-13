use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::ext::IdentExt;

use crate::proc_macro::TokenStream;

///impl CRUDEnable
pub(crate) fn impl_crud_driver(ast: &syn::DeriveInput, arg_id_type: &str, arg_id_name: &str, arg_table_name: &str, arg_table_columns: &str, arg_column_format: &str) -> TokenStream {
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
        let new_fields = gen_fields(&ast.data);
        fields = quote! {#new_fields.to_string()};
    } else {
        fields = quote! {#arg_table_columns.to_string()};
    }
    let mut formats = quote! {
       let mut m = std::collections::HashMap::new();
    };
    if arg_column_format.is_empty() {
        formats = quote! {
          return std::collections::HashMap::new();
        }
    } else {
        let items: Vec<&str> = arg_column_format.split(",").collect();
        for item in items {
            if !item.contains(":") {
                panic!("[rbatis] crud_enable[column_format: format_str ] format_str must be column:format_value ")
            }
            let index = item.find(":").unwrap();
            let column = item[0..index].to_string();
            let format_str = item[index + 1..item.len()].to_string();
            formats = quote! {
               #formats
               m.insert(#column.to_string(),#format_str.to_string());
            };
        }
        formats = quote! { #formats  return m; };
    }
    let gen = quote! {
        impl rbatis::crud::CRUDEnable for #name {
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

            fn format_chain() -> std::collections::HashMap<String, String> {
                 #formats
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
    let mut fields = String::new();
    match &data {
        syn::Data::Struct(s) => {
            let mut index = 0;
            for field in &s.fields {
                let field_name = &field.ident.as_ref().map(|ele| ele.unraw()).to_token_stream().to_string();
                if index == 0 {
                    fields = fields + field_name
                } else {
                    fields = fields + "," + field_name
                }
                index += 1;
            }
        }
        _ => {
            panic!("[rbatis] only support struct for crud_enable's macro!")
        }
    }
    fields.to_token_stream()
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
    pub column_format: String,
}

/// impl the crud macro
pub(crate) fn impl_crud(args: TokenStream, input: TokenStream) -> TokenStream {
    let arg_str = args.to_string();
    let config = read_config(&arg_str);
    let token_string = input.to_string();
    let input_clone: proc_macro2::TokenStream = input.clone().into();
    let driver_token = gen_driver_token(&token_string);
    let ast = syn::parse(input).unwrap();
    let stream = impl_crud_driver(&ast, &config.id_type, &config.id_name, &config.table_name, &config.table_columns, &config.column_format);
    let s: proc_macro2::TokenStream = stream.into();
    let qt = quote! {
       #driver_token
       #input_clone
       #s
    };
    if !cfg!(feature = "no_print") {
        println!("............gen impl crud_enable:\n {}", qt);
        println!("............gen impl crud_enable end............");
    }
    qt.into()
}

fn gen_driver_token(token_string: &str) -> proc_macro2::TokenStream {
    let have_ser_driver_macro = token_string.contains("Serialize") && token_string.contains("Deserialize");
    let driver_token;
    if have_ser_driver_macro {
        driver_token = quote! {}
    } else {
        driver_token = quote! {
           #[derive(serde::Serialize, serde::Deserialize)]
        }
    }
    return driver_token;
}

///read config
///     id_name:id|
///     id_type:String|
///     table_name:biz_activity|
///     table_columns:id,name,version,delete_flag|
///     column_format:id:{}::uuid
fn read_config(arg: &str) -> CrudEnableConfig {
    let keys: Vec<&str> = arg.split("|").collect();
    let mut map = HashMap::new();
    for item in keys {
        let item = item.trim().replace("\n", "");
        if item.is_empty() {
            continue;
        }
        if !item.contains(":") {
            panic!("[rbaits] crud_enable must be key:value");
        }
        let index = item.find(":").unwrap();
        let key = item[0..index].replace(" ", "").to_string();
        let value = item[index + 1..item.len()].replace(" ", "").to_string();
        map.insert(key, value);
    }
    return CrudEnableConfig {
        id_name: map.get("id_name").unwrap_or(&"".to_string()).to_string(),
        id_type: map.get("id_type").unwrap_or(&"".to_string()).to_string(),
        table_name: map.get("table_name").unwrap_or(&"".to_string()).to_string(),
        table_columns: map.get("table_columns").unwrap_or(&"".to_string()).to_string(),
        column_format: map.get("column_format").unwrap_or(&"".to_string()).to_string(),
    };
}