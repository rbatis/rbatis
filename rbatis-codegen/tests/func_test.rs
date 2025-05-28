use rbatis_codegen::codegen::func;
use syn::{parse_str, Expr};

#[test]
fn test_translate_path_null() {
    let expr: Expr = parse_str("null").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("rbs :: Value :: Null"));
}

#[test]
fn test_translate_path_sql() {
    let expr: Expr = parse_str("sql").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert_eq!(token_stream.to_string(), "sql");
}

#[test]
fn test_translate_path_param() {
    let expr: Expr = parse_str("user_id").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("& arg [\"user_id\"]"));
}

#[test]
fn test_translate_path_ignored_param() {
    let expr: Expr = parse_str("ignored_param").unwrap();
    let ignore = vec!["ignored_param".to_string()];
    let result = func::translate("", expr, &ignore).unwrap();
    let token_stream = quote::quote! { #result };
    assert_eq!(token_stream.to_string(), "ignored_param");
}

#[test]
fn test_translate_binary_add_string() {
    let expr: Expr = parse_str("\"hello\" + name").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_add"));
}

#[test]
fn test_translate_binary_add_non_string() {
    let expr: Expr = parse_str("a + b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_add"));
}

#[test]
fn test_translate_binary_sub() {
    let expr: Expr = parse_str("a - b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_sub"));
}

#[test]
fn test_translate_binary_mul() {
    let expr: Expr = parse_str("a * b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_mul"));
}

#[test]
fn test_translate_binary_div() {
    let expr: Expr = parse_str("a / b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_div"));
}

#[test]
fn test_translate_binary_rem() {
    let expr: Expr = parse_str("a % b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_rem"));
}

#[test]
fn test_translate_binary_eq() {
    let expr: Expr = parse_str("a == b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_eq"));
}

#[test]
fn test_translate_binary_lt() {
    let expr: Expr = parse_str("a < b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_lt"));
}

#[test]
fn test_translate_binary_le() {
    let expr: Expr = parse_str("a <= b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_le"));
}

#[test]
fn test_translate_binary_ne() {
    let expr: Expr = parse_str("a != b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_ne"));
}

#[test]
fn test_translate_binary_ge() {
    let expr: Expr = parse_str("a >= b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_ge"));
}

#[test]
fn test_translate_binary_gt() {
    let expr: Expr = parse_str("a > b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_gt"));
}

#[test]
fn test_translate_binary_bitand() {
    let expr: Expr = parse_str("a & b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_bitand"));
}

#[test]
fn test_translate_binary_bitor() {
    let expr: Expr = parse_str("a | b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_bitor"));
}

#[test]
fn test_translate_binary_bitxor() {
    let expr: Expr = parse_str("a ^ b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_bitxor"));
}

#[test]
fn test_translate_binary_and() {
    let expr: Expr = parse_str("a && b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("bool :: op_from"));
}

#[test]
fn test_translate_binary_or() {
    let expr: Expr = parse_str("a || b").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("bool :: op_from"));
}

#[test]
fn test_translate_method_call() {
    let expr: Expr = parse_str("user.get_name()").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("get_name"));
}

#[test]
fn test_translate_nested_expressions() {
    let expr: Expr = parse_str("(a + b) * c").unwrap();
    let result = func::translate("", expr, &[]).unwrap();
    let token_stream = quote::quote! { #result };
    assert!(token_stream.to_string().contains("op_mul"));
    assert!(token_stream.to_string().contains("op_add"));
}
