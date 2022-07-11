#[cfg(test)]
mod test {
    use rbatis::snowflake::new_snowflake_id;

    #[tokio::test]
    pub async fn test_snowflake() {
        let id = new_snowflake_id();
        println!("snowflakeid: {}", id);
    }
}
