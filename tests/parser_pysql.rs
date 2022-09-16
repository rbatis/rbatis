#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::parser_pysql::ParsePySql;
    use rbatis_codegen::codegen::syntax_tree::if_node::IfNode;
    use rbatis_codegen::codegen::syntax_tree::string_node::StringNode;
    use rbatis_codegen::codegen::syntax_tree::NodeType;

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
}
