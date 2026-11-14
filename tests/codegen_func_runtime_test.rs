// Runtime coverage tests for `rbatis-codegen/src/codegen/func.rs` (`translate` + `impl_fn`)
// and the `ParseArgs` `Parse`/`Clone` impls in `rbatis-codegen/src/codegen/mod.rs`.
//
// The generated-token assertions below were pinned from actual output of this code.
#[cfg(test)]
mod test {
    use proc_macro2::Literal;
    use quote::ToTokens;
    use rbatis_codegen::codegen::func::{impl_fn, translate};
    use rbatis_codegen::codegen::ParseArgs;
    use syn::{Expr, ExprLit, Lit};

    fn tr(src: &str) -> String {
        let e = syn::parse_str::<Expr>(src).unwrap();
        translate("", e, &[]).unwrap().to_token_stream().to_string()
    }

    /// Same as `tr` but with a non-empty context and an ignore-list.
    fn tr_ignored(src: &str, ignore: &[&str]) -> String {
        let e = syn::parse_str::<Expr>(src).unwrap();
        let ignore: Vec<String> = ignore.iter().map(|s| s.to_string()).collect();
        translate("some_context", e, &ignore)
            .unwrap()
            .to_token_stream()
            .to_string()
    }

    fn tr_err(src: &str) -> String {
        let e = syn::parse_str::<Expr>(src).unwrap();
        translate("", e, &[])
            .err()
            .expect("expected Err")
            .to_string()
    }

    // ------------------------------------------------------------------
    // Expr::Path
    // ------------------------------------------------------------------

    #[test]
    fn path_plain_becomes_arg_index() {
        assert_eq!(tr("a"), r#"& arg ["a"]"#);
    }

    #[test]
    fn path_null_becomes_value_null() {
        assert_eq!(tr("null"), "rbs :: Value :: Null");
    }

    #[test]
    fn path_sql_returned_unchanged() {
        assert_eq!(tr("sql"), "sql");
    }

    #[test]
    fn path_in_ignore_list_returned_unchanged() {
        // `ignore` loop hits `param.eq(x)` and clears `fetch_from_arg`.
        assert_eq!(tr_ignored("a", &["a"]), "a");
    }

    #[test]
    fn path_not_in_ignore_list_becomes_arg_index() {
        assert_eq!(tr_ignored("a", &["b", "c"]), r#"& arg ["a"]"#);
    }

    #[test]
    fn path_ignore_list_is_exact_match() {
        // "ab" is not equal to "a", so the arg-index branch is taken.
        assert_eq!(tr_ignored("ab", &["a"]), r#"& arg ["ab"]"#);
    }

    // ------------------------------------------------------------------
    // Expr::MethodCall
    // ------------------------------------------------------------------

    #[test]
    fn method_call_receiver_is_translated() {
        assert_eq!(
            tr(r#"name.contains_str("x")"#),
            r#"(& arg ["name"]) . contains_str (rbs :: value ("x") . unwrap_or_default ())"#
        );
    }

    #[test]
    fn string_method_arguments_are_translated() {
        assert_eq!(
            tr("name.contains_str(other)"),
            r#"(& arg ["name"]) . contains_str (rbs :: value (& arg ["other"]) . unwrap_or_default ())"#
        );
        assert_eq!(
            tr("name.starts_with(prefix)"),
            r#"(& arg ["name"]) . starts_with (rbs :: value (& arg ["prefix"]) . unwrap_or_default ())"#
        );
        assert_eq!(
            tr("name.ends_with(suffix)"),
            r#"(& arg ["name"]) . ends_with (rbs :: value (& arg ["suffix"]) . unwrap_or_default ())"#
        );
    }

    #[test]
    fn method_call_chained_receivers_are_translated_recursively() {
        assert_eq!(tr("a.trim().len()"), r#"(& arg ["a"]) . trim () . len ()"#);
    }

    // ------------------------------------------------------------------
    // Expr::Binary: `+` special branches
    // ------------------------------------------------------------------

    #[test]
    fn binary_add_ordinary_operands() {
        assert_eq!(tr("a + b"), r#"(& arg ["a"]) . op_add (& & arg ["b"])"#);
    }

    #[test]
    fn binary_add_string_literal_left_uses_string_from() {
        // Left operand token starts and ends with `"`.
        assert_eq!(
            tr("\"s\" + x"),
            r#"(String :: from ("s")) . op_add (& arg ["x"])"#
        );
    }

    #[test]
    fn binary_add_nested_expression() {
        // Left operand is itself a BinOp, so the ordinary branch is used again.
        assert_eq!(
            tr("a + b + c"),
            r#"((& arg ["a"]) . op_add (& & arg ["b"])) . op_add (& & arg ["c"])"#
        );
    }

    // ------------------------------------------------------------------
    // Expr::Binary: all arithmetic / bitwise / comparison operators
    // ------------------------------------------------------------------

    #[test]
    fn binary_sub() {
        assert_eq!(tr("a - b"), r#"(& arg ["a"]) . op_sub (& & arg ["b"])"#);
    }

    #[test]
    fn binary_mul() {
        assert_eq!(tr("a * b"), r#"(& arg ["a"]) . op_mul (& & arg ["b"])"#);
    }

    #[test]
    fn binary_div() {
        assert_eq!(tr("a / b"), r#"(& arg ["a"]) . op_div (& & arg ["b"])"#);
    }

    #[test]
    fn binary_rem() {
        assert_eq!(tr("a % b"), r#"(& arg ["a"]) . op_rem (& & arg ["b"])"#);
    }

    #[test]
    fn binary_bitand() {
        assert_eq!(tr("a & b"), r#"(& arg ["a"]) . op_bitand (& & arg ["b"])"#);
    }

    #[test]
    fn binary_bitor() {
        assert_eq!(tr("a | b"), r#"(& arg ["a"]) . op_bitor (& & arg ["b"])"#);
    }

    #[test]
    fn binary_eq() {
        assert_eq!(tr("a == b"), r#"(& arg ["a"]) . op_eq (& & arg ["b"])"#);
    }

    #[test]
    fn binary_lt() {
        assert_eq!(tr("a < b"), r#"(& arg ["a"]) . op_lt (& & arg ["b"])"#);
    }

    #[test]
    fn binary_le() {
        assert_eq!(tr("a <= b"), r#"(& arg ["a"]) . op_le (& & arg ["b"])"#);
    }

    #[test]
    fn binary_ne() {
        assert_eq!(tr("a != b"), r#"(& arg ["a"]) . op_ne (& & arg ["b"])"#);
    }

    #[test]
    fn binary_ge() {
        assert_eq!(tr("a >= b"), r#"(& arg ["a"]) . op_ge (& & arg ["b"])"#);
    }

    #[test]
    fn binary_gt() {
        assert_eq!(tr("a > b"), r#"(& arg ["a"]) . op_gt (& & arg ["b"])"#);
    }

    #[test]
    fn binary_bitxor() {
        assert_eq!(tr("a ^ b"), r#"(& arg ["a"]) . op_bitxor (& & arg ["b"])"#);
    }

    #[test]
    fn binary_shl() {
        assert_eq!(tr("a << b"), r#"(& arg ["a"]) . op_shl (& & arg ["b"])"#);
    }

    #[test]
    fn binary_shr() {
        assert_eq!(tr("a >> b"), r#"(& arg ["a"]) . op_shr (& & arg ["b"])"#);
    }

    #[test]
    fn binary_and_wraps_both_operands_in_op_from() {
        assert_eq!(
            tr("a && b"),
            r#"bool :: op_from (& arg ["a"]) && bool :: op_from (& arg ["b"])"#
        );
    }

    #[test]
    fn binary_or_wraps_both_operands_in_op_from() {
        assert_eq!(
            tr("a || b"),
            r#"bool :: op_from (& arg ["a"]) || bool :: op_from (& arg ["b"])"#
        );
    }

    #[test]
    fn unsupported_binary_operator_is_error() {
        // Compound assignment parses as Expr::Binary with BinOp::AddAssign, which
        // falls into the `_` arm of the operator match.
        assert_eq!(tr_err("a += b"), "unsupported token +=");
    }

    // ------------------------------------------------------------------
    // Expr::Unary
    // ------------------------------------------------------------------

    #[test]
    fn unary_neg_uses_i64_zero_op_sub() {
        assert_eq!(tr("-x"), r#"0i64 . op_sub (& arg ["x"])"#);
    }

    #[test]
    fn unary_not_wraps_operand_in_op_from() {
        assert_eq!(tr("!x"), r#"! bool :: op_from (& arg ["x"])"#);
    }

    #[test]
    fn unary_deref_falls_through_unchanged() {
        assert_eq!(tr("*x"), r#"* & arg ["x"]"#);
    }

    // ------------------------------------------------------------------
    // Expr::Paren / Field / Reference / Index
    // ------------------------------------------------------------------

    #[test]
    fn paren_translates_inner_expression() {
        assert_eq!(tr("(a + 1)"), r#"((& arg ["a"]) . op_add (& 1i64))"#);
    }

    #[test]
    fn field_named_member_becomes_nested_index() {
        assert_eq!(tr("a.b"), r#"(& & arg ["a"] ["b"])"#);
    }

    #[test]
    fn field_unnamed_member_keeps_field_expr() {
        assert_eq!(tr("a.0"), r#"(& arg ["a"]) . 0"#);
    }

    #[test]
    fn reference_translates_inner_expression() {
        assert_eq!(tr("&x"), r#"& & arg ["x"]"#);
    }

    #[test]
    fn index_translates_base_and_argument_index() {
        assert_eq!(tr("a[b]"), r#"& & arg ["a"] [& arg ["b"]]"#);
        assert_eq!(tr("a[0]"), r#"& & arg ["a"] [0]"#);
    }

    // ------------------------------------------------------------------
    // Expr::Let error
    // ------------------------------------------------------------------

    #[test]
    fn let_expression_is_error() {
        assert_eq!(tr_err("let x = 1"), "unsupported token `let`");
    }

    // ------------------------------------------------------------------
    // Expr::Lit
    // ------------------------------------------------------------------

    #[test]
    fn lit_string_unchanged() {
        assert_eq!(tr("\"x\""), r#""x""#);
    }

    #[test]
    fn lit_byte_string_unchanged() {
        assert_eq!(tr("b\"x\""), r#"b"x""#);
    }

    #[test]
    fn lit_byte_unchanged() {
        assert_eq!(tr("b'x'"), "b'x'");
    }

    #[test]
    fn lit_char_unchanged() {
        assert_eq!(tr("'x'"), "'x'");
    }

    #[test]
    fn lit_int_gets_i64_suffix() {
        assert_eq!(tr("1"), "1i64");
    }

    #[test]
    fn lit_float_gets_f64_suffix() {
        assert_eq!(tr("1.5"), "1.5f64");
    }

    #[test]
    fn lit_bool_unchanged() {
        assert_eq!(tr("true"), "true");
        assert_eq!(tr("false"), "false");
    }

    #[test]
    fn lit_verbatim_unchanged() {
        // `syn::parse_str` never produces Lit::Verbatim, so build one directly.
        let e = Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: Lit::Verbatim(Literal::string("v")),
        });
        let out = translate("", e, &[]).unwrap().to_token_stream().to_string();
        assert_eq!(out, r#""v""#);
    }

    #[test]
    fn lit_cstr_hits_fallback_arm() {
        // C string literals are a `Lit` variant the match does not name
        // explicitly, so they reach the `_ => {}` fallback.
        let e = syn::parse_str::<Expr>(r#"c"x""#).unwrap();
        match &e {
            Expr::Lit(ExprLit {
                lit: Lit::CStr(_), ..
            }) => {}
            other => panic!("expected Lit::CStr, got {:?}", other.to_token_stream()),
        }
        assert_eq!(
            translate("", e, &[]).unwrap().to_token_stream().to_string(),
            r#"c"x""#
        );
    }

    // ------------------------------------------------------------------
    // Unsupported expressions
    // ------------------------------------------------------------------

    #[test]
    fn unsupported_expression_is_an_error() {
        assert_eq!(tr_err("foo(1)"), "unsupported expression: foo (1)");
        assert_eq!(
            tr_err("name as bool"),
            "unsupported expression: name as bool"
        );
    }

    // ------------------------------------------------------------------
    // impl_fn: function body vs named function
    // ------------------------------------------------------------------

    #[test]
    fn impl_fn_empty_name_generates_expression_body() {
        let out = impl_fn("", "", r#""a + 1""#, false, &[]).to_string();
        assert_eq!(out, r#"{ (& arg ["a"]) . op_add (& 1i64) }"#);
    }

    #[test]
    fn impl_fn_quoted_empty_name_generates_expression_body() {
        // `"\"\""` is the two-character string `""`.
        let out = impl_fn("", "\"\"", r#""a + 1""#, false, &[]).to_string();
        assert_eq!(out, r#"{ (& arg ["a"]) . op_add (& 1i64) }"#);
    }

    #[test]
    fn impl_fn_real_ident_generates_pub_fn() {
        let out = impl_fn("", "my_fn", r#""a + 1""#, false, &[]).to_string();
        assert_eq!(
            out,
            r#"pub fn my_fn (arg : & rbs :: Value) -> rbs :: Value { use rbatis_codegen :: ops :: * ; { (& arg ["a"]) . op_add (& 1i64) } }"#
        );
        assert!(out.contains("pub fn my_fn"));
    }

    // ------------------------------------------------------------------
    // impl_fn: serialize_result
    // ------------------------------------------------------------------

    #[test]
    fn impl_fn_serialize_false_has_no_rbs_value() {
        let out = impl_fn("", "", r#""1""#, false, &[]).to_string();
        assert_eq!(out, r#"{ 1i64 }"#);
        assert!(!out.contains("rbs :: value"));
    }

    #[test]
    fn impl_fn_serialize_true_wraps_in_rbs_value() {
        let out = impl_fn("", "", r#""a + 1""#, true, &[]).to_string();
        assert_eq!(
            out,
            r#"rbs :: value ({ (& arg ["a"]) . op_add (& 1i64) }) . unwrap_or_default ()"#
        );
        assert!(out.contains("rbs :: value"));
        assert!(out.contains("unwrap_or_default"));
    }

    // ------------------------------------------------------------------
    // impl_fn: quote conversion loop
    // ------------------------------------------------------------------

    #[test]
    fn impl_fn_converts_single_quotes_to_double_quotes() {
        let out = impl_fn("", "", r#""a == 'x'""#, false, &[]).to_string();
        assert_eq!(out, r#"{ (& arg ["a"]) . op_eq (& "x") }"#);
    }

    #[test]
    fn impl_fn_keeps_escaped_single_quote() {
        // Quote preceded by `\` is not converted; the resulting Rust string
        // literal `"it\'s"` still parses.
        let out = impl_fn("", "", r#""a == 'it\'s'""#, false, &[]).to_string();
        assert_eq!(out, r#"{ (& arg ["a"]) . op_eq (& "it\'s") }"#);
    }

    #[test]
    fn impl_fn_strips_outer_argument_quotes() {
        // `'abc'`: first/last chars are stripped, leaving path `abc`.
        let out = impl_fn("", "", "'abc'", false, &[]).to_string();
        assert_eq!(out, r#"{ & arg ["abc"] }"#);
    }

    #[test]
    fn impl_fn_ignore_list_is_applied() {
        let ignore = vec!["x".to_string()];
        let out = impl_fn("some_context", "", r#""x""#, false, &ignore).to_string();
        assert_eq!(out, r#"{ x }"#);
    }

    #[test]
    #[should_panic(expected = "syn::parse_str")]
    fn impl_fn_panics_on_unparsable_args() {
        // Exercises the `unwrap_or_else` panic closure of the first parse.
        impl_fn("", "", r#""a +""#, false, &[]);
    }

    // ------------------------------------------------------------------
    // ParseArgs (codegen/mod.rs)
    // ------------------------------------------------------------------

    #[test]
    fn parse_args_parses_comma_separated_lit_strs() {
        let args = syn::parse_str::<ParseArgs>(r#""a", "b""#).unwrap();
        assert_eq!(args.sqls.len(), 2);
        assert_eq!(args.sqls[0].value(), "a");
        assert_eq!(args.sqls[1].value(), "b");
    }

    #[test]
    fn parse_args_accepts_empty_input() {
        let args = syn::parse_str::<ParseArgs>("").unwrap();
        assert!(args.sqls.is_empty());
    }

    #[test]
    fn parse_args_accepts_trailing_comma() {
        let args = syn::parse_str::<ParseArgs>(r#""a", "b","#).unwrap();
        let values: Vec<String> = args.sqls.iter().map(|s| s.value()).collect();
        assert_eq!(values, vec!["a", "b"]);
    }

    #[test]
    fn parse_args_clone_copies_all_sqls() {
        let args = syn::parse_str::<ParseArgs>(r#""one", "two", "three""#).unwrap();
        let cloned = args.clone();
        assert_eq!(cloned.sqls.len(), args.sqls.len());
        let orig: Vec<String> = args.sqls.iter().map(|s| s.value()).collect();
        let copy: Vec<String> = cloned.sqls.iter().map(|s| s.value()).collect();
        assert_eq!(copy, orig);
    }

    #[test]
    fn parse_args_clone_from_replaces_existing_sqls() {
        let args = syn::parse_str::<ParseArgs>(r#""a", "b""#).unwrap();
        let mut target = ParseArgs { sqls: Vec::new() };
        target.clone_from(&args);
        let values: Vec<String> = target.sqls.iter().map(|s| s.value()).collect();
        assert_eq!(values, vec!["a", "b"]);
    }
}
