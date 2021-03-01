#[cfg(test)]
mod test {
    use rbatis::plugin::snowflake::new_snowflake_id;
    use rbatis::utils::bencher::bench;
    #[test]
    fn test_new_block_id() {
        println!("{}", new_snowflake_id());
        println!("{}", new_snowflake_id());
    }

    #[test]
    fn test_bench_new_block_id() {
        rbatis::bench!(100000,{
          new_snowflake_id();
        });
    }
}
