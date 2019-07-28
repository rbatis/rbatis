extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast:syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
  //  impl_hello_macro(&ast)
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn decode_name() -> &'static str {
                 //println!("Hello, Macro! My name is {}", stringify!(#name));
                 return stringify!(#name);
            }
        }
    };
    gen.into()
}