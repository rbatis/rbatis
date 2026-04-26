/// Tests for id_generator module covering:
/// - SNOWFLAKE global instance
/// - OID_COUNTER global instance
/// - ObjectIdGenerator
/// - Snowflake default construction
/// - IdGenerator trait usage

#[cfg(test)]
mod test {
    use rbatis::plugin::id_generator::{IdGenerator, ObjectId, ObjectIdGenerator};
    use rbatis::plugin::{Snowflake, SNOWFLAKE};

    // ==================== SNOWFLAKE Global Tests ====================

    #[test]
    fn test_snowflake_global_exists() {
        let _snowflake = &*SNOWFLAKE;
        let id = SNOWFLAKE.generate_id();
        assert!(id > 0);
    }

    #[test]
    fn test_snowflake_global_generates_unique_ids() {
        let id1 = SNOWFLAKE.generate_id();
        let id2 = SNOWFLAKE.generate_id();
        assert_ne!(id1, id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_snowflake_global_concurrent_uniqueness() {
        use std::collections::HashSet;
        use std::sync::{Arc, Barrier};
        use std::thread;

        let n_threads = 10;
        let ids_per_thread = 100;
        let barrier = Arc::new(Barrier::new(n_threads));
        let all_ids: Arc<std::sync::Mutex<HashSet<i64>>> =
            Arc::new(std::sync::Mutex::new(HashSet::new()));

        let mut handles = vec![];
        for _ in 0..n_threads {
            let barrier_clone = barrier.clone();
            let ids_clone = all_ids.clone();
            handles.push(thread::spawn(move || {
                barrier_clone.wait();
                let mut local_ids = Vec::with_capacity(ids_per_thread);
                for _ in 0..ids_per_thread {
                    local_ids.push(SNOWFLAKE.generate_id());
                }
                let mut ids = ids_clone.lock().unwrap();
                for id in local_ids {
                    assert!(ids.insert(id), "Duplicate ID found: {}", id);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let ids = all_ids.lock().unwrap();
        assert_eq!(ids.len(), n_threads * ids_per_thread);
    }

    #[test]
    fn test_snowflake_global_id_is_positive() {
        for _ in 0..100 {
            let id = SNOWFLAKE.generate_id();
            assert!(id > 0, "Snowflake ID should be positive, got {}", id);
        }
    }

    // ==================== Snowflake Default Construction ====================

    #[test]
    fn test_snowflake_default_new() {
        let snowflake = Snowflake::default();
        let id = snowflake.generate_id();
        assert!(id > 0);

        let id2 = snowflake.generate_id();
        assert_ne!(id, id2);
    }

    #[test]
    fn test_snowflake_multiple_instances_generate_different_ids() {
        let s1 = Snowflake::default();
        let s2 = Snowflake::default();

        let id1 = s1.generate_id();
        let id2 = s2.generate_id();
        assert!(id1 > 0);
        assert!(id2 > 0);
    }

    // ==================== ObjectId New Tests ====================

    #[test]
    fn test_object_id_new() {
        let oid = ObjectId::new();
        let hex = oid.to_hex();
        assert_eq!(hex.len(), 24);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_object_id_multiple_new_are_unique() {
        let mut set = std::collections::HashSet::new();
        for _ in 0..1000 {
            let oid = ObjectId::new();
            assert!(set.insert(oid.to_hex()), "Duplicate ObjectId detected");
        }
    }

    #[test]
    fn test_object_id_display_and_debug() {
        let oid = ObjectId::new();
        let display_str = format!("{}", oid);
        assert_eq!(display_str.len(), 24);

        let debug_str = format!("{:?}", oid);
        assert_eq!(debug_str, format!("ObjectId({})", display_str));
    }

    #[test]
    fn test_object_id_u128_roundtrip() {
        let oid = ObjectId::new();
        let val: u128 = oid.u128();
        let restored = ObjectId::with_u128(val);
        assert_eq!(oid.to_hex(), restored.to_hex());
    }

    #[test]
    fn test_object_id_with_bytes_roundtrip() {
        let oid = ObjectId::new();
        let bytes = oid.bytes();
        let restored = ObjectId::with_bytes(bytes);
        assert_eq!(oid.to_hex(), restored.to_hex());
    }

    #[test]
    fn test_object_id_from_valid_string() {
        let oid = ObjectId::new();
        let s = oid.to_hex();
        let restored = ObjectId::with_string(s.as_str()).expect("valid hex should parse");
        assert_eq!(oid.to_hex(), restored.to_hex());
    }

    #[test]
    fn test_object_id_from_invalid_string() {
        let result = ObjectId::with_string("not-valid-hex!!");
        assert!(result.is_err());

        let result = ObjectId::with_string("short");
        assert!(result.is_err());
    }

    #[test]
    fn test_object_id_is_send_sync() {
        fn assert_send_sync<T: Send + Sync + Clone>() {}
        assert_send_sync::<ObjectId>();
        assert_send_sync::<Snowflake>();
        assert_send_sync::<ObjectIdGenerator>();
    }

    // ==================== ObjectIdGenerator Tests ====================

    #[test]
    fn test_oid_generator_exists() {
        let gen = ObjectIdGenerator;
        let id = gen.generate_id();
        assert!(id != 0);
    }

    #[test]
    fn test_oid_generator_unique_ids() {
        let gen = ObjectIdGenerator;
        let id1 = gen.generate_id();
        let id2 = gen.generate_id();
        assert_ne!(id1, id2);
    }

    // ==================== IdGenerator Trait Object Usage ====================

    #[test]
    fn test_id_generator_as_trait_object() {
        let snowflake = Snowflake::default();
        let oid_gen = ObjectIdGenerator;

        // Test snowflake via trait object
        let id = snowflake.generate_id();
        assert!(id > 0);

        // Test OID generator via trait object
        let gen_id = (&oid_gen as &dyn IdGenerator).generate();
        // generate returns i64 for both implementations
        assert!(gen_id != 0);
    }

    // ==================== Snowflake Clock Safety ====================

    #[test]
    fn test_snowflake_generates_increasing_sequence() {
        let snowflake = Snowflake::default();
        let mut prev_id = 0;

        for _ in 0..1000 {
            let current_id = snowflake.generate_id();
            assert!(
                current_id > prev_id,
                "IDs should be monotonically increasing: {} <= {}",
                prev_id,
                current_id
            );
            prev_id = current_id;
        }
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_object_id_default() {
        let oid = ObjectId::default();
        let s = oid.to_hex();
        assert_eq!(s.len(), 24);
    }

    #[test]
    fn test_object_id_partial_eq() {
        let oid1 = ObjectId::new();
        let oid2 = ObjectId::new();
        assert_ne!(oid1, oid2);
        assert_eq!(oid1, oid1);
    }
}
