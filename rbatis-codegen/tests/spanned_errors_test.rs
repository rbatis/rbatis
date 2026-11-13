//! The fallible codegen entry points return a spanned `syn::Error` instead of
//! panicking. These tests lock in that behaviour: the error text is the same as
//! before, the error is rendered as a `compile_error!`, and the span is the one the
//! caller passed (the SQL/template literal span in a proc macro).

use proc_macro2::Span;
use rbatis_codegen::codegen::parser_html::{try_impl_fn_html, try_parse_html};
use rbatis_codegen::codegen::parser_pysql::try_impl_fn_py;
use rbatis_codegen::codegen::ParseArgs;

#[test]
fn try_parse_html_reports_unsupported_tag() {
    let err = try_parse_html(
        r#"<foobar id="f">x</foobar>"#,
        "f",
        &mut vec![],
        Span::call_site(),
    )
    .expect_err("an unsupported tag must be an error");

    assert!(
        err.to_string().contains("unsupported html tag"),
        "unexpected message: {err}"
    );
    assert!(
        err.to_compile_error().to_string().contains("compile_error"),
        "the error must render as compile_error!"
    );
}

#[test]
fn try_parse_html_reports_missing_statement() {
    let err = try_parse_html("", "f", &mut vec![], Span::call_site())
        .expect_err("an empty template must be an error");
    assert!(
        err.to_string().contains("HTML not found for function"),
        "unexpected message: {err}"
    );
}

#[test]
fn try_parse_html_reports_missing_tag_attribute() {
    let err = try_parse_html(
        r#"<select id="f"><if>`x`</if></select>"#,
        "f",
        &mut vec![],
        Span::call_site(),
    )
    .expect_err("<if> without test must be an error");
    assert!(
        err.to_string()
            .contains("<if> is missing the required \"test\" attribute"),
        "unexpected message: {err}"
    );
}

#[test]
fn elements_keep_the_template_line() {
    let html = "<select id=\"s\">\n  <if test=\"a\">\n  </if>\n</select>";
    let map = rbatis_codegen::codegen::parser_html::load_mapper_map(html).unwrap();
    let select = map.get("s").expect("select is stored in the map");
    assert_eq!(select.source.line, 1);
    assert_eq!(select.childs[0].source.line, 2);
}

#[test]
fn try_parse_html_reports_the_source_line() {
    let html = "<select id=\"f\">\n  <if>`x`</if>\n</select>";
    let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
        .expect_err("<if> without test must be an error");
    let message = err.to_string();
    assert!(message.contains("template:2"), "unexpected message: {message}");
    assert!(message.contains("<if>`x`</if>"), "missing source line: {message}");
}

#[test]
fn set_rejects_invalid_skip_null() {
    let err = try_parse_html(
        r#"<select id="f"><set collection="m" skip_null="yes">`x`</set></select>"#,
        "f",
        &mut vec![],
        Span::call_site(),
    )
    .expect_err("an invalid skip_null value must be an error");
    assert!(
        err.to_string().contains("invalid skip_null"),
        "unexpected message: {err}"
    );
}

#[test]
fn fallback_prefers_the_single_statement_over_a_sql_fragment() {
    let html = r#"<sql id="a">`frag`</sql><select id="other">`stmt`</select>"#;
    let code = try_parse_html(html, "missing", &mut vec![], Span::call_site())
        .expect("the single statement must be selected")
        .to_string();
    assert!(code.contains("stmt"), "got: {code}");
    assert!(!code.contains("frag"), "got: {code}");
}

#[test]
fn nested_node_error_has_a_single_code_frame() {
    let html = "<select id=\"f\">\n  <where>\n    <if>`x`</if>\n  </where>\n</select>";
    let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
        .expect_err("<if> without test must be an error");
    let message = err.to_string();
    assert_eq!(
        message.matches("--> template:").count(),
        1,
        "the innermost node must be the only code frame: {message}"
    );
    assert!(message.contains("template:3"), "{message}");
}

#[test]
fn foreach_missing_collection_has_a_code_frame() {
    let html = "<select id=\"f\">\n  <foreach item=\"id\">`x`</foreach>\n</select>";
    let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
        .expect_err("<foreach> without collection must be an error");
    let message = err.to_string();
    assert!(message.contains("template:2"), "{message}");
    assert!(message.contains("<foreach item=\"id\">"), "{message}");
    assert!(message.contains("collection"), "{message}");
}

#[test]
fn py_sql_errors_do_not_claim_html_source_lines() {
    let f: syn::ItemFn = syn::parse_quote! {
        pub async fn f(rb: &dyn rbatis::Executor) -> rbatis::Result<()> {}
    };
    let args = ParseArgs {
        sqls: vec![syn::LitStr::new("if:", Span::call_site())],
    };
    let err = try_impl_fn_py(&f, &args).expect_err("an empty if test must be an error");
    let message = err.to_string();
    assert!(message.contains("test"), "{message}");
    assert!(
        !message.contains("--> template:"),
        "py_sql has no html source line to show: {message}"
    );
}

#[test]
fn try_impl_fn_html_without_template_is_error() {
    let f: syn::ItemFn = syn::parse_quote! {
        pub async fn f(rb: &dyn rbatis::Executor) -> rbatis::Result<()> {}
    };
    let args = ParseArgs { sqls: vec![] };
    let err = try_impl_fn_html(&f, &args).expect_err("missing html_data must be an error");
    assert!(
        err.to_string().contains("must have html_data"),
        "unexpected message: {err}"
    );
}

#[test]
fn try_impl_fn_py_reports_parse_error() {
    let f: syn::ItemFn = syn::parse_quote! {
        pub async fn f(rb: &dyn rbatis::Executor) -> rbatis::Result<()> {}
    };
    let args = ParseArgs {
        sqls: vec![syn::LitStr::new("for x:", Span::call_site())],
    };
    let err = try_impl_fn_py(&f, &args).expect_err("an invalid py_sql template must be an error");
    assert!(
        err.to_string().contains("parse py_sql fail"),
        "unexpected message: {err}"
    );
}
