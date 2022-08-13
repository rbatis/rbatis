#[cfg(test)]
mod test {
    use rbatis::object_id::ObjectId;

    #[test]
    fn test_new_object_id() {
        println!("{}", ObjectId::new());
        println!("{}", ObjectId::new().u128());
    }
}
