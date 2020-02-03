use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::LinkedList;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Log {
    pub sql: String,
    pub return_data: String,
    pub second: i64,
}

pub fn find_slow_sql(log_path: &str) {
    let data_result = fs::read_to_string(log_path);
    if data_result.is_err(){
        return;
    }
    let data = data_result.unwrap();

    let mut use_time_vec = LinkedList::new();
    let mut temp = 0;
    let mut time_start = None;
    let mut sql = "";
    for x in data.lines() {
        if x.contains("Query:") || x.contains("Exec:") {
            time_start = Some(x[0..x.find(" - ").unwrap()].to_string());
            sql = x;
        }
        if time_start.is_some() && (x.contains("Total: <==") || x.contains("Affected: <==")) {
            let time_end = x[0..x.find(" - ").unwrap()].to_string();
            let date_time = time_start.unwrap().parse::<DateTime<Utc>>().unwrap();
            let date_time_end = time_end.parse::<DateTime<Utc>>().unwrap();

            let use_time = date_time_end.timestamp() - date_time.timestamp();
            if use_time > temp {
                use_time_vec.push_front(Log {
                    sql: sql.to_string(),
                    return_data: x.to_string(),
                    second: use_time,
                });
                temp = use_time;
            }
            //println!("wait: {}", use_time);
            time_start = None;
            sql = "";
        }
    }
    println!("top use time: {:?}", use_time_vec);
}

#[test]
pub fn test_find_slow_sql() {
    //let date_time = "2020-01-10T21:03:47.617953100+08:00".parse::<DateTime<Utc>>().unwrap();

    // println!("{}", date_time.timestamp());
    //let date_time2 = "2020-01-10T21:04:26.669343400+08:00".parse::<DateTime<Utc>>().unwrap();

    //println!("{}", date_time2.timestamp() - date_time.timestamp());

    find_slow_sql("D:/RustProject/rbatis/requests.log");
}