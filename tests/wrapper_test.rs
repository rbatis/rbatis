#[cfg(test)]
mod test {
    use rbatis::core::db::DriverType;
    use rbatis::wrapper::Wrapper;
    use serde_json::json;
    use serde_json::Map;

    #[test]
    fn test_trim() {
        let mut w = Wrapper::new(&DriverType::Mysql);
        w = w.push_sql(" where ").order_by(true, &["id"]);
        println!("sql:{:?}", w.sql.as_str());
        println!("arg:{:?}", w.args.clone());
        assert_eq!("order by id asc", w.sql.as_str().trim());
        println!("{:?}", w);
    }

    #[test]
    fn test_select() {
        let mut m = Map::new();
        m.insert("a".to_string(), json!("1"));
        let w = Wrapper::new(&DriverType::Postgres)
            .eq("id", 1)
            .ne("id", 1)
            .in_array("id", &[1, 2, 3])
            .not_in("id", &[1, 2, 3])
            .all_eq(&m)
            .like("name", 1)
            .or()
            .not_like("name", "asdf")
            .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")
            .group_by(&["id"])
            .order_by(true, &["id", "name"]);
        println!("sql_len:{}", w.sql.len());
        println!("sql:{:?}", w.sql.as_str());
        println!("arg:{:?}", w.args.clone());

        let ms: Vec<&str> = w.sql.matches("$").collect();
        assert_eq!(ms.len(), w.args.len());
    }


    ///cargo test --release --package rbatis --test wrapper_test test::bench_select --no-fail-fast -- --exact -Z unstable-options --show-output
    ///run with windows10:
    ///(Windows)use Time: 0.51 s,each:5100 nano/op  use QPS: 196078.431372549 QPS/s
    ///(MacOs) use Time: 1.718391272s ,each:17183 ns/op use QPS: 58193 QPS/s
    ///
    /// (windows) after
    /// use Time: 312.6553ms ,each:3126 ns/op
    /// use QPS: 319814 QPS/s
    #[test]
    fn bench_select() {
        let mut map = Map::new();
        map.insert("a".to_string(), json!("1"));
        rbatis::bench!(100000, {
            Wrapper::new(&DriverType::Mysql)
                .eq("id", 1)
                .ne("id", 1)
                .in_array("id", &[1, 2, 3])
                .r#in("id", &[1, 2, 3])
                .in_("id", &[1, 2, 3])
                .not_in("id", &[1, 2, 3])
                .all_eq(&map)
                .like("name", 1)
                .or()
                .not_like("name", "asdf")
                .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")
                .group_by(&["id"])
                .order_by(true, &["id", "name"]);
        });
    }

    #[test]
    fn test_link() {
        let w = Wrapper::new(&DriverType::Postgres).eq("a", "1");
        let w2 = Wrapper::new(&DriverType::Postgres)
            .eq("b", "2")
            .and()
            .push_wrapper(w.clone());

        println!("sql:{:?}", w2.sql.as_str());
        println!("arg:{:?}", w2.args.clone());

        let ms: Vec<&str> = w.sql.matches("$").collect();
        assert_eq!(ms.len(), w.args.len());
    }

    #[test]
    fn test_do_if() {
        let p = Option::<i32>::Some(1);
        let w = Wrapper::new(&DriverType::Postgres).do_if(p.is_some(), |w| w.eq("a", p.clone()));
        println!("sql:{:?}", w.sql.as_str());
        println!("arg:{:?}", w.args.clone());
        assert_eq!(&w.sql, "a = $1");
        assert_eq!(&w.args[0], &json!(p));
    }

    #[test]
    fn test_do_match() {
        let p = 1;
        let w = Wrapper::new(&DriverType::Postgres).do_match(
            &[
                (p == 0, |w| w.eq("0", "some")),
                (p == 1, |w| w.eq("1", "some")),
            ],
            |w| w.eq("default", "default"),
        );
        assert_eq!(&w.sql, "1 = $1");
    }

    #[test]
    fn test_wp() {
        let w = Wrapper::new(&DriverType::Postgres)
            .eq("1", "1")
            .or()
            .like("TITLE", "title")
            .or()
            .like("ORIGINAL_NAME", "saf");
        println!("sql:{:?}", w.sql.as_str());
        println!("arg:{:?}", w.args.clone());
    }

    #[test]
    fn test_push_arg() {
        let w = Wrapper::new(&DriverType::Mysql)
            .push_sql("?,?")
            .push_arg(1)
            .push_arg("asdfasdfa");
        println!("sql:{:?}", w.sql.as_str());
        println!("arg:{:?}", w.args.clone());
    }

    #[test]
    fn test_push_wrapper() {
        let w1 = Wrapper::new(&DriverType::Postgres);
        let w2 = w1.clone();

        let w2 = w1
            .eq("b", "b")
            .eq("b1", "b1")
            .eq("b2", "b2")
            .and()
            .push_wrapper(w2.push_sql("(").eq("a", "a").push_sql(")"));
        println!("sql:{:?}", w2.sql.as_str());
        println!("arg:{:?}", w2.args.clone());
        assert_eq!(w2.sql.contains("b = $1"), true);
        assert_eq!(w2.sql.contains("a = $4"), true);
    }
}
