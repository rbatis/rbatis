use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, ItemFn, Token};

pub struct ParseArgs {
    pub sqls: Vec<syn::Ident>,
}

impl Parse for ParseArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let r = Punctuated::<syn::Ident, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            sqls: r.into_iter().collect(),
        })
    }
}

pub fn snake_name(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let mut struct_name = "".to_string();
    for x in args.sqls {
        struct_name += x.to_string().as_str();
    }
    struct_name = to_snake_name(&struct_name);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let func_name_ident = target_fn.sig.ident.to_token_stream();
    let stream = quote!(
        pub fn #func_name_ident() -> String {
             #struct_name.to_string()
        }
    );
    stream.into()
}

fn to_snake_name(name: &str) -> String {
    let len = name.len();
    let bytes = name.as_bytes();
    let mut new_name = String::with_capacity(name.len());
    let mut index = 0;
    for x in bytes {
        let c = *x as char;
        if c.is_ascii_uppercase() {
            if index != 0 && (index + 1) != len {
                new_name.push('_');
            }
            new_name.push(c.to_ascii_lowercase() as char);
        } else {
            new_name.push(c);
        }
        index += 1;
    }
    return new_name;
}

#[cfg(test)]
mod test {
    use crate::macros::snake_name::to_snake_name;

    #[test]
    fn test_to_snake_name() {
        assert_eq!("abc_def", to_snake_name("AbcDeF"));
    }
}
