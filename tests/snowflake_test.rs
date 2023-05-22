#[cfg(test)]
mod test {
    use std::sync::atomic::Ordering;
    use rbatis::plugin::snowflake::new_snowflake_id;
    use rbatis::snowflake::Snowflake;

    #[test]
    fn test_new_snowflake_id() {
        println!("{}", new_snowflake_id());
        println!("{}", new_snowflake_id());
    }


    #[test]
    fn test_snowflake_serialization() {
        let sf1 = Snowflake::default();
        let serialized = serde_json::to_string(&sf1).unwrap();
        let sf2: Snowflake = serde_json::from_str(&serialized).unwrap();
        assert_eq!(sf1.epoch, sf2.epoch);
        assert_eq!(sf1.worker_id, sf2.worker_id);
        assert_eq!(sf1.datacenter_id, sf2.datacenter_id);
        assert_eq!(sf1.sequence.load(Ordering::Relaxed), sf2.sequence.load(Ordering::Relaxed));
    }

    #[test]
    fn test_snowflake_cloning() {
        let sf1 = Snowflake::default();
        let sf2 = sf1.clone();
        assert_eq!(sf1.epoch, sf2.epoch);
        assert_eq!(sf1.worker_id, sf2.worker_id);
        assert_eq!(sf1.datacenter_id, sf2.datacenter_id);
        assert_eq!(sf1.sequence.load(Ordering::Relaxed), sf2.sequence.load(Ordering::Relaxed));
    }

    #[test]
    fn test_snowflake_generation() {
        let sf = Snowflake::default();
        let id = sf.generate();
        assert_ne!(id, 0);
    }

}
