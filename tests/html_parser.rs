#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::parser_html::load_html_include_replace;

    #[test]
    fn test_include_sql() {
        let datas = load_html_include_replace(
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