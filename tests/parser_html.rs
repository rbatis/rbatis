#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::parser_html::{load_mapper_map, parse_html};

    #[test]
    fn test_parse_line_feed() {
        let mut ig = vec![];
        let token = parse_html(
            r#"
       <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
        <mapper>
            <select id="select_by_condition">
        `select * `
        ` from biz_activity`
         </select>
        </mapper>"#,
            "select_by_condition",
            &mut ig,
        );
        let code = token.to_string();
        println!("{}", token);
        assert!(!code.contains(r#"`"#));
    }

    #[test]
    fn test_parse_line_feed2() {
        let mut ig = vec![];
        let token = parse_html(
            r#"
       <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
        <mapper>
            <select id="select_by_condition">
        `select * from biz_activity
         where`
         </select>
        </mapper>"#,
            "select_by_condition",
            &mut ig,
        );
        let code = token.to_string();
        println!("{}", token);
        assert!(!code.contains(r#"`"#));
    }

    #[test]
    fn test_parse_line_feed3() {
        let mut ig = vec![];
        let token = parse_html(
            r#"
       <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
        <mapper>
            <select id="select_by_condition">
            `select * from biz_activity
         where`
        `id = 1
         and id = 2`
         </select>
        </mapper>"#,
            "select_by_condition",
            &mut ig,
        );
        let code = token.to_string();
        println!("{}", token);
        assert!(!code.contains(r#"`"#));
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

    #[test]
    fn test_html_include_issue_628() {
        let mut ig = vec![];
        let token = parse_html(
            r#"
        <mapper>
            <sql id="table_name">` activity `</sql>
            <select id="select_by_condition">
        `select * from <include refid="table_name"></include>`
         </select>
        </mapper>"#,
            "select_by_condition",
            &mut ig,
        );
        let code = token.to_string();
        println!("{}", token);
        // The `<include>` must be expanded into the referenced `<sql>` fragment,
        // otherwise the generated SQL would silently lose the table name (issue #628).
        assert!(
            code.contains("select * from") && code.contains("activity"),
            "include should be expanded into the referenced sql fragment, got: {}",
            code
        );
        assert!(
            !code.contains("include"),
            "unresolved <include> should not remain in the generated code, got: {}",
            code
        );
    }

    #[test]
    fn test_load_mapper_vec_is_the_same_as_loader_html() {
        use rbatis_codegen::codegen::loader_html;
        use rbatis_codegen::codegen::parser_html;

        let html = r#"
        <mapper>
            <sql id="a">1</sql>
            <select id="b">2</select>
        </mapper>
        <select id="c">3</select>"#;

        // the copy this module used to carry behaved differently from the one of
        // loader_html (it is the one the <include> tag and the macros use)
        let from_parser = parser_html::load_mapper_vec(html).unwrap();
        let from_loader = loader_html::load_mapper_vec(html).unwrap();
        assert_eq!(from_parser, from_loader);
        // every <mapper> child becomes one element, a tag outside a mapper as well
        assert_eq!(from_parser.len(), 3);
        assert_eq!(from_parser[0].tag, "sql");
        assert_eq!(from_parser[2].tag, "select");
    }

    #[test]
    fn test_element_display_escapes_attribute_value() {
        use rbatis_codegen::codegen::loader_html::load_mapper_vec;

        // writing an Element back out and reading it again must give the same
        // attribute value. A `"` in the value would end the attribute and the value
        // would come back truncated
        let html = r#"<if test="name == 'a'">x</if>"#;
        let element = &load_mapper_vec(html).unwrap()[0];
        let written = format!("{}", element);
        println!("written={}", written);
        assert_eq!(written, r#"<if test="name == 'a'">x</if>"#);

        let again = &load_mapper_vec(&written).unwrap()[0];
        assert_eq!(
            again.attrs.get("test").map(String::as_str),
            Some("name == 'a'")
        );

        // an Element whose attribute value carries a `"`. Without the escape the value
        // would be read back as `name == ` (everything after the second `"` is lost)
        let element = rbatis_codegen::codegen::loader_html::Element {
            tag: "if".to_string(),
            line: 0,
            line_text: String::new(),
            data: String::new(),
            attrs: std::collections::HashMap::from([(
                "test".to_string(),
                "name == \"a\"".to_string(),
            )]),
            childs: vec![],
        };
        let written = format!("{}", element);
        println!("written={}", written);
        assert_eq!(written, r#"<if test="name == &quot;a&quot;"></if>"#);

        let reparsed = load_mapper_vec(&written).unwrap();
        // the parser decodes `&quot;` back to `"`, so the expression is unchanged
        assert_eq!(
            reparsed[0].attrs.get("test").map(String::as_str),
            Some("name == \"a\"")
        );
    }

    #[test]
    fn test_parse_html_uses_element_with_the_fn_id() {
        use rbatis_codegen::codegen::parser_html::parse_html;

        // load_mapper_map keeps the statements in a map ordered by their id. Taking the
        // first entry instead of the one with the id of the function compiled the wrong
        // statement
        let html = r#"<select id="aaa_first_by_id">`wrong sql`</select><select id="wanted">`right sql`</select>"#;
        let code = parse_html(html, "wanted", &mut vec![]).to_string();
        println!("{}", code);
        assert!(code.contains("right sql"), "got: {}", code);
        assert!(!code.contains("wrong sql"), "got: {}", code);
    }
}
