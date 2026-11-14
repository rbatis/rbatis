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
    assert!(
        message.contains("template:2"),
        "unexpected message: {message}"
    );
    assert!(
        message.contains("<if>`x`</if>"),
        "missing source line: {message}"
    );
}

#[test]
fn text_expression_error_points_to_the_expression_line_and_column() {
    let html = concat!(
        "<select id=\"f\">\n",
        "  <if test=\"name == ''\">\n",
        "    ` and name like #{name === 1}`\n",
        "  </if>\n",
        "</select>",
    );
    let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
        .expect_err("the invalid interpolation expression must be an error");
    let message = err.to_string();

    assert!(message.contains("template:3:"), "wrong line: {message}");
    assert!(
        message.contains("` and name like #{name === 1}`"),
        "missing text line: {message}"
    );
    assert!(
        message.contains("^^^^^^^^^^ check the expression"),
        "the expression itself must be underlined: {message}"
    );
    assert!(
        !message.contains("template:2:"),
        "the parent <if> line must not be used: {message}"
    );
}

#[test]
fn text_expression_error_points_to_its_line_inside_a_multiline_text_node() {
    let html = concat!(
        "<select id=\"f\">\n",
        "  <if test=\"true\">\n",
        "    `select *\n",
        "     from activity\n",
        "     where name like #{name === 1}`\n",
        "  </if>\n",
        "</select>",
    );
    let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
        .expect_err("the invalid interpolation expression must be an error");
    let message = err.to_string();

    assert!(message.contains("template:5:"), "wrong line: {message}");
    assert!(
        message.contains("where name like #{name === 1}`"),
        "missing expression line: {message}"
    );
    assert!(
        message.contains("^^^^^^^^^^ check the expression"),
        "the expression itself must be underlined: {message}"
    );
}

#[test]
fn text_expression_errors_keep_their_source_in_every_container() {
    let cases = [
        (
            "select",
            "<select id=\"f\">\n  `#{name === 1}`\n</select>",
            2,
        ),
        ("sql", "<sql id=\"f\">\n  `#{name === 1}`\n</sql>", 2),
        (
            "insert",
            "<insert id=\"f\">\n  `#{name === 1}`\n</insert>",
            2,
        ),
        (
            "update",
            "<update id=\"f\">\n  `#{name === 1}`\n</update>",
            2,
        ),
        (
            "delete",
            "<delete id=\"f\">\n  `#{name === 1}`\n</delete>",
            2,
        ),
        (
            "mapper",
            concat!(
                "<mapper>\n",
                "  <select id=\"f\">\n",
                "    `#{name === 1}`\n",
                "  </select>\n",
                "</mapper>",
            ),
            3,
        ),
        (
            "if",
            "<select id=\"f\">\n  <if test=\"true\">\n    `#{name === 1}`\n  </if>\n</select>",
            3,
        ),
        (
            "when",
            concat!(
                "<select id=\"f\">\n",
                "  <choose>\n",
                "    <when test=\"true\">\n",
                "      `#{name === 1}`\n",
                "    </when>\n",
                "  </choose>\n",
                "</select>",
            ),
            4,
        ),
        (
            "otherwise",
            concat!(
                "<select id=\"f\">\n",
                "  <choose>\n",
                "    <otherwise>\n",
                "      `#{name === 1}`\n",
                "    </otherwise>\n",
                "  </choose>\n",
                "</select>",
            ),
            4,
        ),
        (
            "foreach",
            concat!(
                "<select id=\"f\">\n",
                "  <foreach collection=\"items\" item=\"item\">\n",
                "    `#{name === 1}`\n",
                "  </foreach>\n",
                "</select>",
            ),
            3,
        ),
        (
            "trim",
            "<select id=\"f\">\n  <trim>\n    `#{name === 1}`\n  </trim>\n</select>",
            3,
        ),
        (
            "where",
            "<select id=\"f\">\n  <where>\n    `#{name === 1}`\n  </where>\n</select>",
            3,
        ),
        (
            "set",
            "<update id=\"f\">\n  <set>\n    `#{name === 1}`\n  </set>\n</update>",
            3,
        ),
    ];

    for (container, html, line) in cases {
        let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
            .expect_err("the invalid interpolation expression must be an error");
        let message = err.to_string();
        assert!(
            message.contains(&format!("template:{line}:")),
            "{container} reported the wrong line: {message}"
        );
        assert!(
            message.contains("`#{name === 1}`"),
            "{container} did not show the text line: {message}"
        );
    }
}

#[test]
fn resolved_include_child_errors_keep_the_fragment_source() {
    let html = concat!(
        "<sql id=\"frag\">\n",
        "  `#{name === 1}`\n",
        "</sql>\n",
        "<select id=\"f\">\n",
        "  <include refid=\"frag\"></include>\n",
        "</select>",
    );
    let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
        .expect_err("the invalid included expression must be an error");
    let message = err.to_string();
    assert!(message.contains("template:2:"), "wrong line: {message}");
    assert!(message.contains("`#{name === 1}`"), "{message}");
}

#[test]
fn structural_node_errors_point_to_the_owning_tag() {
    let cases = [
        (
            "choose",
            "<select id=\"f\">\n  <choose>\n    invalid\n  </choose>\n</select>",
            3,
            "invalid",
        ),
        (
            "include",
            "<select id=\"f\">\n  <include></include>\n</select>",
            2,
            "<include>",
        ),
        ("sql", "<sql>`x`</sql>", 1, "<sql>"),
    ];

    for (node, html, line, source_text) in cases {
        let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
            .expect_err("the invalid node structure must be an error");
        let message = err.to_string();
        assert!(
            message.contains(&format!("template:{line}:")),
            "<{node}> reported the wrong line: {message}"
        );
        assert!(
            message.contains(source_text),
            "<{node}> did not show its source: {message}"
        );
    }
}

#[test]
fn node_expression_errors_point_to_the_owning_tag() {
    let cases = [
        (
            "if",
            "<select id=\"f\">\n  <if test=\"name === 1\">`x`</if>\n</select>",
            2,
        ),
        (
            "bind",
            "<select id=\"f\">\n  <bind name=\"x\" value=\"name === 1\">`x`</bind>\n</select>",
            2,
        ),
        (
            "foreach",
            "<select id=\"f\">\n  <foreach collection=\"name === 1\">`x`</foreach>\n</select>",
            2,
        ),
        (
            "when",
            concat!(
                "<select id=\"f\">\n",
                "  <choose>\n",
                "    <when test=\"name === 1\">`x`</when>\n",
                "  </choose>\n",
                "</select>",
            ),
            3,
        ),
    ];

    for (node, html, line) in cases {
        let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
            .expect_err("the invalid node expression must be an error");
        let message = err.to_string();
        assert!(
            message.contains(&format!("template:{line}:")),
            "<{node}> reported the wrong line: {message}"
        );
        assert!(
            message.contains(&format!("<{node}")),
            "<{node}> did not show its own source line: {message}"
        );
    }
}

#[test]
fn invalid_identifiers_are_reported_without_panicking() {
    let cases = [
        (
            "foreach item",
            "<select id=\"f\">\n  <foreach collection=\"items\" item=\"a-b\">`x`</foreach>\n</select>",
            "invalid item",
        ),
        (
            "foreach index",
            "<select id=\"f\">\n  <foreach collection=\"items\" index=\"a-b\">`x`</foreach>\n</select>",
            "invalid index",
        ),
        (
            "bind name",
            "<select id=\"f\">\n  <bind name=\"a-b\" value=\"1\">`x`</bind>\n</select>",
            "invalid name",
        ),
    ];

    for (case, html, expected) in cases {
        let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
            .expect_err("an invalid identifier must be a template error");
        assert!(err.to_string().contains(expected), "{case}: {err}");
        assert!(err.to_string().contains("template:2"), "{case}: {err}");
    }
}

#[test]
fn py_sql_rejects_invalid_foreach_identifiers() {
    let f: syn::ItemFn = syn::parse_quote! {
        pub async fn f(rb: &dyn rbatis::Executor) -> rbatis::Result<()> {}
    };
    let args = ParseArgs {
        sqls: vec![syn::LitStr::new(
            "for a-b in items:\n  `x`",
            Span::call_site(),
        )],
    };
    let err = try_impl_fn_py(&f, &args).expect_err("invalid py_sql item must fail");
    assert!(err.to_string().contains("invalid py_sql foreach item"), "{err}");
}

#[test]
fn sql_less_than_text_does_not_desynchronize_later_source_lines() {
    let html = concat!(
        "<select id=\"f\">\n",
        "  `where a < #{a}`\n",
        "  <if test=\"true\">\n",
        "    `and b = #{name === 1}`\n",
        "  </if>\n",
        "</select>",
    );
    let err = try_parse_html(html, "f", &mut vec![], Span::call_site())
        .expect_err("the invalid interpolation expression must be an error");
    let message = err.to_string();
    assert!(message.contains("template:4:"), "wrong line: {message}");
    assert!(message.contains("`and b = #{name === 1}`"), "{message}");
}

#[test]
fn cdata_is_parsed_as_sql_text() {
    let html = concat!(
        "<select id=\"f\">\n",
        "  <![CDATA[`select * from t where a < #{a}`]]>\n",
        "</select>",
    );
    let code = try_parse_html(html, "f", &mut vec![], Span::call_site())
        .expect("CDATA must be valid template text")
        .to_string();
    assert!(code.contains("where a <"), "{code}");
    assert!(code.contains("arg [\"a\"]"), "{code}");
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
