#[cfg(test)]
mod test {
    use rbatis::plugin::snowflake::{async_snowflake_id, block_snowflake_id};

    #[test]
    fn test_new_block_id() {
        println!("{}", block_snowflake_id());
        println!("{}", block_snowflake_id());
    }

    #[test]
    fn test_new_async_id() {
        rbatis::core::runtime::block_on(async {
            println!("{}", async_snowflake_id().await);
            println!("{}", async_snowflake_id().await);
        });
    }
}