extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use rbatis_macro::RbatisMacro;

#[proc_macro_derive(RbatisMacro)]
pub fn rbatis_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast:syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
  //  impl_hello_macro(&ast)
    let name = &ast.ident;
    let gen = quote! {
        impl RbatisMacro for #name {
            fn decode_name() -> &'static str {
                 //println!("Hello, Macro! My name is {}", stringify!(#name));
                 return stringify!(#name);
            }
        }
    };
    gen.into()
}