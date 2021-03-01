#[cfg(test)]
mod test {
    use rbatis::plugin::snowflake::new_snowflake_id;
    #[test]
    fn test_new_block_id() {
        println!("{}", new_snowflake_id());
        println!("{}", new_snowflake_id());
    }

    //cargo.exe test --release --package rbatis --test snowflake_test test::test_bench_new_block_id --no-fail-fast -- --exact -Z unstable-options --show-output
    #[test]
    fn test_bench_new_block_id() {
        rbatis::bench!(100000,{
          new_snowflake_id();
        });
    }
}
