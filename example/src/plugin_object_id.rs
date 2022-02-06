#[cfg(test)]
mod test {
    use rbatis::plugin::object_id::ObjectId;

    #[tokio::test]
    pub async fn test_snowflake() {
        let id = ObjectId::new();
        println!("obj id: {}", id);
    }
}