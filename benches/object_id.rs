#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

use rbs::Value;
use std::rc::Rc;
use test::Bencher;
use rbatis::object_id::ObjectId;

#[bench]
fn bench_wrapper(b: &mut Bencher) {
    b.iter(|| ObjectId::new());
}
