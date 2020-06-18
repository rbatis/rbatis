use std::env::Args;
use std::ops::Deref;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use fast_log::log::RuntimeType;
use futures_core::future::BoxFuture;
use log::{error, info, warn};
use serde_json::json;
use tokio::macros::support::{Future, Pin};

use rbatis_core::connection::Connection;
use rbatis_core::cursor::Cursor;
use rbatis_core::executor::Executor;
use rbatis_core::mysql::{MySqlCursor, MySqlPool, MySqlRow};
use rbatis_core::types::BigDecimal;

use crate::example::conf::MYSQL_URL;
use crate::rbatis::Rbatis;

#[test]
pub fn test_log() {
    //1 启用日志(可选，不添加则不加载日志库)
    fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
    info!("print data");
    sleep(Duration::from_secs(1));
}


#[test]
pub fn test_mysql_driver() {
    let r = async_std::task::block_on(
        async move {
            let pool = MySqlPool::new(MYSQL_URL).await.unwrap();
            //pooledConn 交由rbatis上下文管理
            let mut conn = pool.acquire().await.unwrap();
            let mut c = conn.fetch("SELECT count(1) FROM biz_activity;");
            let r: serde_json::Value = c.decode().await.unwrap();
            println!("done:{:?}", r);
        }
    );
}

#[test]
pub fn test_mysql_() {
    let r = async_std::task::block_on(
        async move {
            let rb = Rbatis::new(MYSQL_URL).await.unwrap();
            //pooledConn 交由rbatis上下文管理
            let arg = &vec![json!("count(1)")];
            let r: serde_json::Value = rb.fetch_prepare("SELECT ? FROM biz_activity;", arg).await.unwrap();
            println!("done:{:?}", r);
        }
    );
}

#[test]
pub fn test_rbatis() {
    async_std::task::block_on(
        async move {
            let mut rb = Rbatis::new("").await.unwrap();
            rb.load_xml("test", r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE mapper PUBLIC "-//mybatis.org//DTD Mapper 3.0//EN"
        "https://raw.githubusercontent.com/zhuxiujia/Rbatis/master/rbatis-mapper.dtd">
<mapper>
    <result_map id="BaseResultMap" table="biz_activity">
        <id column="id"/>
        <result column="name" lang_type="string"/>
        <result column="pc_link" lang_type="string"/>
        <result column="h5_link" lang_type="string"/>
        <result column="pc_banner_img" lang_type="string"/>
        <result column="h5_banner_img" lang_type="string"/>
        <result column="sort" lang_type="string"/>
        <result column="status" lang_type="number"/>
        <result column="remark" lang_type="string"/>
        <result column="version" lang_type="number" version_enable="true"/>
        <result column="create_time" lang_type="time"/>
        <result column="delete_flag" lang_type="number" logic_enable="true" logic_undelete="1"
                logic_deleted="0"/>
    </result_map>
    <select id="select_by_condition">
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
</mapper>"#).unwrap();
        }
    )
}



#[test]
pub fn test_hook() {

}