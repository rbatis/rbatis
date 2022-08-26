#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

use test::Bencher;
use rbatis::utils::string_util::to_snake_name;

#[bench]
fn bench_to_snake_name(b: &mut Bencher) {
    b.iter(|| {
        to_snake_name("Abc");
    });
}