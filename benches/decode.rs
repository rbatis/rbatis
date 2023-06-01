#![feature(test)]
extern crate test;

use rbs::{Value, to_value};
use test::Bencher;

#[bench]
fn bench_rbs_decode(b: &mut Bencher) {
    let v: Value = 1.into();
    b.iter(|| {
        rbs::from_value::<i32>(v.clone()).unwrap();
    });
}

#[bench]
fn bench_rbs_decode_value(b: &mut Bencher) {
    let v: Value = 1.into();
    b.iter(|| {
        rbs::from_value::<Value>(v.clone()).unwrap();
    });
}

#[bench]
fn bench_rbatis_decode(b: &mut Bencher) {
    let array = Value::Array(vec![to_value! {
        1 : 1,
    }]);
    b.iter(|| {
        rbatis::decode::<i32>(array.clone()).unwrap();
    });
}

#[bench]
fn bench_rbatis_decode_map(b: &mut Bencher) {
    let date = rbdc::types::datetime::DateTime::now();
    let array = Value::Array(vec![to_value! {
        1 : date,
    }]);
    b.iter(|| {
        rbatis::decode::<rbdc::types::datetime::DateTime>(array.clone()).unwrap();
    });
}

#[bench]
fn bench_is_debug_mode(b: &mut Bencher) {
    let rb = rbatis::RBatis::new();
    b.iter(|| {
        rb.is_debug_mode();
    });
}
