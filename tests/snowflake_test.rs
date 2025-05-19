#[cfg(test)]
mod test {
    use rbatis::plugin::snowflake::new_snowflake_id;
    use rbatis::snowflake::Snowflake;
    use std::collections::HashMap;
    use std::thread::sleep;
    use std::time::Duration;
    use dark_std::sync::WaitGroup;
    
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
    
    #[test]
    fn test_gen() {
        let id = Snowflake::new(1, 1, 0);
        println!("{}", id.generate());
        sleep(Duration::from_secs(1));
        println!("{}", id.generate());
    }

    #[test]
    fn test_gen1() {
        let id = Snowflake::new(1, 1, 1);
        println!("{}", id.generate());
        println!("{}", id.generate());
        sleep(Duration::from_secs(1));
        println!("{}", id.generate());
        println!("{}", id.generate());
    }

    #[test]
    fn test_race() {
        let id_generator_generator = Snowflake::new(1, 1, 0);
        let size = 1000000;
        let mut v1: Vec<i64> = Vec::with_capacity(size);
        let mut v2: Vec<i64> = Vec::with_capacity(size);
        let mut v3: Vec<i64> = Vec::with_capacity(size);
        let mut v4: Vec<i64> = Vec::with_capacity(size);
        let wg = WaitGroup::new();
        std::thread::scope(|s| {
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v1.push(id_generator_generator.generate());
                }
                drop(wg1);
            });
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v2.push(id_generator_generator.generate());
                }
                drop(wg1);
            });
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v3.push(id_generator_generator.generate());
                }
                drop(wg1);
            });
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v4.push(id_generator_generator.generate());
                }
                drop(wg1);
            });
        });

        wg.wait();

        println!(
            "v1 len:{},v2 len:{},v3 len:{},v4 len:{}",
            v1.len(),
            v2.len(),
            v3.len(),
            v4.len()
        );
        let mut all: Vec<i64> = Vec::with_capacity(size * 4);
        all.append(&mut v1);
        all.append(&mut v2);
        all.append(&mut v3);
        all.append(&mut v4);

        let mut id_map: HashMap<i64, i64> = HashMap::with_capacity(all.len());
        for id in all {
            id_map
                .entry(id)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        for (_, v) in id_map {
            assert_eq!(v <= 1, true);
        }
    }

    #[test]
    fn test_generate_no_clock_back() {
        let snowflake = Snowflake::default();
        let id1 = snowflake.generate();
        let id2 = snowflake.generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_generate_clock_rollback() {
        let id_generator_generator = Snowflake::new(1, 1, 0);
        let initial_id = id_generator_generator.generate();
        println!("initial_id={}", initial_id);

        let new_id = id_generator_generator.generate();
        println!("new_id____={}", new_id);
        assert!(new_id > initial_id);
    }
}
