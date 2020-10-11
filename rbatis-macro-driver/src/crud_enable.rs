use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, Data, FnArg, ItemFn, parse_macro_input, ReturnType};

use crate::proc_macro::TokenStream;

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


pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
    println!("............proc_macro_struct sql start............");
    let name = &ast.ident;
    let id_type = find_id_type_ident(&ast.data);
    /// gen fields token
    let mut fields = quote! { String::new() };
    match &ast.data {
        syn::Data::Struct(s) => {
            let mut index = 0;
            for field in &s.fields {
                let field_name = &field.ident.to_token_stream().to_string();
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

    let gen = quote! {
        impl CRUDEnable for #name {
            type IdType = #id_type;

            fn table_name() -> String {
                 let mut name = stringify!(#name).to_string();
                 let names: Vec<&str> = name.split("::").collect();
                 name = names.get(names.len() - 1).unwrap().to_string();
                 return rbatis::utils::string_util::to_snake_name(&name);
            }

            fn table_columns() -> String{
                 #fields
            }
        }
    };
    println!("............gen rust code:\n {}", format!("{}", gen));
    println!("............proc_macro_struct sql end............");
    gen.into()
}