#[cfg(test)]
mod test {
    use rbatis::object_id::ObjectId;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_new() {
        println!("objectId:{}", ObjectId::new().to_string());
        println!("objectId:{}", ObjectId::new().to_string());
        println!("objectId:{}", ObjectId::new().to_string());
        println!("objectId:{}", ObjectId::new().to_string());
    }

    #[test]
    fn test_new_u128() {
        println!("objectId:{}", ObjectId::new().u128());
        println!("objectId:{}", ObjectId::new().u128());
        println!("objectId:{}", ObjectId::new().u128());
        println!("objectId:{}", ObjectId::new().u128());
    }

    #[test]
    fn test_display() {
        let id = ObjectId::with_string("53e37d08776f724e42000000").unwrap();

        assert_eq!(format!("{}", id), "53e37d08776f724e42000000")
    }

    #[test]
    fn test_debug() {
        let id = ObjectId::with_string("53e37d08776f724e42000000").unwrap();

        assert_eq!(format!("{:?}", id), "ObjectId(53e37d08776f724e42000000)")
    }

    #[test]
    fn test_u128() {
        let oid = ObjectId::new();
        println!("oid={}", oid);
        println!("oid-u128={}", oid.u128());
        println!("oid-from={}", ObjectId::with_u128(oid.u128()));
        assert_eq!(oid, ObjectId::with_u128(oid.u128()));
    }

    #[test]
    fn test_u128_parse() {
        for _ in 0..1000 {
            sleep(Duration::from_nanos(500));
            let oid = ObjectId::new();
            assert_eq!(oid, ObjectId::with_u128(oid.u128()));
        }
    }
    #[test]
    fn test_new_object_id() {
        println!("{}", ObjectId::new());
        println!("{}", ObjectId::new().u128());
    }
    
}
