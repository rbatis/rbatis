extern crate rbatis_macro_derive;
use crate::utils;
use std::collections::HashMap;
use chrono::Local;
use crate::utils::time_util;

use std::thread;
use std::time::Duration;
use core::fmt::Debug;
use std::fmt::Display;
use postgres::{Connection, TlsMode};
use core::borrow::Borrow;
use mysql::{Value, Conn};
use serde::{Serialize, Deserialize};
use test::Bencher;
use std::any::Any;
use crate::utils::decode_util::decode;
use serde_json::Error;
use rbatis_macro_derive::RbatisMacro;
use rbatis_macro::RbatisMacro;
use std::collections::hash_map::RandomState;

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
        sql.push_str("select * from biz_activity  where ");

        let name = paramMap.get("name").unwrap();
        if name != &"" {
            sql.push_str(name);
            sql.push_str(&" ");
        }
        let startTime = paramMap.get("startTime").unwrap();
        if startTime != &"" {
            sql.push_str(startTime);
            sql.push_str(&" ");
        }
        let endTime = paramMap.get("endTime").unwrap();
        if endTime != &"" {
            sql.push_str(endTime);
            sql.push_str(&" ");
        }
        let pageStr = paramMap.get("page").unwrap();
        let page: i32 = pageStr.to_string().parse().unwrap();
        if page != 0 {
            sql.push_str(&page.to_string());
            sql.push_str(&" ");
        }
        let sizeStr = paramMap.get("size").unwrap();
        let size: i32 = sizeStr.parse().unwrap();
        if size != 0 {
            sql.push_str(&size.to_string());
            sql.push_str(&" ");
        }
        // println!("{}",sql);
    }

    time_util::count_time(total, now);
    time_util::count_tps(total, now);
}


#[derive(Serialize, Deserialize, Debug, Clone,RbatisMacro)]
pub struct Act {
    pub id: String,
    pub name: String,
    pub version: Option<i32>,
}




#[test]
fn TestLinkMysql() {
    let mut ops = mysql::OptsBuilder::new();
    ops.user(Some("root"));
    ops.pass(Some("TEST"));
    ops.db_name(Some("test"));
    ops.ip_or_hostname(Some("115.220.9.139"));

    let mut conn = Conn::new(ops).unwrap();
    let rows = conn.prep_exec("SELECT * from biz_activity limit 2;", ()).unwrap();
    let rq=utils::decode_util::RQueryResult::from_query_result(rows);
    let result:Result<Vec<Act>,String> = decode(rq);
    if result.is_err() {
        panic!(result.err().unwrap());
    }
    for item in &result.unwrap() {
        println!("{}", item.name);
    }
}

#[test]
fn TestLinkPostgres() {
    let conn = Connection::connect("postgres://postgres:postgres@127.0.0.1:5432/postgres", TlsMode::None).unwrap();
    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            VARCHAR NULL
                  )", &[]).unwrap();
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&"Steven".to_string(), &"".to_string()]).unwrap();
    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let mut personMap = HashMap::new();
        //personMap.insert(0,row.get(0));
        let s: String = row.get(1);
        personMap.insert("1", s);
        //personMap.insert(2,row.get(2));
        println!("Found person {}", personMap.get("1").unwrap());
    }
}

#[bench]
fn Bench_Decode_Util(b: &mut Bencher) {
    let mut ops = mysql::OptsBuilder::new();
    ops.user(Some("root"));
    ops.pass(Some("TEST"));
    ops.db_name(Some("test"));
    ops.ip_or_hostname(Some("115.220.9.139"));

    let mut conn = Conn::new(ops).unwrap();
    let rows = conn.prep_exec("SELECT * from biz_activity limit 1;", ()).unwrap();
    let rq=&utils::decode_util::RQueryResult::from_query_result(rows);

    b.iter( || {
        let result:Result<Act,String> = decode(rq.clone());
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
    ops.ip_or_hostname(Some("115.220.9.139"));

    let mut conn = Conn::new(ops).unwrap();
    let rows = conn.prep_exec("SELECT * from biz_activity limit 1;", ()).unwrap();
    let rq=&utils::decode_util::RQueryResult::from_query_result(rows);

    let total=100000;
    for _ in 0..total{
        let result:Result<Act,String> = decode(rq.clone());
    }
    utils::time_util::count_time(total,now);
    utils::time_util::count_tps(total,now);
}


#[test]
fn TestFF() {
    foo();
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