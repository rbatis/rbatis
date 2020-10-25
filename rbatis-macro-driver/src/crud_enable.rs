use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::ext::IdentExt;

use crate::proc_macro::TokenStream;

///impl CRUDEnable
pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let table_name = gen_table_name(&ast.ident);
    let id_type = find_id_type_ident(&ast.data);
    let fields = gen_fields(&ast.data);
    let gen = quote! {
        impl CRUDEnable for #name {
            type IdType = #id_type;

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

fn gen_table_name(data:&syn::Ident)->String{
    let mut table_name = data.to_string();
    let names: Vec<&str> = table_name.split("::").collect();
    table_name = names.get(names.len() - 1).unwrap().to_string();
    table_name = to_snake_name(&table_name);
    table_name
}

fn gen_fields(data:&syn::Data) -> proc_macro2::TokenStream {
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