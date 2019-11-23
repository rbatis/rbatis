extern crate rbatis_macro_derive;
use crate::utils;
use std::collections::HashMap;
use chrono::Local;
use crate::utils::time_util;

use std::thread;
use std::time::Duration;
use core::fmt::Debug;
use std::fmt::Display;
use core::borrow::Borrow;
use mysql::{Value, Conn};
use serde::{Serialize, Deserialize};
use test::Bencher;
use std::any::Any;
use serde_json::Error;
use rbatis_macro_derive::RbatisMacro;
use rbatis_macro::RbatisMacro;
use std::collections::hash_map::RandomState;
use crate::decode::Decoder::Decoder;

use postgres::{Client, NoTls};
use serde_json::{json};
use crate::example::Activity::Activity;

pub struct SqlBuilder {}


#[test]
fn TestSqlBuilder() {
    utils::print_util::print_rust_mybatis();


    let total = 100000;
    let mut paramMap = HashMap::new();
    paramMap.insert("name", "aaa");
    paramMap.insert("startTime", "aaa");
    paramMap.insert("endTime", "aaa");
    paramMap.insert("page", "1");
    paramMap.insert("size", "20");

    let now = Local::now();
    for i in 0..total {
        let mut sql = String::new();


       let mut arg=json!({
       "name":"sadf",
       "startTime":"startTime",
       "endTime":"endTime",
       "page":1,
       "size":1,
        });

        sql.push_str("select * from biz_activity  where name = #{name} and startTime=#{startTime} and endTime=#{endTime} limit page=#{page},size=#{size}");

        let name = paramMap.get("name").unwrap();
        if name != &"" {
            sql.replace("#{name}",name);
        }
        let startTime = paramMap.get("startTime").unwrap();
        if startTime != &"" {
            sql.replace("#{startTime}",startTime);
        }
        let endTime = paramMap.get("endTime").unwrap();
        if endTime != &"" {
            sql.replace("#{endTime}",endTime);
        }
        let pageStr = paramMap.get("page").unwrap();
        let page: i32 = pageStr.to_string().parse().unwrap();
        if page != 0 {
            sql.replace("#{page}",pageStr);
        }
        let sizeStr = paramMap.get("size").unwrap();
        let size: i32 = sizeStr.parse().unwrap();
        if size != 0 {
            sql.replace("#{size}",sizeStr);
        }
        // println!("{}",sql);
    }

    time_util::count_time(total, now);
    time_util::count_tps(total, now);
}

#[bench]
fn benchSqlBuilder(b: &mut Bencher){

    let mut paramMap = HashMap::new();
    paramMap.insert("name", "aaa");
    paramMap.insert("startTime", "aaa");
    paramMap.insert("endTime", "aaa");
    paramMap.insert("page", "1");
    paramMap.insert("size", "20");

    b.iter(|| {
        let mut sql = String::new();

        let mut arg=json!({
       "name":"sadf",
       "startTime":"startTime",
       "endTime":"endTime",
       "page":1,
       "size":1,
        });

        sql.push_str("select * from biz_activity  where name = #{name} and startTime=#{startTime} and endTime=#{endTime} limit page=#{page},size=#{size}");

        let name = paramMap.get("name").unwrap();
        if name != &"" {
            sql.replace("#{name}",name);
        }
        let startTime = paramMap.get("startTime").unwrap();
        if startTime != &"" {
            sql.replace("#{startTime}",startTime);
        }
        let endTime = paramMap.get("endTime").unwrap();
        if endTime != &"" {
            sql.replace("#{endTime}",endTime);
        }
        let pageStr = paramMap.get("page").unwrap();
        let page: i32 = pageStr.to_string().parse().unwrap();
        if page != 0 {
            sql.replace("#{page}",pageStr);
        }
        let sizeStr = paramMap.get("size").unwrap();
        let size: i32 = sizeStr.parse().unwrap();
        if size != 0 {
            sql.replace("#{size}",sizeStr);
        }
    });
}



#[test]
fn TestLinkMysql() {
    let mut ops = mysql::OptsBuilder::new();
    ops.user(Some("root"));
    ops.pass(Some("TEST"));
    ops.db_name(Some("test"));
    ops.ip_or_hostname(Some("localhost"));

    let mut conn = Conn::new(ops).unwrap();
    let mut rows = conn.prep_exec("SELECT * from biz_activity", ()).unwrap();
    let result:Result<Vec<Activity>,String> = rows.decode();
    if result.is_err() {
        panic!(result.err().unwrap());
    }
    println!("{:?}",result.unwrap());
}

#[test]
fn TestLinkPostgres() {
    let mut client = Client::connect("postgres://postgres:postgres@localhost:5432/postgres", postgres::NoTls).unwrap();
//    conn.execute("CREATE TABLE person (
//                    id              SERIAL PRIMARY KEY,
//                    name            VARCHAR NOT NULL,
//                    data            VARCHAR NULL
//                  )", &[]).unwrap();
    //conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)", &[&"Steven".to_string(), &"".to_string()]).unwrap();
//    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
//        let mut personMap = HashMap::new();
//        //personMap.insert(0,row.get(0));
//        let s: String = row.get(1);
//        personMap.insert("1", s);
//        //personMap.insert(2,row.get(2));
//        println!("Found person {}", personMap.get("1").unwrap());
//
//    }

    let mut r=client.query("SELECT * FROM person;", &[]).unwrap();
    let mut act:Result<Vec<Activity>,String> = r.decode();
    println!("decode: {:?}",act.unwrap());
}

#[bench]
fn Bench_Decode_Util(b: &mut Bencher) {
    let mut ops = mysql::OptsBuilder::new();
    ops.user(Some("root"));
    ops.pass(Some("TEST"));
    ops.db_name(Some("test"));
    ops.ip_or_hostname(Some("localhost"));

    let mut conn = Conn::new(ops).unwrap();
    let mut rows = conn.prep_exec("SELECT * from biz_activity;", ()).unwrap();
    b.iter( || {
        let result:Result<Vec<Activity>,String> = rows.decode();
        println!("{:?}",result.unwrap());
    });
}


//use Time: 0.638 s,each:6380 nano/op
//use TPS: 156739.8119122257 TPS/s
#[test]
fn TestBenchmarkTPS() {
    let now=Local::now();
    let mut ops = mysql::OptsBuilder::new();
    ops.user(Some("root"));
    ops.pass(Some("TEST"));
    ops.db_name(Some("test"));
    ops.ip_or_hostname(Some("localhost"));

    let mut conn = Conn::new(ops).unwrap();
    let mut rows = conn.prep_exec("SELECT * from biz_activity limit 1;", ()).unwrap();

    let total=100000;
    for _ in 0..total{
        let result:Result<Activity,String> = rows.decode();
    }
    utils::time_util::count_time(total,now);
    utils::time_util::count_tps(total,now);
}


#[test]
fn TestFF() {
    let mut m =HashMap::new();
    m.insert("s",serde_json::Value::String("s".to_string()));
    let arg=m.get("name").unwrap_or(&serde_json::value::Value::Null);

    let mut sql_data="".to_string();

    let mut fnvec:Vec<fn(param:&mut HashMap<&str,serde_json::Value,RandomState>,  sql:&mut String)>=vec![];
    fnvec.push(| param,sql |{  param.insert("patern",serde_json::Value::String("%".to_owned()+"%"));    });
    fnvec.push(| param,sql |{  sql.push_str("select * from biz_activity"); });

// sql.push_str("select * from biz_activity");

    fnvec[0](&mut m,&mut sql_data);
    fnvec[1](&mut m,&mut sql_data);

    println!("sql:{}",sql_data);
}