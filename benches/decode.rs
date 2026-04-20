#![feature(test)]
extern crate test;

use rbs::{Value};
use test::Bencher;

#[bench]
fn bench_rbatis_decode(b: &mut Bencher) {
    let m = csv_data();
    b.iter(|| {
        rbatis::decode_ref::<i32>(&m).unwrap();
    });
}


#[bench]
fn bench_rbs_decode_inner(b: &mut Bencher) {
    let m = csv_data();
    b.iter(|| {
        rbatis::decode_ref::<i64>(&m).unwrap();
    });
}

// CSV格式: [[col_name], [value]]
fn csv_data() -> Value {
    Value::Array(vec![
        Value::Array(vec![Value::String("a".to_string())]),
        Value::Array(vec![Value::I64(1)])
    ])
}