use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::ext::IdentExt;

use crate::proc_macro::TokenStream;
use syn::Type;

///impl CRUDEnable
pub(crate) fn impl_crud_driver(
    ast: &syn::DeriveInput,
    arg_id_type: &str,
    arg_id_name: &str,
    arg_table_name: &str,
    arg_table_columns: &str,
    arg_formats: &HashMap<String, String>,
) -> TokenStream {
    let mut arg_id_name = arg_id_name.to_owned();
    if arg_id_name.is_empty() {
        arg_id_name = "id".to_string();
    }
    let name = &ast.ident;
    let id_type_inner;
    let id_type_source;
    match find_id_type_ident(&arg_id_name, &ast.data) {
        Some((id_type_ident, id_type_ident_source)) => {
            id_type_inner = id_type_ident;
            id_type_source = id_type_ident_source;
        }
        None => {
            panic!("[rbatis] can not find ident:{}", arg_id_name);
        }
    }
    // make return Option<&Self::IdType>
    let id_field_str = Ident::new(
        &arg_id_name
            .to_string()
            .trim_start_matches("\"")
            .trim_end_matches("\""),
        Span::call_site(),
    );
    let mut id_field = id_field_str.to_token_stream();
    if id_type_source.to_string().contains("Option") {
        id_field = quote! {
          self.#id_field.as_ref()
        };
    } else {
        id_field = quote! {
           Option::<&#id_type_source>::from(&self.#id_field)
        };
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
    let mut formats_mysql = proc_macro2::TokenStream::new();
    let mut formats_pg = proc_macro2::TokenStream::new();
    let mut formats_sqlite = proc_macro2::TokenStream::new();
    let mut formats_mssql = proc_macro2::TokenStream::new();
    if !arg_formats.is_empty() {
        for (k, v) in arg_formats {
            match k.as_str() {
                "formats_mysql" => {
                    formats_mysql = gen_format(v);
                }
                "formats_pg" | "formats_postgres" => {
                    formats_pg = gen_format(v);
                }
                "formats_sqlite" => {
                    formats_sqlite = gen_format(v);
                }
                "formats_mssql" => {
                    formats_mssql = gen_format(v);
                }
                _ => {}
            }
        }
    }
    let gen = quote! {
        impl rbatis::crud::CRUDEnable for #name {
            type IdType = #id_type_inner;

            fn id_name() -> String {
                 #arg_id_name.to_string()
            }

            fn get_id(&self) ->  Option<&Self::IdType>{
                #id_field
            }

            fn table_name() -> String {
                 #table_name.to_string()
            }

            fn table_columns() -> String{
                 #fields
            }

            fn formats(driver_type: &rbatis::core::db::DriverType) -> std::collections::HashMap<String, fn(arg:&str)->String> {
                  let mut m = std::collections::HashMap::new();
                  match driver_type{
                    rbatis::core::db::DriverType::Mysql=>{
                         #formats_mysql
                         return m;
                    },
                    rbatis::core::db::DriverType::Postgres=>{
                         #formats_pg
                         return m;
                    },
                    rbatis::core::db::DriverType::Sqlite=>{
                         #formats_sqlite
                         return m;
                    },
                    rbatis::core::db::DriverType::Mssql=>{
                         #formats_mssql
                         return m;
                    },
                    rbatis::core::db::DriverType::None=>{
                         return m;
                    },
                 }
            }
        }
    };
    gen.into()
}

fn gen_format(v: &str) -> proc_macro2::TokenStream {
    let mut formats = quote! {};
    let items: Vec<&str> = v.split(",").collect();
    for item in items {
        if !item.contains(":") {
            panic!(format!("[rbatis] [crud_enable] format_str:'{}' must be [column]:[format_string],for example ->  '{}'  ", item, "formats_pg:id:{}::uuid"));
        }
        if !item.contains("{}") {
            panic!(format!("[rbatis] [crud_enable] format_str:'{}' must be [column]:[format_string],for example ->  '{}'  ", item, "formats_pg:id:{}::uuid"));
        }
        let index = item.find(":").unwrap();
        let column = item[0..index].to_string();
        let mut format_str = item[index + 1..item.len()].to_string();

        let formats_data = find_format_string(&format_str);
        let mut args_quote = quote! {};
        if formats_data.is_empty() {
            args_quote = quote! {arg};
        } else {
            let mut index = 0;
            for (inner, data) in formats_data {
                if index == 0 {
                    args_quote = quote! {arg};
                } else {
                    args_quote = quote! {#args_quote,arg};
                }
                index += 1;
            }
        }
        //"id:{}::uuid"  ,  "id:{}"
        let mut format_func = quote! {
              |arg:&str| -> String {
                  format!(#format_str,#args_quote)
              }
        };
        println!("format_func:{}", format_func);
        formats = quote! {
           #formats
           m.insert(#column.to_string(),#format_func);
        };
    }
    return formats;
}

//find like {*},{*} value *
fn find_format_string(arg: &str) -> Vec<(String, String)> {
    let mut list = Vec::new();
    let chars: Vec<u8> = arg.bytes().collect();
    let mut item = String::with_capacity(arg.len());
    let mut index: i32 = -1;
    for v in &chars {
        index = index + 1;
        if !item.is_empty() {
            item.push(*v as char);
            if *v == '}' as u8 {
                let key = item[1..item.len() - 1].to_string();
                list.push((key, item.clone()));
                item.clear();
            }
            continue;
        }
        if *v == '{' as u8 {
            item.push(*v as char);
        }
    }
    return list;
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
                let field_name = &field
                    .ident
                    .as_ref()
                    .map(|ele| ele.unraw())
                    .to_token_stream()
                    .to_string();
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
fn find_id_type_ident(id_name: &str, arg: &syn::Data) -> Option<(Ident, proc_macro2::TokenStream)> {
    match &arg {
        syn::Data::Struct(ref data_struct) => match data_struct.fields {
            // field: (0) a: String
            syn::Fields::Named(ref fields_named) => {
                for (_, field) in fields_named.named.iter().enumerate() {
                    let field_name = format!("{}", field.ident.to_token_stream());
                    if field_name.trim().eq(id_name) {
                        let ty = format!("{}", field.ty.to_token_stream());
                        let mut inner_type = ty.trim().replace(" ", "").to_string();
                        if inner_type.starts_with("Option<") {
                            inner_type = inner_type
                                .trim_start_matches("Option<")
                                .trim_end_matches(">")
                                .to_string();
                        }
                        let id_type = field.ty.to_token_stream();
                        //println!("id_type from:{}", &inner_type);
                        let id_type_ident = Ident::new(inner_type.as_str(), Span::call_site());
                        //println!("id_type:{}", &id_type);
                        return Some((id_type_ident, id_type));
                    }
                }
            }
            syn::Fields::Unnamed(_) => {}
            syn::Fields::Unit => {}
        },
        _ => (),
    }
    None
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
    pub formats: HashMap<String, String>,
}

/// impl the crud macro
pub(crate) fn impl_crud(args: TokenStream, input: TokenStream) -> TokenStream {
    let arg_str = args.to_string();
    let config = read_config(&arg_str);
    let token_string = input.to_string();
    let input_clone: proc_macro2::TokenStream = input.clone().into();
    let driver_token = gen_driver_token(&token_string);
    let ast = syn::parse(input).unwrap();
    let stream = impl_crud_driver(
        &ast,
        &config.id_type,
        &config.id_name,
        &config.table_name,
        &config.table_columns,
        &config.formats,
    );
    let s: proc_macro2::TokenStream = stream.into();
    let qt = quote! {
       #driver_token
       #input_clone
       #s
    };
    qt.into()
}

fn gen_driver_token(token_string: &str) -> proc_macro2::TokenStream {
    let have_ser_driver_macro =
        token_string.contains("Serialize") && token_string.contains("Deserialize");
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
///     formats_pg:id:{}::uuid,name:{}::string
///
///
///     id_name:"id"|
///     id_type:"String"|
///     table_name:"biz_activity"|
///     table_columns:"id,name,version,delete_flag"|
///     formats_pg:"id:{}::uuid,name:{}::string"
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
        let mut value = item[index + 1..item.len()].replace(" ", "").to_string();
        if value.len() >= 2 && value.starts_with("\"") && value.ends_with("\"") {
            value = value[1..value.len() - 1].to_string();
        }
        map.insert(key, value);
    }
    let mut formats = HashMap::new();
    for (k, v) in &map {
        if k.starts_with("formats") {
            if !(k.ends_with("formats_pg")
                || k.ends_with("formats_postgres")
                || k.ends_with("formats_mysql")
                || k.ends_with("formats_sqlite")
                || k.ends_with("formats_mssql"))
            {
                panic!("[rbatis] formats must be formats_pg, formats_mysql,formats_sqlite,formats_mssql!");
            }
            formats.insert(k.to_owned(), v.to_owned());
        }
    }
    return CrudEnableConfig {
        id_name: map.get("id_name").unwrap_or(&String::new()).to_string(),
        id_type: map.get("id_type").unwrap_or(&String::new()).to_string(),
        table_name: map.get("table_name").unwrap_or(&String::new()).to_string(),
        table_columns: map
            .get("table_columns")
            .unwrap_or(&String::new())
            .to_string(),
        formats: formats,
    };
}
