#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::parser_pysql::ParsePySql;
    use rbatis_codegen::codegen::syntax_tree_pysql::if_node::IfNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::string_node::StringNode;
    use rbatis_codegen::codegen::syntax_tree_pysql::NodeType;

    #[test]
    fn test_parse_str() {
        let nodes = NodeType::parse_pysql(
            "if 1==1:
                    `1=1 and 2=2`",
        )
        .unwrap();
        println!("{:?}", nodes);
        assert_eq!(
            nodes,
            vec![NodeType::NIf(IfNode {
                childs: vec![NodeType::NString(StringNode {
                    value: "`1=1 and 2=2`".to_string()
                })],
                test: "1==1".to_string()
            })]
        );
    }

    #[test]
    fn test_parse_str2() {
        let nodes = NodeType::parse_pysql(
            "if 1==1:
                    `1=1
                    2=2`",
        )
        .unwrap();
        println!("{:?}", nodes);
        assert_eq!(
            nodes,
            vec![NodeType::NIf(IfNode {
                childs: vec![
                    NodeType::NString(StringNode {
                        value: "`1=1".to_string()
                    }),
                    NodeType::NString(StringNode {
                        value: "2=2`".to_string()
                    })
                ],
                test: "1==1".to_string()
            })]
        );
    }

    #[test]
    fn test_parse_if() {
        let nodes = NodeType::parse_pysql(
            "if 1==1:
                    1=1",
        )
        .unwrap();
        println!("{:?}", nodes);
        assert_eq!(
            nodes,
            vec![NodeType::NIf(IfNode {
                childs: vec![NodeType::NString(StringNode {
                    value: "1=1".to_string()
                })],
                test: "1==1".to_string(),
            })]
        );
    }

    #[test]
    fn test_string_node_name() {
        use rbatis_codegen::codegen::syntax_tree_pysql::Name;
        // the impl used to be written on `String` instead of on the node
        assert_eq!(StringNode::name(), "string");
    }

    #[test]
    fn test_as_html_escapes_attribute_values() {
        use rbatis_codegen::codegen::syntax_tree_pysql::bind_node::BindNode;
        use rbatis_codegen::codegen::syntax_tree_pysql::foreach_node::ForEachNode;
        use rbatis_codegen::codegen::syntax_tree_pysql::set_node::SetNode;
        use rbatis_codegen::codegen::syntax_tree_pysql::sql_node::SqlNode;
        use rbatis_codegen::codegen::syntax_tree_pysql::trim_node::TrimNode;
        use rbatis_codegen::codegen::syntax_tree_pysql::when_node::WhenNode;
        use rbatis_codegen::codegen::syntax_tree_pysql::ToHtml;

        // py_sql is turned into the HTML the html code generator reads. A `"` inside a
        // value would end the attribute if written raw, so it is encoded as `&quot;`
        // and decoded again before the expression is translated
        let if_node = IfNode {
            childs: vec![],
            test: "name == \"a\"".to_string(),
        };
        assert_eq!(
            if_node.as_html(),
            r#"<if test="name == &quot;a&quot;"></if>"#
        );

        let when_node = WhenNode {
            childs: vec![],
            test: "name != \"a\"".to_string(),
        };
        assert_eq!(
            when_node.as_html(),
            r#"<when test="name != &quot;a&quot;"></when>"#
        );

        let bind_node = BindNode {
            name: "x".to_string(),
            value: "a == b".to_string(),
        };
        assert_eq!(
            bind_node.as_html(),
            r#"<bind name="x" value="a == b"></bind>"#
        );

        let for_node = ForEachNode {
            childs: vec![],
            collection: "list".to_string(),
            index: "k".to_string(),
            item: "v".to_string(),
        };
        assert_eq!(
            for_node.as_html(),
            r#"<foreach collection="list" index="k" item="v" ></foreach>"#
        );

        let trim_node = TrimNode {
            childs: vec![],
            start: " and ".to_string(),
            end: ",".to_string(),
        };
        assert_eq!(
            trim_node.as_html(),
            r#"<trim prefixOverrides=" and " suffixOverrides=","></trim>"#
        );

        let sql_node = SqlNode {
            id: "frag".to_string(),
            childs: vec![],
        };
        assert_eq!(sql_node.as_html(), r#"<sql id="frag"></sql>"#);

        let set_node = SetNode {
            childs: vec![],
            collection: "list".to_string(),
            skips: "id".to_string(),
            skip_null: None,
        };
        assert_eq!(
            set_node.as_html(),
            r#"<set collection="list" skips="id"></set>"#
        );
    }

    #[test]
    fn test_parse_bind_with_eq_in_value() {
        // the value of a bind may contain a `=` (here a comparison). Splitting the
        // expression at the first `=` keeps name='flag' and value='id == 1'
        let nodes = NodeType::parse_pysql("bind flag = id == 1:\n  `${flag}`").unwrap();
        println!("{:?}", nodes);
        match &nodes[0] {
            NodeType::NBind(bind) => {
                assert_eq!(bind.name, "flag");
                assert_eq!(bind.value, "id == 1");
            }
            other => panic!("expected a bind node, got: {:?}", other),
        }
    }

    #[test]
    fn test_parse_bind_without_value_is_an_error() {
        // no `=` in the bind expression
        let result = NodeType::parse_pysql("bind flag:\n  `x`");
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bad_for_is_an_error() {
        // a `for` line with more than the `key,item` pair used to panic while the macro
        // expanded the template, now the parser reports it as an error
        let result = NodeType::parse_pysql("for a,b,c in list:\n  `x`");
        println!("{:?}", result);
        assert!(result.is_err());
    }
}
