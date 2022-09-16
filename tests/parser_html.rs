#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::parser_html::load_mapper_map;

    #[test]
    fn test_load_html_include_replace() {
        let datas = load_mapper_map(
            r#"
            <sql id="aaa">`and name != ''`</sql>
            <select id="custom_func">
        `select * from biz_activity`
        <include refid="aaa"></include>
    </select>"#,
        )
        .unwrap();
        assert_eq!(
            datas.get("custom_func").unwrap().childs[1].childs[0].data,
            "and name != ''"
        );
    }
}
