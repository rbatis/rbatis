#![feature(test)]
extern crate test;

use test::Bencher;

//test bench_rbs_encode ... bench:         139 ns/iter (+/- 1)
#[bench]
fn bench_rbs_encode(b: &mut Bencher) {
    let v = rbdc::types::datetime::DateTime::now();
    b.iter(|| {
        rbs::to_value!(&v);
    });
}

#[bench]
fn bench_rbs_from(b: &mut Bencher) {
    let v = rbs::to_value! {
        "a":1,
        "b":2,
    };
    b.iter(|| {
        let _ = rbdc::types::json::Json::from(v.clone());
    });
}
