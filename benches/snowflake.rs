#![feature(test)]
extern crate test;

use rbatis::snowflake::new_snowflake_id;
use test::Bencher;

#[bench]
fn bench_snowflake_id(b: &mut Bencher) {
    b.iter(|| new_snowflake_id());
}
