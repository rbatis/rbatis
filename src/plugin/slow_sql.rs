use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::LinkedList;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SqlLog {
    pub sql: String,
    pub return_data: String,
    pub second: i64,
}

pub fn find_slow_sql(log_path: &str) -> LinkedList<SqlLog>{
    let data_result = fs::read_to_string(log_path);
    let mut use_time_vec = LinkedList::new();
    if data_result.is_err(){
        return use_time_vec;
    }
    let data = data_result.unwrap();
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
                use_time_vec.push_front(SqlLog {
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
    return use_time_vec;
}

#[test]
pub fn test_find_slow_sql() {
    let  use_time_vec= find_slow_sql("D:/RustProject/rbatis/requests.log");
    println!("top use time: {:?}", use_time_vec);
}