#![feature(test)]
extern crate test;

use rbatis::snowflake::{Snowflake};
use test::Bencher;

#[bench]
fn bench_snowflake_id(b: &mut Bencher) {
    let snow = Snowflake::new(1, 1, 0);
    b.iter(|| snow.generate());
}

#[bench]
fn bench_snowflake_mode1(b: &mut Bencher) {
    let snow = Snowflake::new(1, 1, 1);
    b.iter(|| snow.generate());
}