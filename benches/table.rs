#![feature(test)]
extern crate test;

use rbatis::utils::string_util::to_snake_name;
use test::Bencher;

#[bench]
fn bench_to_snake_name(b: &mut Bencher) {
    b.iter(|| {
        to_snake_name("Abc");
    });
}
