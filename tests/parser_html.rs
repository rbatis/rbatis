#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::parser_html::{load_mapper_map, parse_html};

    #[test]
    fn test_parse_line_feed() {
        let mut ig=vec![];
       let token= parse_html(r#"
       <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
        <mapper>
            <select id="select_by_condition">
        `select * `
        ` from biz_activity`
         </select>
        </mapper>"#,"select_by_condition",&mut ig);
        let code=token.to_string();
        println!("{}",token);
        assert_eq!(code.contains(r#"select * \n from biz_activity"#),true);
    }

    #[test]
    fn test_parse_line_feed2() {
        let mut ig=vec![];
        let token= parse_html(r#"
       <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
        <mapper>
            <select id="select_by_condition">
        `select * from biz_activity
         where`
         </select>
        </mapper>"#,"select_by_condition",&mut ig);
        let code=token.to_string();
        println!("{}",token);
        assert_eq!(code.contains(r#"select * from biz_activity\nwhere"#),true);
    }

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
            "`and name != ''`"
        );
    }
}
