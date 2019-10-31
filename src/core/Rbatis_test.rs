use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::core::Rbatis::Rbatis;
use serde_json::{json, Value};

#[test]
pub fn TestRbatis(){
    let mut john =  json!({
        "name": "John Doe",
    });

    let mut rbatis=Rbatis::new(r#"
    <mapper>
    <select id="selectByCondition" resultMap="BaseResultMap">
        <bind name="pattern" value="'%' + name + '%'"/>
        select * from biz_activity
        <where>
            <if test="name != null">and name like #{pattern}\n</if>
            <if test="startTime != null">and create_time >= #{startTime}\n</if>
            <if test="endTime != null">and create_time &lt;= #{endTime}\n</if>
        \n</where>
        order by create_time desc
        <if test="page != null and size != null">limit #{page}, #{size}\n</if>
    \n</select>
    \n</mapper>
    "#.to_string());
    let mut r=rbatis.eval("selectByCondition",&mut john).unwrap();
    println!("{}",r);
}