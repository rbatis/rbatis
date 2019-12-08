use crate::ast::config_holder::ConfigHolder;
use crate::core::rbatis::Rbatis;
use serde_json::{json, Value};
use crate::core::db_config::DBConfig;

#[test]
pub fn test_rbatis() {
    let mut john = json!({
        "name": "John Doe",
        "startTime":"2019-02-08"
    });

    let mut rbatis = Rbatis::new(r#"
    <mapper>
    <select id="selectByCondition" resultMap="BaseResultMap">
        <bind name="pattern" value="'%' + name + '%'"/>
        select * from biz_activity
        <where>
            <if test="name != null">and name like #{pattern}</if>
            <if test="startTime != null">and create_time >= #{startTime}</if>
            <if test="endTime != null">and create_time &lt;= #{endTime}</if>
        </where>
        order by create_time desc
        <if test="page != null and size != null">limit #{page}, #{size}</if>
    </select>
    </mapper>
    "#.to_string(),"mysql://postgres:1233456@localhost:5432/test".to_string());
    let mut r = rbatis.eval("selectByCondition", &mut john).unwrap();
    println!("{}", r);
}

#[test]
pub fn test_parser_url() {
    let link = "mysql://postgres:1233456@localhost:5432/test";

    let db_config=DBConfig::new(link.to_string());

    println!("{:?}",db_config);
}