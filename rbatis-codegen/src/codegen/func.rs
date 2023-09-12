use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{BinOp, Expr, Lit, Member};

fn token_steam_string(arg: proc_macro2::TokenStream) -> String {
    arg.to_token_stream().to_string().trim().to_string()
}

fn convert_to_arg_access(context: &str, arg: Expr, ignore: &[String]) -> Expr {
    match arg {
        Expr::Path(b) => {
            let token = b.to_token_stream().to_string();
            if token == "null" {
                return syn::parse_str::<Expr>("rbs::Value::Null").expect("codegen_func fail");
            }
            if token == "sql" {
                return Expr::Path(b);
            }
            let param = token_steam_string(b.to_token_stream());
            let mut fetch_from_arg = true;
            for x in ignore {
                if param.eq(x) {
                    fetch_from_arg = false;
                }
            }
            if fetch_from_arg {
                return syn::parse_str::<Expr>(&format!("&arg[\"{}\"]", param))
                    .expect("codegen_func fail");
            } else {
                return syn::parse_str::<Expr>(&format!("{}", param)).expect("codegen_func fail");
            }
        }
        Expr::MethodCall(mut b) => {
            //receiver is named need to convert to arg["xxx"]
            b.receiver = Box::new(convert_to_arg_access(context, *b.receiver.clone(), ignore));
            return Expr::MethodCall(b);
        }
        Expr::Binary(mut b) => {
            b.left = Box::new(convert_to_arg_access(context, *b.left.clone(), ignore));
            b.right = Box::new(convert_to_arg_access(context, *b.right.clone(), ignore));
            match b.op {
                BinOp::Add(_) => {
                    let left_token = b.left.to_token_stream().to_string();
                    if left_token.trim().ends_with("\"") && left_token.trim().starts_with("\"") {
                        return syn::parse_str::<Expr>(&format!(
                            "(String::from({})).op_add({})",
                            b.left.to_token_stream(),
                            b.right.to_token_stream()
                        ))
                        .expect("codegen_func fail");
                    } else {
                        return syn::parse_str::<Expr>(&format!(
                            "({}).op_add(&{})",
                            b.left.to_token_stream(),
                            b.right.to_token_stream()
                        ))
                        .expect("codegen_func fail");
                    }
                }
                BinOp::And(_) => {
                    b.left = Box::new(
                        syn::parse_str::<Expr>(&format!(
                            "bool::op_from({})",
                            b.left.to_token_stream().to_string().trim()
                        ))
                        .expect("codegen_func fail"),
                    );
                    b.right = Box::new(
                        syn::parse_str::<Expr>(&format!(
                            "bool::op_from({})",
                            b.right.to_token_stream().to_string().trim()
                        ))
                        .expect("codegen_func fail"),
                    );
                }
                BinOp::Or(_) => {
                    b.left = Box::new(
                        syn::parse_str::<Expr>(&format!(
                            "bool::op_from({})",
                            b.left.to_token_stream().to_string().trim()
                        ))
                        .expect("codegen_func fail"),
                    );
                    b.right = Box::new(
                        syn::parse_str::<Expr>(&format!(
                            "bool::op_from({})",
                            b.right.to_token_stream().to_string().trim()
                        ))
                        .expect("codegen_func fail"),
                    );
                }

                // The `-` operator (subtraction)
                BinOp::Sub(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_sub(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `*` operator (multiplication)
                BinOp::Mul(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_mul(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `/` operator (division)
                BinOp::Div(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_div(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `%` operator (modulus)
                BinOp::Rem(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_rem(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `&` operator (bitwise and)
                BinOp::BitAnd(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_bitand(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `|` operator (bitwise or)
                BinOp::BitOr(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_bitor(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `==` operator (equality)
                BinOp::Eq(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_eq(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `<` operator (less than)
                BinOp::Lt(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_lt(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `<=` operator (less than or equal to)
                BinOp::Le(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_le(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `!=` operator (not equal to)
                BinOp::Ne(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_ne(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `>=` operator (greater than or equal to)
                BinOp::Ge(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_ge(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `>` operator (greater than)
                BinOp::Gt(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_gt(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `^` operator (bitwise xor)
                BinOp::BitXor(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_bitxor(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                // The `<<` operator (shift left)
                _ => {
                    panic!("unsupported token {}", b.op.to_token_stream())
                }
            }
            return Expr::Binary(b);
        }
        Expr::Unary(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr, ignore));
            if b.op.to_token_stream().to_string().trim() == "-" {
                return syn::parse_str::<Expr>(&format!(
                    " (0.op_sub(&{}))",
                    b.expr.to_token_stream().to_string().trim()
                ))
                .expect("codegen_func fail");
            }
            return Expr::Unary(b);
        }
        Expr::Paren(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr, ignore));
            return Expr::Paren(b);
        }
        Expr::Field(mut b) => {
            b.base = Box::new(convert_to_arg_access(context, *b.base.clone(), ignore));
            match b.member {
                Member::Named(named) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}[\"{}\"])",
                        b.base.to_token_stream(),
                        named.to_token_stream()
                    ))
                    .expect("codegen_func fail");
                }
                Member::Unnamed(_) => {}
            }
            return Expr::Field(b);
        }
        Expr::Reference(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr, ignore));
            let result = Expr::Reference(b);
            return result;
        }
        Expr::Index(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr, ignore));
            return syn::parse_str::<Expr>(&format!(
                "{}[{}]",
                b.expr.to_token_stream(),
                b.index.to_token_stream()
            ))
            .expect("codegen_func fail");
        }
        Expr::Let(mut let_expr) => {
            let_expr.expr = Box::new(convert_to_arg_access(context, *let_expr.expr, ignore));
            return Expr::Let(let_expr);
        }
        Expr::Lit(b) => {
            match b.lit.clone() {
                Lit::Str(_) => {}
                Lit::ByteStr(_) => {}
                Lit::Byte(_) => {}
                Lit::Char(_) => {}
                Lit::Int(i) => {
                    //cast int to i64
                    return syn::parse_str::<Expr>(&format!("{}i64", i))
                        .expect("codegen_func fail");
                }
                Lit::Float(f) => {
                    //cast int to f64
                    return syn::parse_str::<Expr>(&format!("{}f64", f))
                        .expect("codegen_func fail");
                }
                Lit::Bool(_) => {}
                Lit::Verbatim(_) => {}
                _ => {}
            }
            return Expr::Lit(b);
        }
        _ => {
            return arg;
        }
    }
}

/// gen method or body(if func_name_ident is empty)
pub fn impl_fn(
    context: &str,
    func_name_ident: &str,
    args: &str,
    serialize_result: bool,
    ignore: &[String],
) -> proc_macro2::TokenStream {
    let mut string_data = args.to_string();
    string_data = string_data[1..string_data.len() - 1].to_string();
    string_data = string_data.replace(".string()", ".to_string()");
    //convert string define
    let mut last_char = '_';
    let mut string_data_new = String::new();
    for x in string_data.chars() {
        if x == '\'' && last_char != '\\' {
            string_data_new.push('\"');
        } else {
            string_data_new.push(x);
        }
        last_char = x;
    }
    string_data = string_data_new;
    let t = syn::parse_str::<Expr>(&string_data);
    if t.is_err() {
        panic!(
            "[rbatis-codegen]syn::parse_str: {} fail: {}",
            args,
            t.err().expect("codegen_func fail").to_string()
        )
    }
    let mut t = t.expect("codegen_func fail");
    t = convert_to_arg_access(context, t, ignore);
    string_data = t.to_token_stream().to_string();
    string_data = string_data.replace(" . ", ".");
    let t = syn::parse_str::<Expr>(&string_data);
    if t.is_err() {
        panic!(
            "[rbatis-codegen]syn::parse_str: {} fail: {}",
            args,
            t.err().expect("codegen_func fail").to_string()
        )
    }
    let t = t.expect("codegen_func fail");
    let mut result_impl = quote! { {#t} };
    if serialize_result {
        result_impl = quote! {rbs::to_value({#t}).unwrap_or_default()};
    }
    if func_name_ident.is_empty() || func_name_ident.eq("\"\"") {
        return quote! {
               #result_impl
        }
        .to_token_stream();
    } else {
        let func_name_ident = Ident::new(&func_name_ident.to_string(), Span::call_site());
        return quote! {
            pub fn #func_name_ident(arg:&rbs::Value) -> rbs::Value {
               use rbatis_codegen::ops::*;
               #result_impl
            }
        }
        .to_token_stream();
    }
}
