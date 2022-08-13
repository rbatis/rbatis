#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

use rbatis::snowflake::new_snowflake_id;
use test::Bencher;

#[bench]
fn bench_wrapper(b: &mut Bencher) {
    b.iter(|| new_snowflake_id());
}
