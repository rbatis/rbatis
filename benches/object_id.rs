#![feature(test)]
extern crate test;

use rbatis::object_id::ObjectId;
use test::Bencher;

#[bench]
fn bench_object_id(b: &mut Bencher) {
    b.iter(|| ObjectId::new().u128());
}
