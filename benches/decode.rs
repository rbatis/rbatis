#![feature(test)]
#![feature(bench_black_box)]
extern crate test;
use test::Bencher;
use rbs::{Value, value_map};

#[bench]
fn bench_rbs_decode(b: &mut Bencher) {
    let v:Value = 1.into();
    b.iter(|| {
        rbs::from_value::<i32>(v.clone()).unwrap();
    });
}

#[bench]
fn bench_rbatis_decode(b: &mut Bencher) {
    let array = Value::Array(vec![Value::Map(value_map! {
            1 => 1,
        })]);
    b.iter(|| {
        rbatis::decode::<i32>(array.clone()).unwrap();
    });
}

#[bench]
fn bench_rbatis_decode_map(b: &mut Bencher) {
    let date = rbdc::types::datetime::FastDateTime::now();
    let array = Value::Array(vec![Value::Map(value_map! {
            1 => date,
        })]);
    b.iter(|| {
        rbatis::decode::<rbdc::types::datetime::FastDateTime>(array.clone()).unwrap();
    });
}