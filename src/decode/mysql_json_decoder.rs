

use crate::decode::decoder::{Decoder, is_array, json_len};
use std::sync::Arc;
use mysql::{Column, Row, QueryResult};
use std::result;
use serde::de;
use std::any::Any;

use std::collections::HashMap;
use serde_json::Number;
use std::str::FromStr;
use serde::de::DeserializeOwned;
use std::ops::Deref;
use serde_json::json;

use mysql_common::packets;
use mysql_common::constants::ColumnType;
use mysql_common::io::WriteMysqlExt;
use mysql_common::packets::column_from_payload;
use mysql_common::value::Value;
use mysql_common::row::new_row;
use chrono::Local;
use crate::utils;


impl Decoder for QueryResult<'_> {
    fn decode<T:?Sized>(&mut self,decode_len:&mut usize) -> Result<T, String> where T: DeserializeOwned {
        let mut js = serde_json::Value::Null;
        let type_name=std::any::type_name::<T>();
        //println!("type_name>>>   {}",type_name);
        if is_array::<T>(type_name) {
            //decode array
            let mut vec_v = vec![];
            for item in self {
                let act = decode_row(&item.unwrap());
                vec_v.push(act);
            }
            js = serde_json::Value::Array(vec_v)
        }else{
            match std::any::type_name::<T>() {
                "i32" | "u32" | "f32" | "i64" | "u64" | "f64" | "serde_json::number::Number" => {
                    //decode number
                    let mut size = 0;
                    for item in self {
                        if size > 0 {
                            continue;
                        }
                        let act = decode_row(&item.unwrap());
                        match act {
                            serde_json::Value::Object(arg) => {
                                for (_, r) in arg {
                                    js = r;
                                    break
                                }
                            }
                            _ => {}
                        }
                        size += 1;
                    }
                },
                "serde_json::value::Value" => {
                    //decode json
                    let mut vec_v = vec![];
                    for item in self {
                        let act = decode_row(&item.unwrap());
                        vec_v.push(act);
                    }
                    js = serde_json::Value::Array(vec_v)
                },
                _ => {
                    //decode struct
                    let mut index = 0;
                    self.for_each(|item| {
                        if index >= 1 {
                            index = index + 1;
                            return;
                        }
                        js = decode_row(&item.unwrap());
                        index = index + 1;
                    });
                    if index > 1 {
                        return Result::Err("[rbatis] rows.affected_rows > 1,but decode one result!".to_string());
                    }
                }
            }
        }
        *decode_len=json_len(&js);
        let decode_result = serde_json::from_value(js);
        if decode_result.is_ok() {
            return Result::Ok(decode_result.unwrap());
        } else {
            let e = decode_result.err().unwrap().to_string();
            return Result::Err(e);
        }
    }
}

fn decode_row(row: &Row) -> serde_json::Value {
    let cs = row.columns();
    let mut m = serde_json::map::Map::new();
    for c in cs.as_ref() {
        let column_name = c.name_str();
        let k = column_name.as_ref();
        let f: mysql::Value = row.get(k).unwrap();

        let mut sql = f.as_sql(true);
        let sql_len = sql.len();
        let item: serde_json::Value;
        if sql.as_str() == "NULL" {
            item = serde_json::Value::Null;
        } else {
            if sql == "''" {
                sql = "\"\"".to_owned();
                item = serde_json::Value::String(sql);
            } else if sql.starts_with("'") {
                let slice = &sql[1..(sql_len - 1)];
                sql = "\"".to_owned() + slice + "\"";
                item = serde_json::Value::String(sql);
            } else {
                let n = Number::from_str(sql.as_str());
                if n.is_ok() {
                    item = serde_json::Value::Number(n.unwrap());
                } else {
                    item = serde_json::Value::Null;
                }
            }
        }
        m.insert(column_name.to_string(), item);
    }
    return serde_json::Value::Object(m);
}




/// ------------------------------------以下为测试内容--------------------------------------------------------------

//解码性能测试代码
fn decode_test<T:?Sized>(decode_len:&mut usize,rows:&Vec<Row>) -> Result<T, String> where T: DeserializeOwned {
        let mut js = serde_json::Value::Null;
        //decode array
        let mut vec_v = vec![];
        for item in rows {
            let act = decode_row(item);
            vec_v.push(act);
        }
        js = serde_json::Value::Array(vec_v);
        *decode_len=json_len(&js);
        let decode_result = serde_json::from_value(js);
        if decode_result.is_ok() {
            return Result::Ok(decode_result.unwrap());
        } else {
            let e = decode_result.err().unwrap().to_string();
            return Result::Err(e);
        }
}

fn col(name: &str, ty: ColumnType) -> Column {
    let mut payload = b"\x00def".to_vec();
    for _ in 0..5 {
        payload.write_lenenc_str(name.as_bytes()).unwrap();
    }
    payload.extend_from_slice(&b"_\x2d\x00\xff\xff\xff\xff"[..]);
    payload.push(ty as u8);
    payload.extend_from_slice(&b"\x00\x00\x00"[..]);
    column_from_payload(payload).unwrap()
}

///压测单线程 sql 数据解码性能
//cargo test --release --color=always --package rbatis --bin rbatis decode::mysql_json_decoder::bench_decode_mysql_json --all-features -- --nocapture --exact
#[test]
pub fn bench_decode_mysql_json(){
    let mut tt=0;

    //模拟sql
    let mut js = serde_json::Value::Null;
    let vs=vec![
        Value::from("dfbdd779-5f70-4b8f-9921-a235a9c75b69"),
        Value::from("新人专享"),
        Value::from(1),
    ];
    let cols=std::sync::Arc::new(vec![
        col("id", ColumnType::MYSQL_TYPE_STRING),
        col("name", ColumnType::MYSQL_TYPE_STRING),
        col("version", ColumnType::MYSQL_TYPE_INT24),
    ]);
    let row = new_row(vs,cols);
    let rows=vec![row];

    //开始测试
    let total=100000;
    let now=Local::now();
    for _ in 0..total{
          let r:serde_json::Value= decode_test(&mut tt,&rows).unwrap();
    }
    utils::time_util::count_time(total,now);
    utils::time_util::count_tps(total,now);
}