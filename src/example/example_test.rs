use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::env::Args;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use fast_log::log::RuntimeType;
use futures_core::future::BoxFuture;
use log::{error, info, LevelFilter, warn};
use serde_json::{json, Value};
use tide::Request;
use tokio::macros::support::{Future, Pin};

use rbatis_core::connection::Connection;
use rbatis_core::cursor::Cursor;
use rbatis_core::executor::Executor;
use rbatis_core::mysql::{MySqlCursor, MySqlPool, MySqlRow};
use rbatis_core::sync_map::SyncMap;
use rbatis_core::types::BigDecimal;

use crate::example::activity::Activity;
use crate::example::conf::MYSQL_URL;
use crate::rbatis::Rbatis;
use crate::utils::time_util::count_time_tps;

#[test]
pub fn test_log() {
    //1 启用日志(可选，不添加则不加载日志库)
    fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
    info!("print data");
    sleep(Duration::from_secs(1));
}


#[test]
pub fn test_use_driver() {
    let r = async_std::task::block_on(
        async move {
            fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
            let pool = MySqlPool::new(MYSQL_URL).await.unwrap();
            //pooledConn 交由rbatis上下文管理
            let mut conn = pool.acquire().await.unwrap();
            let mut c = conn.fetch("SELECT count(1) FROM biz_activity;");
            let r: serde_json::Value = c.decode_json().await.unwrap();
            println!("done:{:?}", r);
        }
    );
}

#[test]
pub fn test_prepare_sql() {
    let r = async_std::task::block_on(
        async move {
            fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
            let rb = Rbatis::new(MYSQL_URL).await.unwrap();
            //pooledConn 交由rbatis上下文管理
            let arg = &vec![json!(1), serde_json::Value::String("test%".to_string())];
            let r: Vec<Activity> = rb.fetch_prepare("", "SELECT * FROM biz_activity WHERE delete_flag =  ? AND name like ?", arg).await.unwrap();
            println!("done:{:?}", r);
        }
    );
}


#[test]
pub fn test_py_sql() {
    async_std::task::block_on(async move {
        fast_log::log::init_log("requests.log", &RuntimeType::AsyncStd).unwrap();
        let mut rb = Rbatis::new(MYSQL_URL).await.unwrap();
        let py = r#"
    SELECT * FROM biz_activity
    WHERE delete_flag = #{delete_flag}
    if name != null:
      AND name like #{name+'%'}
    if ids != null:
      AND id in (
      trim ',':
         for item in ids:
           #{item},
      )"#;
        let data: serde_json::Value = rb.py_fetch("", py, &json!({

           "delete_flag": 1
        })).await.unwrap();
        println!("{}", data);
    });
}


#[test]
pub fn test_xml_sql() {
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


lazy_static! {
  static ref M:SyncMap<String>=SyncMap::new();
}

#[test]
pub fn test_tx() {
    async_std::task::block_on(async {
        //TODO fix tx out of time
        let rb = Rbatis::new(MYSQL_URL).await.unwrap();
        let tx_id = "1";
        rb.begin(tx_id).await.unwrap();
        let v: serde_json::Value = rb.fetch(tx_id, "SELECT count(1) FROM biz_activity;").await.unwrap();
        println!("{}", v.clone());
        rb.commit(tx_id).await.unwrap();
    });
}




lazy_static! {
  static ref RB:Rbatis<'static>=async_std::task::block_on(async {
        Rbatis::new(MYSQL_URL).await.unwrap()
    });
}

#[test]
pub fn test_tide() {
    async_std::task::block_on(async {
        let mut app = tide::new();
        app.at("/test").get(|mut req: Request<()>| async move {
            let a = req.body_string().await;
            // println!("accept req[{} /test] arg: {:?}",req.url().to_string(),a);
            let v = RB.fetch("", "SELECT count(1) FROM biz_activity;").await;
            if v.is_ok() {
                let data: Value = v.unwrap();
                Ok(data.to_string())
            } else {
                Ok(v.err().unwrap().to_string())
            }
        });
        app.at("/").get(|_| async { Ok("Hello, world!") });
        let addr = "0.0.0.0:8000";
        println!("server on {}", addr);
        app.listen(addr).await.unwrap();
    });
}