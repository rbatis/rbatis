#![feature(test)]
extern crate test;

use test::Bencher;
use rbs::value::util::to_number;

//test bench_extract_number ... bench:           3 ns/iter (+/- 0)
#[bench]
fn bench_extract_number(b: &mut Bencher) {
    let v ="dsafasdfasdfasdfasdf1.111gdfsgdsfgsdfg";
    b.iter(|| {
       to_number(v);
    });
}