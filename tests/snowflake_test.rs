#[cfg(test)]
mod test {
    use rbatis::plugin::snowflake::new_snowflake_id;
    use rbatis::snowflake::Snowflake;

    #[test]
    fn test_new_snowflake_id() {
        println!("{}", new_snowflake_id());
        println!("{}", new_snowflake_id());
    }

    #[test]
    fn test_snowflake_generation() {
        let sf = Snowflake::default();
        let id = sf.generate();
        assert_ne!(id, 0);
    }
}
