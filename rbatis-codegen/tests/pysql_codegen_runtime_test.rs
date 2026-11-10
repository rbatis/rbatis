//! Runtime coverage for the py_sql parser and syntax tree.
//!
//! Targets:
//! * `rbatis-codegen/src/codegen/parser_pysql.rs`
//! * `rbatis-codegen/src/codegen/syntax_tree_pysql/*.rs` (nodes, `to_html.rs`, `error.rs`)
//!
//! The tests live in the `rbatis` package because that is the binary the coverage
//! tool measures; they call the public `rbatis_codegen` API at runtime.

#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::parser_pysql::{impl_fn_py, ParsePySql};
    use rbatis_codegen::codegen::syntax_tree_pysql::bind_node::BindNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::break_node::BreakNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::choose_node::ChooseNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::continue_node::ContinueNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::error::Error as PySqlError;
    use rbatis_codegen::codegen::syntax_tree_pysql::foreach_node::ForEachNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::if_node::IfNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::otherwise_node::OtherwiseNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::set_node::SetNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::sql_node::SqlNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::string_node::StringNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::to_html::to_html_mapper;
    use rbatis_codegen::codegen::syntax_tree_pysql::trim_node::TrimNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::when_node::WhenNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::where_node::WhereNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::{DefaultName, Name, NodeType, ToHtml};
    use rbatis_codegen::codegen::ParseArgs;
    use std::error::Error as StdError;

    fn s(value: &str) -> NodeType {
        NodeType::NString(StringNode {
            value: value.to_string(),
        })
    }

    fn parse(sql: &str) -> Vec<NodeType> {
        NodeType::parse_pysql(sql).expect("parse_pysql should succeed")
    }

    fn parse_err(sql: &str) -> PySqlError {
        NodeType::parse_pysql(sql).expect_err("parse_pysql should fail")
    }

    fn item_fn() -> syn::ItemFn {
        syn::parse_str::<syn::ItemFn>(
            "fn demo(arg: rbs::Value) -> rbs::Value { rbatis::impled!() }",
        )
        .unwrap()
    }

    fn parse_args(sql: &str) -> ParseArgs {
        ParseArgs {
            sqls: vec![syn::LitStr::new(sql, proc_macro2::Span::call_site())],
        }
    }

    // ---------------------------------------------------------------- plain text

    #[test]
    fn parse_plain_string_and_indented_continuation() {
        // a non-block line may still collect more-indented lines as children
        assert_eq!(
            parse("select *\n  from t"),
            vec![s("select *"), s("from t")]
        );
    }

    #[test]
    fn parse_skips_blank_lines() {
        assert_eq!(
            parse("\nselect 1\n\nselect 2"),
            vec![s("select 1"), s("select 2")]
        );
    }

    #[test]
    fn parse_whitespace_only_line_becomes_empty_string() {
        // whitespace-only lines are not empty, so they are parsed as a string node
        assert_eq!(
            parse("select 1\n   \nselect 2"),
            vec![s("select 1"), s(""), s("select 2")]
        );
    }

    #[test]
    fn parse_comment_lines_are_dropped() {
        assert_eq!(
            parse("// top level comment\nselect 1\n  // nested comment\n  from t"),
            vec![s("select 1"), s("from t")]
        );
    }

    // --------------------------------------------------------------------- if

    #[test]
    fn parse_if_node() {
        assert_eq!(
            parse("if a == 1:\n  and b = 2"),
            vec![NodeType::NIf(IfNode {
                childs: vec![s("and b = 2")],
                test: "a == 1".to_string(),
            })]
        );
    }

    #[test]
    fn parse_nested_if_nodes() {
        assert_eq!(
            parse("if a:\n  if b:\n    c"),
            vec![NodeType::NIf(IfNode {
                childs: vec![NodeType::NIf(IfNode {
                    childs: vec![s("c")],
                    test: "b".to_string(),
                })],
                test: "a".to_string(),
            })]
        );
    }

    #[test]
    fn parse_if_with_quoted_expression() {
        assert_eq!(
            parse("if name != null && status == \"x\":\n  `ok`"),
            vec![NodeType::NIf(IfNode {
                childs: vec![s("`ok`")],
                test: "name != null && status == \"x\"".to_string(),
            })]
        );
    }

    #[test]
    fn parse_multiline_if_test_is_not_supported() {
        // the parser is line oriented: a continuation line of a test expression is
        // parsed on its own and rejected as an unknown tag
        assert!(NodeType::parse_pysql("if a == 1\n  && b == 2:\n  x").is_err());
    }

    // -------------------------------------------------------------------- for

    #[test]
    fn parse_for_each_simple() {
        assert_eq!(
            parse("for item in list:\n  #{item}"),
            vec![NodeType::NForEach(ForEachNode {
                childs: vec![s("#{item}")],
                collection: "list".to_string(),
                index: String::new(),
                item: "item".to_string(),
            })]
        );
    }

    #[test]
    fn parse_for_each_with_index() {
        assert_eq!(
            parse("for key,item in user_map:\n  x"),
            vec![NodeType::NForEach(ForEachNode {
                childs: vec![s("x")],
                collection: "user_map".to_string(),
                index: "key".to_string(),
                item: "item".to_string(),
            })]
        );
    }

    #[test]
    fn parse_for_each_whitespace_variants() {
        assert_eq!(
            parse("for  key , item  in  list:\n  x"),
            vec![NodeType::NForEach(ForEachNode {
                childs: vec![s("x")],
                collection: "list".to_string(),
                index: "key".to_string(),
                item: "item".to_string(),
            })]
        );
    }

    #[test]
    fn parse_for_errors() {
        let missing_in = parse_err("for item:\n  x");
        assert!(missing_in.to_string().contains("parser express fail"));

        let too_many = parse_err("for a,b,c in list:\n  x");
        assert!(too_many.to_string().contains("for node must be"));
    }

    // ------------------------------------------------------------------- trim

    #[test]
    fn parse_trim_quoted() {
        assert_eq!(
            parse("trim ',':\n  a"),
            vec![NodeType::NTrim(TrimNode {
                childs: vec![s("a")],
                start: ",".to_string(),
                end: ",".to_string(),
            })]
        );
        assert_eq!(
            parse("trim `,`:\n  a"),
            vec![NodeType::NTrim(TrimNode {
                childs: vec![s("a")],
                start: ",".to_string(),
                end: ",".to_string(),
            })]
        );
    }

    #[test]
    fn parse_trim_start_and_end_variants() {
        assert_eq!(
            parse("trim end=')':\n  a"),
            vec![NodeType::NTrim(TrimNode {
                childs: vec![s("a")],
                start: String::new(),
                end: ")".to_string(),
            })]
        );
        assert_eq!(
            parse("trim start='(',end=')':\n  a"),
            vec![NodeType::NTrim(TrimNode {
                childs: vec![s("a")],
                start: "(".to_string(),
                end: ")".to_string(),
            })]
        );
        // space after the comma is trimmed as well
        assert_eq!(
            parse("trim start='(', end=')':\n  a"),
            vec![NodeType::NTrim(TrimNode {
                childs: vec![s("a")],
                start: "(".to_string(),
                end: ")".to_string(),
            })]
        );
        assert_eq!(
            parse("trim start='ab':\n  a"),
            vec![NodeType::NTrim(TrimNode {
                childs: vec![s("a")],
                start: "ab".to_string(),
                end: String::new(),
            })]
        );
    }

    #[test]
    fn parse_trim_errors() {
        // neither quoted nor written as start=/end=
        let plain = parse_err("trim foo:\n  a");
        assert!(plain.to_string().contains("express trim node error"));

        // unknown part in a start=/end= list
        let unknown_part = parse_err("trim start='(',foo='x':\n  a");
        assert!(unknown_part.to_string().contains("express trim node error"));

        // the comma inside the quoted value is treated as an attribute separator,
        // leaving a dangling `'` part -> error
        assert!(NodeType::parse_pysql("trim start=',':\n  a").is_err());
    }

    // ----------------------------------------------------------------- choose

    #[test]
    fn parse_choose_full() {
        let sql = "choose:\n  when a == 1:\n    x\n  when b == 2:\n    y\n  otherwise:\n    z";
        assert_eq!(
            parse(sql),
            vec![NodeType::NChoose(ChooseNode {
                when_nodes: vec![
                    NodeType::NWhen(WhenNode {
                        childs: vec![s("x")],
                        test: "a == 1".to_string(),
                    }),
                    NodeType::NWhen(WhenNode {
                        childs: vec![s("y")],
                        test: "b == 2".to_string(),
                    }),
                ],
                otherwise_node: Some(Box::new(NodeType::NOtherwise(OtherwiseNode {
                    childs: vec![s("z")],
                }))),
            })]
        );
    }

    #[test]
    fn parse_choose_short_otherwise() {
        assert_eq!(
            parse("choose:\n  when a:\n    x\n  _:\n    y"),
            vec![NodeType::NChoose(ChooseNode {
                when_nodes: vec![NodeType::NWhen(WhenNode {
                    childs: vec![s("x")],
                    test: "a".to_string(),
                })],
                otherwise_node: Some(Box::new(NodeType::NOtherwise(OtherwiseNode {
                    childs: vec![s("y")],
                }))),
            })]
        );
    }

    #[test]
    fn parse_choose_inline_parts() {
        // the "choose: when x:" same-line form is split on ": " and rebuilt
        assert_eq!(
            parse("choose: when a:\n  x"),
            vec![NodeType::NChoose(ChooseNode {
                when_nodes: vec![NodeType::NWhen(WhenNode {
                    childs: vec![s("x")],
                    test: "a".to_string(),
                })],
                otherwise_node: None,
            })]
        );

        // three parts: choose -> when -> otherwise
        assert_eq!(
            parse("choose: when a: otherwise:\n  x"),
            vec![NodeType::NChoose(ChooseNode {
                when_nodes: vec![NodeType::NWhen(WhenNode {
                    childs: vec![NodeType::NOtherwise(OtherwiseNode { childs: vec![s("x")] })],
                    test: "a".to_string(),
                })],
                otherwise_node: None,
            })]
        );
    }

    #[test]
    fn parse_choose_rejects_non_when_children() {
        let err = parse_err("choose:\n  plain");
        assert!(err
            .to_string()
            .contains("choose node' child must be when and otherwise nodes"));
    }

    // ------------------------------------------------------------------- bind

    #[test]
    fn parse_bind_and_let_nodes() {
        // the indented body of a bind/let line is parsed but not kept by BindNode
        assert_eq!(
            parse("bind x = 'v':\n  ${x}"),
            vec![NodeType::NBind(BindNode {
                name: "x".to_string(),
                value: "'v'".to_string(),
            })]
        );
        assert_eq!(
            parse("let y = 1:\n  ${y}"),
            vec![NodeType::NBind(BindNode {
                name: "y".to_string(),
                value: "1".to_string(),
            })]
        );
    }

    #[test]
    fn parse_bind_value_containing_equals() {
        assert_eq!(
            parse("bind flag = id == 1:\n  x"),
            vec![NodeType::NBind(BindNode {
                name: "flag".to_string(),
                value: "id == 1".to_string(),
            })]
        );
    }

    #[test]
    fn parse_bind_error() {
        let err = parse_err("bind flag:\n  x");
        assert!(err.to_string().contains("parser bind express fail"));
    }

    // -------------------------------------------------------------------- set

    #[test]
    fn parse_set_all_attributes() {
        assert_eq!(
            parse("set collection=\"m\",skips=\"id\",skip_null=true:\n  a"),
            vec![NodeType::NSet(SetNode {
                childs: vec![s("a")],
                collection: "m".to_string(),
                skips: "id".to_string(),
                skip_null: Some(true),
            })]
        );
    }

    #[test]
    fn parse_set_skip_null_variants() {
        let sql = "set skip_null='false':\n  a";
        match &parse(sql)[0] {
            NodeType::NSet(n) => {
                assert_eq!(n.skip_null, Some(false));
                assert!(n.collection.is_empty());
                assert!(n.skips.is_empty());
            }
            other => panic!("expected set node, got {:?}", other),
        }

        match &parse("set skip_null=TRUE:\n  a")[0] {
            NodeType::NSet(n) => assert_eq!(n.skip_null, Some(true)),
            other => panic!("expected set node, got {:?}", other),
        }

        // empty value -> None
        match &parse("set skip_null=:\n  a")[0] {
            NodeType::NSet(n) => assert_eq!(n.skip_null, None),
            other => panic!("expected set node, got {:?}", other),
        }
        // quoted empty value -> None
        match &parse("set skip_null='':\n  a")[0] {
            NodeType::NSet(n) => assert_eq!(n.skip_null, None),
            other => panic!("expected set node, got {:?}", other),
        }
    }

    #[test]
    fn parse_set_collection_quote_variants() {
        assert_eq!(
            parse("set collection=\"m\":\n  a"),
            vec![NodeType::NSet(SetNode {
                childs: vec![s("a")],
                collection: "m".to_string(),
                skips: String::new(),
                skip_null: None,
            })]
        );
        assert_eq!(
            parse("set collection=m:\n  a"),
            vec![NodeType::NSet(SetNode {
                childs: vec![s("a")],
                collection: "m".to_string(),
                skips: String::new(),
                skip_null: None,
            })]
        );
        // malformed (only the opening quote) -> quotes are kept
        match &parse("set collection='m:\n  a")[0] {
            NodeType::NSet(n) => assert_eq!(n.collection, "'m"),
            other => panic!("expected set node, got {:?}", other),
        }
        // a single quote char is not stripped (len < 2)
        match &parse("set collection=':\n  a")[0] {
            NodeType::NSet(n) => assert_eq!(n.collection, "'"),
            other => panic!("expected set node, got {:?}", other),
        }
        // attributes separated by spaces are one key/value pair; strip_quotes
        // removes only the outer pair of quotes
        match &parse("set collection=\"m\" skips=\"id\" skip_null=\"true\":\n  a")[0] {
            NodeType::NSet(n) => {
                assert_eq!(n.collection, "m\" skips=\"id\" skip_null=\"true");
                assert!(n.skips.is_empty());
                assert_eq!(n.skip_null, None);
            }
            other => panic!("expected set node, got {:?}", other),
        }
    }

    #[test]
    fn parse_set_empty_parts_are_skipped() {
        assert_eq!(
            parse("set ,:\n  a"),
            vec![NodeType::NSet(SetNode {
                childs: vec![s("a")],
                collection: String::new(),
                skips: String::new(),
                skip_null: None,
            })]
        );
    }

    #[test]
    fn parse_set_errors() {
        // part without '='
        let malformed = parse_err("set foo:\n  a");
        assert!(malformed.to_string().contains("Malformed attribute"));

        // unknown attribute name
        let unknown = parse_err("set foo=bar:\n  a");
        assert!(unknown.to_string().contains("Unknown attribute 'foo'"));

        // invalid boolean for skip_null
        let invalid_bool = parse_err("set skip_null=maybe:\n  a");
        assert!(invalid_bool
            .to_string()
            .contains("Invalid boolean value for skip_null"));

        // a comma in the quoted skips value splits it like an attribute separator
        assert!(NodeType::parse_pysql("set collection=\"m\",skips=\"id,name\":\n  a").is_err());
        assert!(NodeType::parse_pysql(
            "set collection=\"m\" skips=\"id,name\" skip_null=\"true\":\n  a"
        )
        .is_err());

        // malformed quoted boolean
        assert!(NodeType::parse_pysql("set skip_null='true:\n  a").is_err());
    }

    // ---------------------------------------------------- where / continue / break

    #[test]
    fn parse_where_continue_break() {
        assert_eq!(
            parse("where:\n  a"),
            vec![NodeType::NWhere(WhereNode { childs: vec![s("a")] })]
        );
        assert_eq!(
            parse("continue:"),
            vec![NodeType::NContinue(ContinueNode {})]
        );
        assert_eq!(parse("break:"), vec![NodeType::NBreak(BreakNode {})]);
    }

    // -------------------------------------------------------------------- sql

    #[test]
    fn parse_sql_nodes() {
        assert_eq!(
            parse("sql id='x':\n  a"),
            vec![NodeType::NSql(SqlNode {
                id: "x".to_string(),
                childs: vec![s("a")],
            })]
        );
        assert_eq!(
            parse("sql id=\"y\":\n  a"),
            vec![NodeType::NSql(SqlNode {
                id: "y".to_string(),
                childs: vec![s("a")],
            })]
        );
        assert_eq!(
            parse("sql id='':\n  a"),
            vec![NodeType::NSql(SqlNode {
                id: String::new(),
                childs: vec![s("a")],
            })]
        );
    }

    #[test]
    fn parse_sql_errors() {
        let unquoted = parse_err("sql id=x:\n  a");
        assert!(unquoted.to_string().contains("sql id value need quotes"));

        let missing_id = parse_err("sql foo:\n  a");
        assert!(missing_id.to_string().contains("need id param"));

        let half_quote = parse_err("sql id='x:\n  a");
        assert!(half_quote.to_string().contains("sql id value need quotes"));
    }

    // ----------------------------------------------------------- unknown / misc

    #[test]
    fn parse_unknown_tag_is_an_error() {
        let err = parse_err("unknown_tag:\n  a");
        assert!(err.to_string().contains("unknown tag"));
    }

    #[test]
    fn parse_otherwise_long_and_short_forms() {
        assert_eq!(
            parse("otherwise:\n  a"),
            vec![NodeType::NOtherwise(OtherwiseNode { childs: vec![s("a")] })]
        );
        assert_eq!(
            parse("_:\n  a"),
            vec![NodeType::NOtherwise(OtherwiseNode { childs: vec![s("a")] })]
        );
    }

    // ------------------------------------------------------------- node names

    #[test]
    fn node_names_and_default_names() {
        assert_eq!(StringNode::name(), "string");
        assert_eq!(IfNode::name(), "if");
        assert_eq!(TrimNode::name(), "trim");
        assert_eq!(ForEachNode::name(), "for");
        assert_eq!(ChooseNode::name(), "choose");
        assert_eq!(OtherwiseNode::name(), "otherwise");
        assert_eq!(OtherwiseNode::default_name(), "_");
        assert_eq!(WhenNode::name(), "when");
        assert_eq!(BindNode::name(), "bind");
        assert_eq!(BindNode::default_name(), "let");
        assert_eq!(SetNode::name(), "set");
        assert_eq!(WhereNode::name(), "where");
        assert_eq!(ContinueNode::name(), "continue");
        assert_eq!(BreakNode::name(), "break");
        assert_eq!(SqlNode::name(), "sql");
    }

    // ---------------------------------------------------------------- as_html

    #[test]
    fn string_node_as_html_wraps_and_preserves_backticks() {
        assert_eq!(StringNode { value: "select 1".into() }.as_html(), "`select 1`");
        assert_eq!(
            StringNode {
                value: "`raw  text`".into()
            }
            .as_html(),
            "`raw  text`"
        );
        // plain strings are not attribute values, quotes are kept verbatim
        assert_eq!(
            StringNode {
                value: "a \"b\" & c".into()
            }
            .as_html(),
            "`a \"b\" & c`"
        );
    }

    #[test]
    fn node_type_as_html_covers_every_variant() {
        let child = s("a");
        let cases: Vec<(NodeType, &str)> = vec![
            (s("a"), "`a`"),
            (
                NodeType::NIf(IfNode {
                    childs: vec![child.clone()],
                    test: "x".into(),
                }),
                "<if test=\"x\">`a`</if>",
            ),
            (
                NodeType::NTrim(TrimNode {
                    childs: vec![child.clone()],
                    start: "(".into(),
                    end: ")".into(),
                }),
                "<trim prefixOverrides=\"(\" suffixOverrides=\")\">`a`</trim>",
            ),
            (
                NodeType::NForEach(ForEachNode {
                    childs: vec![child.clone()],
                    collection: "c".into(),
                    index: "i".into(),
                    item: "it".into(),
                }),
                "<foreach collection=\"c\" index=\"i\" item=\"it\" >`a`</foreach>",
            ),
            (
                NodeType::NChoose(ChooseNode {
                    when_nodes: vec![NodeType::NWhen(WhenNode {
                        childs: vec![child.clone()],
                        test: "w".into(),
                    })],
                    otherwise_node: Some(Box::new(NodeType::NOtherwise(OtherwiseNode {
                        childs: vec![child.clone()],
                    }))),
                }),
                "<choose><when test=\"w\">`a`</when><otherwise>`a`</otherwise></choose>",
            ),
            (
                NodeType::NOtherwise(OtherwiseNode {
                    childs: vec![child.clone()],
                }),
                "<otherwise>`a`</otherwise>",
            ),
            (
                NodeType::NWhen(WhenNode {
                    childs: vec![child.clone()],
                    test: "w".into(),
                }),
                "<when test=\"w\">`a`</when>",
            ),
            (
                NodeType::NBind(BindNode {
                    name: "n".into(),
                    value: "v".into(),
                }),
                "<bind name=\"n\" value=\"v\"></bind>",
            ),
            (
                NodeType::NSet(SetNode {
                    childs: vec![child.clone()],
                    collection: "c".into(),
                    skips: "s".into(),
                    skip_null: Some(true),
                }),
                "<set collection=\"c\" skips=\"s\" skip_null=\"true\">`a`</set>",
            ),
            (
                NodeType::NWhere(WhereNode {
                    childs: vec![child.clone()],
                }),
                "<where>`a`</where>",
            ),
            (
                NodeType::NContinue(ContinueNode {}),
                "<continue></continue>",
            ),
            (NodeType::NBreak(BreakNode {}), "<break></break>"),
            (
                NodeType::NSql(SqlNode {
                    id: "i".into(),
                    childs: vec![child.clone()],
                }),
                "<sql id=\"i\">`a`</sql>",
            ),
        ];
        for (node, expected) in cases {
            assert_eq!(node.as_html(), expected);
        }
    }

    #[test]
    fn choose_node_as_html_without_otherwise() {
        let node = ChooseNode {
            when_nodes: vec![],
            otherwise_node: None,
        };
        assert_eq!(node.as_html(), "<choose></choose>");
    }

    #[test]
    fn set_node_as_html_covers_every_attribute_branch() {
        // all optional attributes absent
        assert_eq!(
            SetNode {
                childs: vec![],
                collection: String::new(),
                skips: String::new(),
                skip_null: None,
            }
            .as_html(),
            "<set></set>"
        );
        // collection + skips + skip_null=false
        assert_eq!(
            SetNode {
                childs: vec![s("a")],
                collection: "m".into(),
                skips: "id".into(),
                skip_null: Some(false),
            }
            .as_html(),
            "<set collection=\"m\" skips=\"id\" skip_null=\"false\">`a`</set>"
        );
        // only skip_null=true
        assert_eq!(
            SetNode {
                childs: vec![],
                collection: String::new(),
                skips: String::new(),
                skip_null: Some(true),
            }
            .as_html(),
            "<set skip_null=\"true\"></set>"
        );
    }

    #[test]
    fn nodes_escape_html_attribute_values() {
        let bind = BindNode {
            name: "a\"b<c>".into(),
            value: "x&y".into(),
        };
        assert_eq!(
            bind.as_html(),
            "<bind name=\"a&quot;b&lt;c&gt;\" value=\"x&amp;y\"></bind>"
        );

        let if_node = IfNode {
            childs: vec![],
            test: "a < b && c > d".into(),
        };
        assert_eq!(
            if_node.as_html(),
            "<if test=\"a &lt; b &amp;&amp; c &gt; d\"></if>"
        );

        let when_node = WhenNode {
            childs: vec![],
            test: "x == \"y\"".into(),
        };
        assert_eq!(
            when_node.as_html(),
            "<when test=\"x == &quot;y&quot;\"></when>"
        );

        let foreach = ForEachNode {
            childs: vec![],
            collection: "a&b".into(),
            index: "i<1".into(),
            item: "v\"2\"".into(),
        };
        assert_eq!(
            foreach.as_html(),
            "<foreach collection=\"a&amp;b\" index=\"i&lt;1\" item=\"v&quot;2&quot;\" ></foreach>"
        );

        let trim = TrimNode {
            childs: vec![],
            start: "&".into(),
            end: "\"".into(),
        };
        assert_eq!(
            trim.as_html(),
            "<trim prefixOverrides=\"&amp;\" suffixOverrides=\"&quot;\"></trim>"
        );

        let sql = SqlNode {
            id: "a&b".into(),
            childs: vec![],
        };
        assert_eq!(sql.as_html(), "<sql id=\"a&amp;b\"></sql>");

        let set = SetNode {
            childs: vec![],
            collection: "a&b".into(),
            skips: "c\"d".into(),
            skip_null: None,
        };
        assert_eq!(
            set.as_html(),
            "<set collection=\"a&amp;b\" skips=\"c&quot;d\"></set>"
        );
    }

    #[test]
    fn vec_node_type_as_html_concatenates_children() {
        let nodes = vec![
            NodeType::NContinue(ContinueNode {}),
            NodeType::NBreak(BreakNode {}),
        ];
        assert_eq!(nodes.as_html(), "<continue></continue><break></break>");
        assert_eq!(Vec::<NodeType>::new().as_html(), "");
    }

    #[test]
    fn to_html_mapper_select_and_update() {
        let nodes = vec![NodeType::NContinue(ContinueNode {})];
        assert_eq!(
            to_html_mapper(&nodes, true, "f"),
            "<mapper><select id=\"f\"><continue></continue></select></mapper>"
        );
        assert_eq!(
            to_html_mapper(&nodes, false, "f"),
            "<mapper><update id=\"f\"><continue></continue></update></mapper>"
        );
        assert_eq!(
            to_html_mapper(&vec![], true, "g"),
            "<mapper><select id=\"g\"></select></mapper>"
        );
    }

    // ------------------------------------------------------------------ error

    #[test]
    fn error_module_display_from_clone_and_trait() {
        // From<&str> + Display
        let from_str = PySqlError::from("oops");
        assert_eq!(from_str.to_string(), "oops");

        // From<String>
        let from_string = PySqlError::from(String::from("boom"));
        assert_eq!(format!("{}", from_string), "boom");

        // From<io::Error>
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "iofail");
        let from_io = PySqlError::from(io_err);
        assert!(from_io.to_string().contains("iofail"));

        // From<&dyn std::error::Error>
        let dyn_io = std::io::Error::new(std::io::ErrorKind::Other, "dynfail");
        let dyn_ref: &dyn StdError = &dyn_io;
        let from_dyn = PySqlError::from(dyn_ref);
        assert!(from_dyn.to_string().contains("dynfail"));

        // Clone
        let cloned = from_string.clone();
        assert_eq!(cloned.to_string(), "boom");

        // Clone::clone_from
        let mut target = PySqlError::from("a");
        target.clone_from(&from_string);
        assert_eq!(target.to_string(), "boom");

        // StdError impl (Display + Debug are required by the trait)
        let trait_obj: &dyn StdError = &from_str;
        assert_eq!(trait_obj.to_string(), "oops");
        assert!(format!("{:?}", from_str).contains("oops"));

        // Result alias
        let ok: rbatis_codegen::codegen::syntax_tree_pysql::error::Result<()> = Ok(());
        assert!(ok.is_ok());
    }

    // --------------------------------------------------------------- impl_fn_py
    //
    // `impl_fn_py` returns `proc_macro::TokenStream`; the final `.into()` panics
    // outside a real procedural macro. Everything before the conversion runs.

    #[test]
    #[should_panic]
    fn impl_fn_py_select_template_panics_at_proc_macro_bridge() {
        let f = item_fn();
        let args = parse_args("select * from t where id = #{id}");
        let _ = impl_fn_py(&f, &args);
    }

    #[test]
    #[should_panic]
    fn impl_fn_py_update_template_panics_at_proc_macro_bridge() {
        let f = item_fn();
        let args = parse_args("update t set a = #{a} where id = #{id}");
        let _ = impl_fn_py(&f, &args);
    }

    #[test]
    #[should_panic(expected = "parse py_sql fail")]
    fn impl_fn_py_invalid_template_panics() {
        let f = item_fn();
        let args = parse_args("unknown_tag:\n  x");
        let _ = impl_fn_py(&f, &args);
    }

    #[test]
    fn impl_fn_py_select_reaches_the_proc_macro_conversion() {
        // make sure the panic of the template test above happens at the final
        // `proc_macro2 -> proc_macro` conversion, not earlier in parse_html
        let result = std::panic::catch_unwind(|| {
            let f = item_fn();
            let args = parse_args("select * from t where id = #{id}");
            let _ = impl_fn_py(&f, &args);
        });
        let payload = result.expect_err("impl_fn_py must panic outside a proc macro");
        let message = payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();
        let lower = message.to_lowercase();
        assert!(
            lower.contains("procedural macro") || lower.contains("proc_macro"),
            "unexpected panic message: {}",
            message
        );
    }
}
