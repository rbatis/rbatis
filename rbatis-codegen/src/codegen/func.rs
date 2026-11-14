use crate::error::Error;
use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn::{BinOp, Expr, Lit, Member};

///translate like `#{a + b}` Expr to rust code Expr
#[allow(clippy::only_used_in_recursion)]
pub fn translate(context: &str, arg: Expr, ignore: &[String]) -> Result<Expr, Error> {
    match arg {
        Expr::Path(b) => {
            let token = b.to_token_stream().to_string();
            if token == "null" {
                return syn::parse_str::<Expr>("rbs::Value::Null").map_err(Error::from);
            }
            if token == "sql" {
                return Ok(Expr::Path(b));
            }
            let param = b.to_token_stream().to_string().trim().to_string();
            let mut fetch_from_arg = true;
            for x in ignore {
                if param.eq(x) {
                    fetch_from_arg = false;
                }
            }
            if fetch_from_arg {
                syn::parse_str::<Expr>(&format!("&arg[\"{}\"]", param)).map_err(Error::from)
            } else {
                syn::parse_str::<Expr>(&param.to_string()).map_err(Error::from)
            }
        }
        Expr::MethodCall(mut b) => {
            b.receiver = Box::new(translate(context, *b.receiver, ignore)?);
            // `Str` is the Value-based equivalent of the standard string methods.
            // Translate its arguments too so `name.contains_str(other)` reads `other`
            // from the template arguments instead of leaving an unresolved name.
            if crate::ops::takes_value_arguments(&b.method.to_string()) {
                for arg in b.args.iter_mut() {
                    let translated = translate(context, arg.clone(), ignore)?;
                    *arg = syn::parse_str::<Expr>(&format!(
                        "rbs::value({}).unwrap_or_default()",
                        translated.to_token_stream()
                    ))
                    .map_err(Error::from)?;
                }
            }
            Ok(Expr::MethodCall(b))
        }
        Expr::Binary(mut b) => {
            b.left = Box::new(translate(context, *b.left, ignore)?);
            b.right = Box::new(translate(context, *b.right, ignore)?);
            match b.op {
                BinOp::Add(_) => {
                    let left_token = b.left.to_token_stream().to_string();
                    if left_token.trim().ends_with("\"") && left_token.trim().starts_with("\"") {
                        return syn::parse_str::<Expr>(&format!(
                            "(String::from({})).op_add({})",
                            b.left.to_token_stream(),
                            b.right.to_token_stream()
                        ))
                        .map_err(Error::from);
                    } else {
                        return syn::parse_str::<Expr>(&format!(
                            "({}).op_add(&{})",
                            b.left.to_token_stream(),
                            b.right.to_token_stream()
                        ))
                        .map_err(Error::from);
                    }
                }
                BinOp::And(_) => {
                    b.left = Box::new(
                        syn::parse_str::<Expr>(&format!(
                            "bool::op_from({})",
                            b.left.to_token_stream().to_string().trim()
                        ))
                        .map_err(|e| Error::from(format!("codegen_func fail: {}", e)))?,
                    );
                    b.right = Box::new(
                        syn::parse_str::<Expr>(&format!(
                            "bool::op_from({})",
                            b.right.to_token_stream().to_string().trim()
                        ))
                        .map_err(|e| Error::from(format!("codegen_func fail: {}", e)))?,
                    );
                }
                BinOp::Or(_) => {
                    b.left = Box::new(
                        syn::parse_str::<Expr>(&format!(
                            "bool::op_from({})",
                            b.left.to_token_stream().to_string().trim()
                        ))
                        .map_err(|e| Error::from(format!("codegen_func fail: {}", e)))?,
                    );
                    b.right = Box::new(
                        syn::parse_str::<Expr>(&format!(
                            "bool::op_from({})",
                            b.right.to_token_stream().to_string().trim()
                        ))
                        .map_err(|e| Error::from(format!("codegen_func fail: {}", e)))?,
                    );
                }

                // The `-` operator (subtraction)
                BinOp::Sub(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_sub(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `*` operator (multiplication)
                BinOp::Mul(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_mul(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `/` operator (division)
                BinOp::Div(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_div(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `%` operator (modulus)
                BinOp::Rem(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_rem(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `&` operator (bitwise and)
                BinOp::BitAnd(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_bitand(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `|` operator (bitwise or)
                BinOp::BitOr(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_bitor(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `==` operator (equality)
                BinOp::Eq(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_eq(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `<` operator (less than)
                BinOp::Lt(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_lt(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `<=` operator (less than or equal to)
                BinOp::Le(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_le(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `!=` operator (not equal to)
                BinOp::Ne(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_ne(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `>=` operator (greater than or equal to)
                BinOp::Ge(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_ge(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `>` operator (greater than)
                BinOp::Gt(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_gt(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `^` operator (bitwise xor)
                BinOp::BitXor(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_bitxor(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `<<` operator (shift left)
                BinOp::Shl(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_shl(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // The `>>` operator (shift right)
                BinOp::Shr(_) => {
                    return syn::parse_str::<Expr>(&format!(
                        "({}).op_shr(&{})",
                        b.left.to_token_stream(),
                        b.right.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                // Unknown operator
                _ => {
                    return Err(Error::from(format!(
                        "unsupported token {}",
                        b.op.to_token_stream()
                    )))
                }
            }
            Ok(Expr::Binary(b))
        }
        Expr::Unary(mut b) => {
            b.expr = Box::new(translate(context, *b.expr, ignore)?);
            if b.op.to_token_stream().to_string().trim() == "-" {
                return syn::parse_str::<Expr>(&format!(
                    "0i64.op_sub({})",
                    b.expr.to_token_stream().to_string().trim()
                ))
                .map_err(Error::from);
            }
            if b.op.to_token_stream().to_string().trim() == "!" {
                // the same conversion `and`/`or` use, so `!x`, `x && y` and `x || y`
                // agree on what a `rbs::Value` means as a bool
                b.expr = Box::new(
                    syn::parse_str::<Expr>(&format!(
                        "bool::op_from({})",
                        b.expr.to_token_stream().to_string().trim()
                    ))
                    .map_err(Error::from)?,
                );
            }
            Ok(Expr::Unary(b))
        }
        //(a-b)
        Expr::Paren(mut b) => {
            b.expr = Box::new(translate(context, *b.expr, ignore)?);
            Ok(Expr::Paren(b))
        }
        //a.b
        Expr::Field(mut b) => {
            b.base = Box::new(translate(context, *b.base, ignore)?);
            match b.member {
                Member::Named(named) => {
                    return syn::parse_str::<Expr>(&format!(
                        "(&{}[\"{}\"])",
                        b.base.to_token_stream(),
                        named.to_token_stream()
                    ))
                    .map_err(Error::from);
                }
                Member::Unnamed(_) => {}
            }
            Ok(Expr::Field(b))
        }
        Expr::Reference(mut b) => {
            b.expr = Box::new(translate(context, *b.expr, ignore)?);
            let result = Expr::Reference(b);
            Ok(result)
        }
        Expr::Index(mut b) => {
            *b.expr = translate(context, *b.expr, ignore)?;
            if !matches!(
                b.index.as_ref(),
                Expr::Lit(syn::ExprLit {
                    lit: Lit::Int(_) | Lit::Str(_),
                    ..
                })
            ) {
                *b.index = translate(context, *b.index, ignore)?;
            }
            syn::parse_str::<Expr>(&format!(
                "&{}[{}]",
                b.expr.to_token_stream(),
                b.index.to_token_stream()
            ))
            .map_err(Error::from)
        }
        Expr::Let(_let_expr) => Err(Error::from("unsupported token `let`")),
        Expr::Lit(b) => {
            match b.lit {
                Lit::Str(_) => {}
                Lit::ByteStr(_) => {}
                Lit::Byte(_) => {}
                Lit::Char(_) => {}
                Lit::Int(i) => {
                    //cast int to i64
                    return syn::parse_str::<Expr>(&format!("{}i64", i)).map_err(Error::from);
                }
                Lit::Float(f) => {
                    //cast int to f64
                    return syn::parse_str::<Expr>(&format!("{}f64", f)).map_err(Error::from);
                }
                Lit::Bool(_) => {}
                Lit::Verbatim(_) => {}
                _ => {}
            }
            Ok(Expr::Lit(b))
        }
        unsupported => Err(Error::from(format!(
            "unsupported expression: {}",
            unsupported.to_token_stream()
        ))),
    }
}

/// gen method or body(if func_name_ident is empty)
///
/// This is the infallible compatibility wrapper: it panics when the template
/// expression cannot be parsed. The proc-macro path uses
/// [`try_impl_fn_with_span`] so the error is reported at the template literal.
pub fn impl_fn(
    context: &str,
    func_name_ident: &str,
    args: &str,
    serialize_result: bool,
    ignore: &[String],
) -> proc_macro2::TokenStream {
    try_impl_fn_with_span(
        context,
        func_name_ident,
        args,
        serialize_result,
        ignore,
        Span::call_site(),
    )
    .unwrap_or_else(|e| panic!("{}", e))
}

/// gen method or body(if func_name_ident is empty), reporting a template expression
/// that does not parse as a spanned [`syn::Error`].
pub fn try_impl_fn_with_span(
    context: &str,
    func_name_ident: &str,
    args: &str,
    serialize_result: bool,
    ignore: &[String],
    span: Span,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut string_data = args.to_string();
    string_data = string_data[1..string_data.len() - 1].to_string();
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
    let mut t = syn::parse_str::<Expr>(&string_data).map_err(|e| {
        syn::Error::new(
            span,
            format!("[rbatis-codegen]syn::parse_str: {} fail: {}", args, e),
        )
    })?;
    t = translate(context, t, ignore)
        .map_err(|e| syn::Error::new(span, format!("translate fail: {}", e)))?;
    string_data = t.to_token_stream().to_string();
    let t = syn::parse_str::<Expr>(&string_data).map_err(|e| {
        syn::Error::new(
            span,
            format!("[rbatis-codegen]syn::parse_str: {} fail: {}", args, e),
        )
    })?;
    let mut result_impl = quote! { {#t} };
    if serialize_result {
        result_impl = quote! {rbs::value({#t}).unwrap_or_default()};
    }
    if func_name_ident.is_empty() || func_name_ident.eq("\"\"") {
        Ok(quote! {#result_impl})
    } else {
        let func_name_ident = Ident::new(func_name_ident, span);
        Ok(quote! {
            pub fn #func_name_ident(arg:&rbs::Value) -> rbs::Value {
               use rbatis_codegen::ops::*;
               #result_impl
            }
        })
    }
}
