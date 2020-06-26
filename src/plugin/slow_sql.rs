use std::collections::LinkedList;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SqlLog {
    pub sql: String,
    pub return_data: String,
    pub second: i64,
}

pub fn find_slow_sql(log_path: &str) -> LinkedList<SqlLog> {
    let mut use_time_vec = LinkedList::new();
    let input_opt = File::open(log_path);
    if input_opt.is_err() {
        println!("{}", input_opt.err().unwrap());
        return use_time_vec;
    }
    let buffered = BufReader::new(input_opt.unwrap());

    let mut temp = 0;
    let mut time_start = None;
    let mut sql = "".to_string();

    for x in buffered.lines().map(|r| r.unwrap()) {
        if x.contains("Query:") || x.contains("Exec:") {
            time_start = Some(x[0..x.find(" - ").unwrap()].to_string());
            sql = x.clone();
        }
        if time_start.is_some() && (x.contains("Total: <==") || x.contains("Affected: <==")) {
            let time_end = x[0..x.find(" - ").unwrap()].to_string();
            let date_time = time_start.unwrap().parse::<DateTime<Utc>>().unwrap();
            let date_time_end = time_end.parse::<DateTime<Utc>>().unwrap();

            let use_time = date_time_end.timestamp() - date_time.timestamp();
            if use_time > temp {
                use_time_vec.push_front(SqlLog {
                    sql: sql.to_string(),
                    return_data: x,
                    second: use_time,
                });
                temp = use_time;
            }
            //println!("wait: {}", use_time);
            time_start = None;
            sql.clear();
        }
    }
    return use_time_vec;
}

#[test]
pub fn test_find_slow_sql() {
    let use_time_vec = find_slow_sql("../rbatis/requests.log");
    println!("top use time: {:?}", use_time_vec);
}