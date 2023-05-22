#![feature(test)]
extern crate test;

use rbatis::snowflake::new_snowflake_id;
use test::Bencher;

#[bench]
fn bench_snowflake_id(b: &mut Bencher) {
    new_snowflake_id();
    b.iter(|| new_snowflake_id());
}
