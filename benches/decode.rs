#![feature(test)]
extern crate test;

use rbs::{to_value, Value};
use test::Bencher;

#[bench]
fn bench_rbs_decode(b: &mut Bencher) {
    let v: Value = to_value!(1);
    b.iter(|| {
        rbs::from_value_ref::<i32>(&v).unwrap();
    });
}

#[bench]
fn bench_rbs_decode_value(b: &mut Bencher) {
    let v: Value = 1.into();
    b.iter(|| {
        rbs::from_value_ref::<Value>(&v).unwrap();
    });
}

#[bench]
fn bench_rbatis_decode(b: &mut Bencher) {
    let array = Value::Array(vec![to_value! {
        1 : 1,
    }]);
    b.iter(|| {
        rbatis::decode_ref::<i32>(&array).unwrap();
    });
}

#[bench]
fn bench_rbatis_decode_map(b: &mut Bencher) {
    let date = rbdc::types::datetime::DateTime::now();
    let array = Value::Array(vec![to_value! {
        1 : date,
    }]);
    b.iter(|| {
        rbatis::decode_ref::<rbdc::types::datetime::DateTime>(&array).unwrap();
    });
}

#[bench]
fn bench_rbs_decode_inner(b: &mut Bencher) {
    let m = Value::Array(vec![to_value! {
        "aa": 0
    }]);
    b.iter(|| {
        rbatis::decode_ref::<i64>(&m).unwrap();
    });
}
