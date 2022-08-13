#![feature(test)]
#![feature(bench_black_box)]
extern crate test;
use test::Bencher;
use rbatis::object_id::ObjectId;

#[bench]
fn bench_object_id(b: &mut Bencher) {
    b.iter(|| ObjectId::new().u64());
}
