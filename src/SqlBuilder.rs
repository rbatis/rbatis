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
use mysql::Value;
use serde::{Serialize, Deserialize};

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


#[derive(Serialize, Deserialize, Debug)]
pub struct Act {
    pub id: String,
    pub name: String,
    pub version: i32,
}


#[test]
fn TestLinkMysql() {
    let mut ops = mysql::OptsBuilder::new();
    ops.user(Option::Some("root"));
    ops.pass(Option::Some("TEST"));
    ops.db_name(Option::Some("test"));
    ops.ip_or_hostname(Option::Some("115.220.9.139"));


    let mut conn = mysql::Conn::new(ops).unwrap();
    for row in conn.prep_exec("SELECT * from biz_activity limit 1;", ()).unwrap() {
        let a = row.unwrap();


        let mut serialized = "{".to_owned();

        let cs = a.columns();
        for t in cs.as_ref() {
            let columnName = t.name_str();
            println!("{:?}", columnName);
            let k = columnName.as_ref();
            let f: Value = a.get(k).unwrap();
            println!("{:?}", f);

            serialized = serialized + "\"" + columnName.as_ref() + "\"";

            let mut sql = f.as_sql(true);
            if sql.as_str() == "NULL" {
                sql = "null".to_string();
            } else {
                if sql == "''" {
                    sql = "\"\"".to_owned();
                } else {
                    let sqlLen = sql.len();
                    let first = sql.find("'").unwrap_or_default();
                    let last = sql.rfind("'").unwrap_or_default();
                    if first == 0 && last == (sqlLen - 1) && first != last {
                        let slice = &sql[1..(sqlLen - 1)];
                        sql = "\"".to_owned() + slice + "\"";
                    }
                }
            }

            serialized = serialized + ":" + sql.as_str() + ",";
        }
        serialized.pop();
        serialized = serialized + "}";
        println!("query sql result ==> {}", serialized);

        //decode
        // Convert the JSON string back to a Point.
        let deserialized: Act = serde_json::from_str(&serialized).unwrap();

        // Prints deserialized = Point { x: 1, y: 2 }
        println!("deserialized = {:?}", deserialized);
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

#[test]
fn TestPool() {
    println!("{}", "dsaf")
}