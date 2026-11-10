//! Runtime coverage for the html_sql code generator:
//! - `rbatis-codegen/src/codegen/parser_html.rs`
//! - `rbatis-codegen/src/codegen/loader_html.rs`
//! - `rbatis-codegen/src/codegen/string_util.rs`
//! - `rbatis-codegen/src/codegen/syntax_tree_html/{select,insert,update,delete,mapper,
//!   sql,include,bind,break,continue}_tag_node.rs`
//!
//! Everything here calls the public `rbatis_codegen` API at runtime (no proc-macro
//! bridge except the two `#[should_panic]` tests for `impl_fn_html`).

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use proc_macro2::TokenStream;
    use quote::quote;
    use rbatis_codegen::codegen::loader_html::{load_html, load_mapper_vec, Element};
    use rbatis_codegen::codegen::parser_html::{
        load_mapper_map, load_mapper_vec as parser_load_mapper_vec, parse_html,
    };
    use rbatis_codegen::codegen::string_util::{
        concat_str, count_string_num, escape_attr_value, find_convert_string, un_packing_string,
    };
    use rbatis_codegen::codegen::syntax_tree_html::{HtmlAstNode, IncludeTagNode, NodeContext};

    // ---------------------------------------------------------------- helpers

    fn parse(html: &str, fn_name: &str) -> String {
        let mut ignore = vec![];
        parse_html(html, fn_name, &mut ignore).to_string()
    }

    fn element(tag: &str, data: &str, attrs: &[(&str, &str)], childs: Vec<Element>) -> Element {
        Element {
            tag: tag.to_string(),
            data: data.to_string(),
            attrs: attrs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            childs,
        }
    }

    fn text(data: &str) -> Element {
        element("", data, &[], vec![])
    }

    fn tag(tag: &str, attrs: &[(&str, &str)]) -> Element {
        element(tag, "", attrs, vec![])
    }

    fn fixture_path() -> std::path::PathBuf {
        let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("target");
        p.push("html_codegen_crud_include_fixture.html");
        p
    }

    fn write_fixture() -> std::path::PathBuf {
        let p = fixture_path();
        // The tests in this binary run in parallel. Write the shared fixture only
        // once: a second `fs::write` truncates the file, and another test reading
        // it at that moment would see an empty/partial document.
        static WRITE_ONCE: std::sync::Once = std::sync::Once::new();
        WRITE_ONCE.call_once(|| {
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(&p, r#"<mapper><sql id="a">frag_a</sql></mapper>"#).unwrap();
        });
        p
    }

    // ------------------------------------------------- parser_html: text nodes

    #[test]
    fn text_hash_only_generates_placeholder_and_push() {
        let code = parse(
            r#"<select id="f">`select * from t where id = #{id}`</select>"#,
            "f",
        );
        assert!(code.contains("select * from t where id = ?"), "{code}");
        assert!(code.contains("push"), "{code}");
        assert!(code.contains("rbs"), "{code}");
    }

    #[test]
    fn text_dollar_only_generates_format() {
        let code = parse(
            r#"<select id="f">`select * from t where name = '${name}'`</select>"#,
            "f",
        );
        // ${name} is replaced by a `{}` format placeholder
        assert!(code.contains("name = '{}'"), "{code}");
        assert!(code.contains("string"), "{code}");
        assert!(code.contains("format"), "{code}");
    }

    #[test]
    fn text_mixed_hash_and_dollar() {
        let code = parse(
            r#"<select id="f">`select #{id}, '${name}', #{age} from t`</select>"#,
            "f",
        );
        assert!(code.contains("select ?, '{}', ? from t"), "{code}");
        // two #{..} => two args.push
        assert_eq!(code.matches("push").count(), 2, "{code}");
    }

    #[test]
    fn text_several_dollar_interpolations() {
        let code = parse(
            r#"<select id="f">`select ${a}, ${b}, ${c}`</select>"#,
            "f",
        );
        assert!(code.contains("select {}, {}, {}"), "{code}");
        assert!(code.matches("string").count() >= 3, "{code}");
    }

    #[test]
    fn text_multiline_backtick_wrapping() {
        let code = parse(
            "<select id=\"f\">\n`select *`\n`from t`\n</select>",
            "f",
        );
        assert!(!code.contains('`'), "{code}");
        assert!(code.contains("select *"), "{code}");
        assert!(code.contains("from t"), "{code}");
    }

    #[test]
    fn text_backtick_double_split() {
        // `a``b` -> remove_extra splits on `` and concat_str joins the pieces
        let code = parse(r#"<select id="f">`a``b`</select>"#, "f");
        assert!(code.contains("a b"), "{code}");
    }

    #[test]
    fn text_newline_and_quote_escapes_are_preprocessed() {
        // parse_html replaces the two-character sequences `\n` and `\"`
        let code = parse(
            r#"<select id="f">`select\nfrom t`</select>"#,
            "f",
        );
        assert!(code.contains("select"), "{code}");
        assert!(code.contains("from t"), "{code}");

        let code = parse(
            r#"<select id="f">`select \"x\" from t`</select>"#,
            "f",
        );
        // the generated string literal carries the quotes, escaped
        assert!(code.contains(r#"\"x\""#), "{code}");
    }

    #[test]
    fn text_leading_and_trailing_spaces_trimmed() {
        let code = parse(r#"<select id="f">   select *   </select>"#, "f");
        assert!(code.contains("select *"), "{code}");
        assert!(!code.contains("   select"), "{code}");
    }

    #[test]
    fn text_outer_quotes_are_trimmed() {
        // inline templates are often passed as a bare quoted string
        let code = parse(r#""<select id="f">`quoted_sql`</select>""#, "f");
        assert!(code.contains("quoted_sql"), "{code}");
    }

    // --------------------------------------- parser_html: CRUD node coverage

    #[test]
    fn select_tag_top_level() {
        let code = parse(r#"<select id="f">`select 1`</select>"#, "f");
        assert!(code.contains("pub fn f"), "{code}");
        assert!(code.contains("select 1"), "{code}");
        assert!(code.contains("with_capacity"), "{code}");
    }

    #[test]
    fn insert_tag_top_level() {
        let code = parse(r#"<insert id="f">`insert into t values (1)`</insert>"#, "f");
        assert!(code.contains("pub fn f"), "{code}");
        assert!(code.contains("insert into t values (1)"), "{code}");
    }

    #[test]
    fn update_tag_top_level() {
        let code = parse(r#"<update id="f">`update t set a = 1`</update>"#, "f");
        assert!(code.contains("pub fn f"), "{code}");
        assert!(code.contains("update t set a = 1"), "{code}");
    }

    #[test]
    fn delete_tag_top_level() {
        let code = parse(r#"<delete id="f">`delete from t`</delete>"#, "f");
        assert!(code.contains("pub fn f"), "{code}");
        assert!(code.contains("delete from t"), "{code}");
    }

    #[test]
    fn crud_tags_inside_mapper() {
        // load_mapper_vec flattens the top level <mapper>, then each statement is
        // compiled by its id
        for (html, needle) in [
            (
                r#"<mapper><select id="f">`select_inside`</select></mapper>"#,
                "select_inside",
            ),
            (
                r#"<mapper><insert id="f">`insert_inside`</insert></mapper>"#,
                "insert_inside",
            ),
            (
                r#"<mapper><update id="f">`update_inside`</update></mapper>"#,
                "update_inside",
            ),
            (
                r#"<mapper><delete id="f">`delete_inside`</delete></mapper>"#,
                "delete_inside",
            ),
        ] {
            let code = parse(html, "f");
            assert!(code.contains("pub fn f"), "{code}");
            assert!(code.contains(needle), "{code}");
        }
    }

    #[test]
    fn select_with_nested_tags() {
        let code = parse(
            r#"<select id="f">`select * from t`<if test="true">`where a = 1`</if></select>"#,
            "f",
        );
        assert!(code.contains("select * from t"), "{code}");
        assert!(code.contains("where a = 1"), "{code}");
        assert!(code.contains("if"), "{code}");
    }

    #[test]
    fn fn_name_lookup_with_fallback_when_no_id_matches() {
        // BTreeMap is ordered by id, the wanted statement is not the first one
        let code = parse(
            r#"<select id="aaa_first">`wrong_sql`</select><select id="wanted">`right_sql`</select>"#,
            "wanted",
        );
        assert!(code.contains("right_sql"), "{code}");
        assert!(!code.contains("wrong_sql"), "{code}");

        // no id of the template equals the function name -> the single statement is used
        let code = parse(r#"<select id="other">`fallback_sql`</select>"#, "missing");
        assert!(code.contains("fallback_sql"), "{code}");
        assert!(code.contains("pub fn missing"), "{code}");
    }

    #[test]
    fn mapper_tag_node_is_parsed_directly() {
        // the outer mapper is flattened, the inner one stays an element and becomes
        // the statement selected by its id; MapperTagNode recurses into its children
        let code = parse(
            r#"<mapper><mapper id="inner"><select id="s">`inner_sql`</select></mapper></mapper>"#,
            "inner",
        );
        assert!(code.contains("pub fn inner"), "{code}");
        assert!(code.contains("inner_sql"), "{code}");
    }

    #[test]
    fn sql_tag_node_can_be_the_parsed_statement() {
        // when the id of the function is a <sql> id, SqlTagNode generates its children
        let code = parse(
            r#"<sql id="frag">`sql_fragment`</sql><select id="other">`x`</select>"#,
            "frag",
        );
        assert!(code.contains("sql_fragment"), "{code}");
    }

    #[test]
    #[should_panic(expected = "HTML not found for function")]
    fn parse_html_empty_html_panics() {
        let mut ignore = vec![];
        parse_html("", "f", &mut ignore);
    }

    #[test]
    #[should_panic(expected = "unsupported html tag")]
    fn parse_html_unsupported_tag_panics() {
        let mut ignore = vec![];
        parse_html(r#"<foobar id="f">x</foobar>"#, "f", &mut ignore);
    }

    #[test]
    #[should_panic(expected = "must have id")]
    fn sql_element_without_id_panics() {
        load_mapper_map(r#"<sql>`no id here`</sql>"#).unwrap();
    }

    #[test]
    #[should_panic(expected = "must have attr")]
    fn include_element_without_refid_panics() {
        load_mapper_map(r#"<select id="s">x<include></include></select>"#).unwrap();
    }

    #[test]
    #[should_panic(expected = "must have name")]
    fn bind_without_name_panics() {
        let mut ignore = vec![];
        parse_html(
            r#"<select id="f"><bind value="1"></bind>`x`</select>"#,
            "f",
            &mut ignore,
        );
    }

    #[test]
    #[should_panic(expected = "must have value")]
    fn bind_without_value_panics() {
        let mut ignore = vec![];
        parse_html(
            r#"<select id="f"><bind name="b"></bind>`x`</select>"#,
            "f",
            &mut ignore,
        );
    }

    // ------------------------------------------------------- parser_html: bind

    #[test]
    fn bind_tag_generates_argument_assignment() {
        let code = parse(
            r#"<select id="f"><bind name="b" value="1 + 2"></bind>`select ${b}`</select>"#,
            "f",
        );
        assert!(code.contains("rbs"), "{code}");
        assert!(code.contains("\"b\""), "{code}");
        assert!(code.contains("select {}"), "{code}");
    }

    // -------------------------------------- parser_html: break / continue tags

    #[test]
    fn break_inside_if() {
        let code = parse(
            r#"<select id="f">`a`<if test="true"><break></break></if>`b`</select>"#,
            "f",
        );
        assert!(code.contains("break"), "{code}");
        assert!(code.contains("if"), "{code}");
    }

    #[test]
    fn continue_inside_foreach() {
        let code = parse(
            r#"<select id="f">`a`<foreach collection="arg" item="v"><continue></continue>`x`</foreach>`b`</select>"#,
            "f",
        );
        assert!(code.contains("continue"), "{code}");
        assert!(code.contains("for"), "{code}");
    }

    // ------------------------------------------- parser_html: <include> logic

    #[test]
    fn include_top_level_is_replaced_with_sql_fragment() {
        let map = load_mapper_map(
            r#"<sql id="a">`from_sql`</sql><include refid="a"></include>"#,
        )
        .unwrap();
        let e = map.get("a").expect("include should resolve to the sql element");
        assert_eq!(e.tag, "sql");
        assert_eq!(e.childs[0].data, "`from_sql`");
    }

    #[test]
    fn include_nested_in_element_children_is_replaced_recursively() {
        let code = parse(
            r#"<sql id="a">`from_sql`</sql><select id="s">`select * `<if test="true"><include refid="a"></include></if></select>"#,
            "s",
        );
        assert!(code.contains("from_sql"), "{code}");
        assert!(!code.contains("include"), "{code}");
    }

    #[test]
    fn include_with_extra_children_is_replaced() {
        let code = parse(
            r#"<sql id="a">`from_sql`</sql><select id="s">`select * ` <include refid="a"><span>ignored</span></include></select>"#,
            "s",
        );
        assert!(code.contains("from_sql"), "{code}");
        assert!(!code.contains("ignored"), "{code}");
    }

    #[test]
    fn include_with_empty_children_is_replaced() {
        let code = parse(
            r#"<sql id="a">`from_sql`</sql><select id="s">`select * ` <include refid="a"></include></select>"#,
            "s",
        );
        assert!(code.contains("from_sql"), "{code}");
    }

    #[test]
    fn include_direct_generate_tokens_parses_children() {
        // normally include_replace resolves includes before code generation; if an
        // IncludeTagNode is generated directly it forwards to its children
        let mut methods = TokenStream::new();
        let mut ignore = vec![];
        let include = element(
            "include",
            "",
            &[("refid", "a")],
            vec![text("child_sql")],
        );
        let node = IncludeTagNode::from_element(&include);
        let mut context = NodeContext {
            methods: &mut methods,
            fn_name: "f",
            child_parser: |children: &[Element],
                           _m: &mut TokenStream,
                           _i: &mut Vec<String>,
                           _n: &str| {
                let count = children.len();
                quote! { let __child_count = #count; }
            },
        };
        let tokens = node.generate_tokens(&mut context, &mut ignore).to_string();
        assert!(tokens.contains("__child_count"), "{tokens}");
    }

    // ---------------------------------------- direct IncludeTagNode::process_include

    #[test]
    fn include_node_current_scheme_success() {
        let include = tag("include", &[("refid", "a")]);
        let node = IncludeTagNode::from_element(&include);
        let mut map = BTreeMap::new();
        let fragment = element("sql", "", &[("id", "a")], vec![text("frag")]);
        map.insert("a".to_string(), fragment.clone());
        assert_eq!(node.process_include(&map), fragment);
    }

    #[test]
    fn include_node_explicit_current_url_success() {
        let include = tag("include", &[("refid", "current://current?refid=a")]);
        let node = IncludeTagNode::from_element(&include);
        let mut map = BTreeMap::new();
        let fragment = element("sql", "", &[("id", "a")], vec![text("frag2")]);
        map.insert("a".to_string(), fragment.clone());
        assert_eq!(node.process_include(&map), fragment);
    }

    #[test]
    #[should_panic(expected = "cannot find element")]
    fn include_node_current_url_without_refid_query_panics() {
        // no `refid` query -> falls back to the whole refid, which is not in the map
        let include = tag("include", &[("refid", "current://current")]);
        let node = IncludeTagNode::from_element(&include);
        let map = BTreeMap::new();
        node.process_include(&map);
    }

    #[test]
    fn include_node_file_scheme_relative_path_success() {
        write_fixture();
        let include = tag(
            "include",
            &[(
                "refid",
                "file://target/html_codegen_crud_include_fixture.html?refid=a",
            )],
        );
        let node = IncludeTagNode::from_element(&include);
        let map = BTreeMap::new();
        let found = node.process_include(&map);
        assert_eq!(found.tag, "sql");
        assert_eq!(found.attrs.get("id").map(String::as_str), Some("a"));
        assert_eq!(found.childs[0].data, "frag_a");
    }

    #[test]
    fn include_node_file_scheme_absolute_path_success() {
        let path = write_fixture();
        let refid = format!("file://{}?refid=a", path.display());
        let include = tag("include", &[("refid", &refid)]);
        let node = IncludeTagNode::from_element(&include);
        let map = BTreeMap::new();
        let found = node.process_include(&map);
        assert_eq!(found.attrs.get("id").map(String::as_str), Some("a"));
    }

    #[test]
    #[should_panic(expected = "No <sql id=")]
    fn include_node_file_scheme_no_matching_sql_panics() {
        write_fixture();
        let include = tag(
            "include",
            &[(
                "refid",
                "file://target/html_codegen_crud_include_fixture.html?refid=missing",
            )],
        );
        let node = IncludeTagNode::from_element(&include);
        node.process_include(&BTreeMap::new());
    }

    #[test]
    #[should_panic(expected = "can't find file")]
    fn include_node_file_scheme_missing_file_panics() {
        let include = tag(
            "include",
            &[(
                "refid",
                "file://target/html_codegen_crud_missing_fixture.html?refid=a",
            )],
        );
        let node = IncludeTagNode::from_element(&include);
        node.process_include(&BTreeMap::new());
    }

    #[test]
    #[should_panic(expected = "No ref_id found")]
    fn include_node_file_scheme_without_refid_query_panics() {
        let include = tag(
            "include",
            &[("refid", "file://target/html_codegen_crud_include_fixture.html")],
        );
        let node = IncludeTagNode::from_element(&include);
        node.process_include(&BTreeMap::new());
    }

    #[test]
    #[should_panic(expected = "Unimplemented scheme")]
    fn include_node_unsupported_scheme_panics() {
        let include = tag("include", &[("refid", "http://example.com/a?refid=x")]);
        let node = IncludeTagNode::from_element(&include);
        node.process_include(&BTreeMap::new());
    }

    #[test]
    #[should_panic(expected = "fail!")]
    fn include_node_malformed_url_panics() {
        let include = tag("include", &[("refid", "http://[")]);
        let node = IncludeTagNode::from_element(&include);
        node.process_include(&BTreeMap::new());
    }

    // ---------------------------------------------------------- load_mapper_map

    #[test]
    fn load_mapper_map_stores_sql_and_statement_ids() {
        let map = load_mapper_map(
            r#"<sql id="a">`frag`</sql><select id="s">`select 1`</select>"#,
        )
        .unwrap();
        assert_eq!(map.get("a").unwrap().tag, "sql");
        assert_eq!(map.get("s").unwrap().tag, "select");
    }

    #[test]
    fn load_mapper_map_empty_id_branch() {
        // the include map ignores ids that are empty; the returned map still keeps
        // the element (keyed by the empty id)
        let map = load_mapper_map(r#"<select id="">`x`</select><select id="s">`y`</select>"#)
            .unwrap();
        assert!(map.contains_key(""));
        assert!(map.contains_key("s"));
    }

    #[test]
    fn load_mapper_vec_flattens_mapper_children() {
        let html = r#"<mapper><sql id="a">1</sql><select id="b">2</select></mapper><select id="c">3</select>"#;
        let from_parser = parser_load_mapper_vec(html).unwrap();
        let from_loader = load_mapper_vec(html).unwrap();
        assert_eq!(from_parser, from_loader);
        assert_eq!(from_parser.len(), 3);
        assert_eq!(from_parser[0].tag, "sql");
        assert_eq!(from_parser[2].tag, "select");
    }

    #[test]
    fn load_mapper_vec_plain_fragment() {
        let els = load_mapper_vec(r#"<select id="a">x</select><div>y</div>"#).unwrap();
        assert_eq!(els.len(), 2);
        assert_eq!(els[0].tag, "select");
        assert_eq!(els[1].tag, "div");
    }

    // ------------------------------------------------ loader_html: Element API

    #[test]
    fn element_display_text_node() {
        assert_eq!(format!("{}", text("plain text")), "plain text");
    }

    #[test]
    fn element_display_escapes_attributes_and_recurses_children() {
        let e = element(
            "if",
            "",
            &[("test", r#"a&b"<c>"#)],
            vec![text("childtext"), element("child", "", &[], vec![text("inner")])],
        );
        assert_eq!(
            format!("{}", e),
            r#"<if test="a&amp;b&quot;&lt;c&gt;">childtext<child>inner</child></if>"#
        );
    }

    #[test]
    fn element_child_strings_and_cup() {
        let e = element(
            "root",
            "",
            &[],
            vec![
                text("ab"),
                element("if", "", &[], vec![text("cd"), element("x", "", &[], vec![text("ef")])]),
            ],
        );
        assert_eq!(e.child_strings(), vec!["ab", "cd", "ef"]);
        assert_eq!(e.child_string_cup(), 6);
        assert_eq!(text("abc").child_string_cup(), 0);
    }

    #[test]
    fn load_html_skips_comments() {
        let els = load_html(r#"<!-- a comment --><select id="a">x</select>"#).unwrap();
        assert_eq!(els.len(), 1);
        assert_eq!(els[0].tag, "select");
    }

    #[test]
    fn load_html_skips_doctype_and_processing_instruction() {
        // html5ever turns `<?...?>` into a bogus comment and drops the doctype in a
        // fragment; both must be ignored by collect_elements
        let els = load_html(
            r#"<!DOCTYPE html><?xml version="1.0" encoding="UTF-8"?><div>d</div>"#,
        )
        .unwrap();
        assert!(els.iter().any(|e| e.tag == "div"), "{:?}", els);
    }

    // ---------------------------------------------------------- string_util

    #[test]
    fn find_convert_string_multiple_hash_and_dollar() {
        let list = find_convert_string("a #{x} b ${y} c #{z}");
        let got: Vec<(String, String)> = list.into_iter().collect();
        assert_eq!(
            got,
            vec![
                ("x".to_string(), "#{x}".to_string()),
                ("y".to_string(), "${y}".to_string()),
                ("z".to_string(), "#{z}".to_string()),
            ]
        );
    }

    #[test]
    fn find_convert_string_unterminated_and_plain() {
        assert!(find_convert_string("select #{").is_empty());
        assert!(find_convert_string("plain text without markers").is_empty());
        // a `$` not followed by `{` is ordinary text
        assert!(find_convert_string("price $ 1").is_empty());
    }

    #[test]
    fn count_string_num_cases() {
        assert_eq!(count_string_num("a#b#c", '#'), 2);
        assert_eq!(count_string_num("abc", '#'), 0);
        assert_eq!(count_string_num("   ", ' '), 3);
    }

    #[test]
    fn escape_attr_value_all_four_escapes() {
        assert_eq!(
            escape_attr_value(r#"&"<>"#),
            "&amp;&quot;&lt;&gt;"
        );
        assert_eq!(escape_attr_value("name = 'a'"), "name = 'a'");
        assert_eq!(escape_attr_value(""), "");
    }

    #[test]
    fn un_packing_string_quote_kinds() {
        assert_eq!(un_packing_string("'abc'"), "abc");
        assert_eq!(un_packing_string("`abc`"), "abc");
        assert_eq!(un_packing_string("\"abc\""), "abc");
        // too short for a quote pair
        assert_eq!(un_packing_string("'"), "'");
        assert_eq!(un_packing_string(""), "");
        // unquoted / mismatched quotes stay as they are
        assert_eq!(un_packing_string("abc"), "abc");
        assert_eq!(un_packing_string("'abc`"), "'abc`");
    }

    #[test]
    fn concat_str_all_cases() {
        let mut s = String::new();
        concat_str(&mut s, "abc");
        assert_eq!(s, "abc");

        let mut s = String::from("a");
        concat_str(&mut s, "abc");
        assert_eq!(s, "a abc");

        let mut s = String::from("a ");
        concat_str(&mut s, "abc");
        assert_eq!(s, "a abc");

        let mut s = String::from("a");
        concat_str(&mut s, " abc");
        assert_eq!(s, "a abc");
    }

    // -------------------------------------------- impl_fn_html panic branches

    #[test]
    #[should_panic(expected = "must have html_data")]
    fn impl_fn_html_without_sql_panics() {
        let f: syn::ItemFn = syn::parse_str("fn my_fn() {}").unwrap();
        let args = rbatis_codegen::codegen::ParseArgs { sqls: vec![] };
        let _ = rbatis_codegen::codegen::parser_html::impl_fn_html(&f, &args);
    }

    #[test]
    #[should_panic]
    fn impl_fn_html_proc_macro_bridge_panics_after_codegen() {
        // parse_html succeeds, the final `proc_macro2 -> proc_macro` conversion panics
        // outside of a real procedural macro invocation
        let f: syn::ItemFn = syn::parse_str("fn my_fn() {}").unwrap();
        let args = rbatis_codegen::codegen::ParseArgs {
            sqls: vec![syn::LitStr::new(
                r#"<select id="my_fn">`select 1`</select>"#,
                proc_macro2::Span::call_site(),
            )],
        };
        let _ = rbatis_codegen::codegen::parser_html::impl_fn_html(&f, &args);
    }
}
