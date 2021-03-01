#[cfg(test)]
mod test {
    use rbatis::plugin::snowflake::new_snowflake_id;

    #[test]
    fn test_new_block_id() {
        println!("{}", new_snowflake_id());
        println!("{}", new_snowflake_id());
    }
}
