#[cfg(test)]
mod tests {
    use rbatis::snowflake::{new_snowflake_id, Snowflake};
    use std::sync::atomic::Ordering;

    #[test]
    fn test_snowflake_serialization() {
        let sf1 = Snowflake::default();
        let serialized = serde_json::to_string(&sf1).unwrap();
        let sf2: Snowflake = serde_json::from_str(&serialized).unwrap();
        assert_eq!(sf1.epoch, sf2.epoch);
        assert_eq!(sf1.worker_id, sf2.worker_id);
        assert_eq!(sf1.datacenter_id, sf2.datacenter_id);
        assert_eq!(
            sf1.sequence.load(Ordering::Relaxed),
            sf2.sequence.load(Ordering::Relaxed)
        );
        assert_eq!(
            sf1.time.load(Ordering::Relaxed),
            sf2.time.load(Ordering::Relaxed)
        );
    }

    #[test]
    fn test_snowflake_cloning() {
        let sf1 = Snowflake::default();
        let sf2 = sf1.clone();
        assert_eq!(sf1.epoch, sf2.epoch);
        assert_eq!(sf1.worker_id, sf2.worker_id);
        assert_eq!(sf1.datacenter_id, sf2.datacenter_id);
        assert_eq!(
            sf1.sequence.load(Ordering::Relaxed),
            sf2.sequence.load(Ordering::Relaxed)
        );
        assert_eq!(
            sf1.time.load(Ordering::Relaxed),
            sf2.time.load(Ordering::Relaxed)
        );
    }

    #[test]
    fn test_snowflake_generation() {
        let sf = Snowflake::default();
        let id = sf.generate();
        assert_ne!(id, 0);
    }

    #[test]
    fn test_new_snowflake_id() {
        let id = new_snowflake_id();
        assert_ne!(id, 0);
    }
}
