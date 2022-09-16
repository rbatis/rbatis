#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::loader_html::load_html;

    #[test]
    fn test_parser() {
        let nodes = load_html(r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<mapper>
    <insert id="insert">
        'insert into biz_activity'
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            '${key}'
        </foreach>
        'values'
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            '${item}'
        </foreach>
    </insert></mapper>"#).unwrap();
        println!("{:?}", nodes);
    }

    #[test]
    fn test_item() {
        let nodes = load_html(
            r#"
    <insert id="insert">
        'insert into biz_activity'
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            '${key}'
        </foreach>
        'values'
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            '${item}'
        </foreach>
    </insert>"#,
        )
        .unwrap();
        println!("{:?}", nodes);
    }
}
