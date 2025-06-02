#![feature(test)]
extern crate test;

use test::Bencher;
use rbs::Value;
use rbatis::crud_gen::gen_insert_batch;

//test bench_rbs_encode ... bench:         139 ns/iter (+/- 1)
#[bench]
fn bench_rbs_encode(b: &mut Bencher) {
    let v = rbdc::types::datetime::DateTime::now();
    b.iter(|| {
        rbs::value!(&v);
    });
}

#[bench]
fn bench_rbs_from(b: &mut Bencher) {
    let v = rbs::value! {
        "a":1,
        "b":2,
    };
    b.iter(|| {
        let _ = rbdc::types::json::Json::from(v.clone());
    });
}


#[bench]
fn bench_insert(b: &mut Bencher) {
    let v = vec![rbs::value! {
        "a":1,
        "b":2,
    }];
    b.iter(|| {
        let _ =  gen_insert_batch("table",Value::Array(v.clone()));
    });
}
