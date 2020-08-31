extern crate proc_macro;


use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;

use crate::proc_macro::TokenStream;


#[proc_macro_derive(CRUDEnable)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_macro(&ast)
}

fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;


    let mut id_type = Ident::new("String", Span::call_site());
    match &ast.data {
        syn::Data::Struct(ref data_struct) => match data_struct.fields {
            // field: (0) a: String
            syn::Fields::Named(ref fields_named) => {
                for (_, field) in fields_named.named.iter().enumerate() {
                    let field_name = format!("{}", field.ident.to_token_stream());
                    let ty = format!("{}", field.ty.to_token_stream());
                    //println!("named struct field: ({}) {}: {}", index, field_name, field.ty.to_token_stream());
                    if field_name.eq("id") {
                        let mut inner_type = ty.trim().replace(" ","").to_string();
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
            //// field: (0) : String
            // syn::Fields::Unnamed(ref fields_unnamed) => {
            //     for (index, field) in fields_unnamed.unnamed.iter().enumerate() {
            //         println!("unnamed struct field: ({}): {}", index, field.ty.to_token_stream())
            //      }
            // }
            // syn::Fields::Unit => {
            //      println!("unit struct field: None")
            // }
            syn::Fields::Unnamed(_) => {

            }
            syn::Fields::Unit => {

            }

        },
        _ => (),
    }


    let gen = quote! {

    fn snake_name(name: &String) -> String {
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
        impl CRUDEnable for #name {
            //识别的表id字段类型
            type IdType = #id_type;

            //识别的表名
            fn table_name() -> String {
                 let mut name = stringify!(#name).to_string();
                 let names: Vec<&str> = name.split("::").collect();
                 name = names.get(names.len() - 1).unwrap().to_string();
                 return snake_name(&name);
            }
        }
    };
    gen.into()
}

