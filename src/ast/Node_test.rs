use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::{json, Value};
use crate::ast::StringNode::StringNode;
use test::Bencher;
use crate::ast::convert::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use std::rc::Rc;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::core::Rbatis::Rbatis;

#[test]
fn TestEval(){
    let mut holder=NodeConfigHolder::new();
    let mut john =  json!({
        "name": "John Doe",
    });

    let mut rbatis=Rbatis::new(r#"
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
    "#.to_string());
    let mut strNode=rbatis.Get("selectByCondition");
    println!("{}",strNode.print());
    let r=&strNode.eval(&mut john,&mut holder);
    println!("r:{}",r.clone().unwrap());
}


#[test]
fn TestStringNode() {
    let mut holder=NodeConfigHolder::new();
    let mut john = json!({
        "name": "John Doe",
    });
    let mut strNode = NodeType::NString(StringNode::new("select * from ${name} where name = #{name}"));

    let result = strNode.eval(&mut john,&mut holder).unwrap();
    println!("{}", result);
}

#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let mut holder=NodeConfigHolder::new();
    let mut john =  json!({
        "name": "John Doe",
    });


    let mut strNode = NodeType::NString(StringNode::new("vvvvvvvvvv#{name}vvvvvvvv"));

    b.iter(|| {
        &strNode.eval(&mut john,&mut holder);
    });
}

#[bench]
fn Bench_Eval(b: &mut Bencher) {
    let mut holder=NodeConfigHolder::new();
    let mut john =  json!({
        "name": "John Doe",
    });

    let mut rbatis=Rbatis::new(r#"
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
    "#.to_string());
    let mut strNode=rbatis.Get("selectByCondition");
    let r=&strNode.eval(&mut john,&mut holder);
    b.iter(|| {
        &strNode.eval(&mut john,&mut holder);
    });
}
