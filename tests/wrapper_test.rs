#[cfg(test)]
mod test {
    use rbatis::core::db::DriverType;
    use rbatis::wrapper::Wrapper;
    use std::collections::{BTreeMap, HashMap};

    #[test]
    fn test_item() {
        let w = Wrapper::new(&DriverType::Postgres).having("id");
        assert_eq!(w.sql, "having id");
        let mut m = BTreeMap::new();
        m.insert("id", 2);
        m.insert("name", 1);
        let w = Wrapper::new(&DriverType::Postgres).all_eq(m);
        assert_eq!(w.sql, "(id = $1 and name = $2)");
        let w = Wrapper::new(&DriverType::Postgres).eq("id", 1);
        assert_eq!(w.sql, "id = $1");
        let w = Wrapper::new(&DriverType::Postgres).ne("id", 1);
        assert_eq!(w.sql, "id <> $1");
        let w = Wrapper::new(&DriverType::Postgres).order_by(true, &["id"]);
        assert_eq!(w.sql, " order by id asc ");
        let w = Wrapper::new(&DriverType::Postgres).group_by(&["id"]);
        assert_eq!(w.sql, " group by id ");
        let w = Wrapper::new(&DriverType::Postgres).gt("id", 1);
        assert_eq!(w.sql, "id > $1");
        let w = Wrapper::new(&DriverType::Postgres).ge("id", 1);
        assert_eq!(w.sql, "id >= $1");
        let w = Wrapper::new(&DriverType::Postgres).lt("id", 1);
        assert_eq!(w.sql, "id < $1");
        let w = Wrapper::new(&DriverType::Postgres).le("id", 1);
        assert_eq!(w.sql, "id <= $1");
        let w = Wrapper::new(&DriverType::Postgres).between("id", 1, 2);
        assert_eq!(w.sql, "id between $1 and $2");
        let w = Wrapper::new(&DriverType::Postgres).not_between("id", 1, 2);
        assert_eq!(w.sql, "id not between $1 and $2");
        let w = Wrapper::new(&DriverType::Postgres).like("id", 1);
        assert_eq!(w.sql, "id like $1");
        let w = Wrapper::new(&DriverType::Postgres).like_left("id", 1);
        assert_eq!(w.sql, "id like $1");
        let w = Wrapper::new(&DriverType::Postgres).like_right("id", 1);
        assert_eq!(w.sql, "id like $1");
        let w = Wrapper::new(&DriverType::Postgres).not_like("id", 1);
        assert_eq!(w.sql, "id not like $1");
        let w = Wrapper::new(&DriverType::Postgres).is_null("id");
        assert_eq!(w.sql, "id is NULL");
        let w = Wrapper::new(&DriverType::Postgres).is_not_null("id");
        assert_eq!(w.sql, "id is not NULL");
        let w = Wrapper::new(&DriverType::Postgres).in_array("id", &[1]);
        assert_eq!(w.sql, "id in ( $1 )");
        let w = Wrapper::new(&DriverType::Postgres).not_in("id", &[1]);
        assert_eq!(w.sql, "id not in ( $1 )");
        let w = Wrapper::new(&DriverType::Postgres).insert_into("table", "c", "v");
        assert_eq!(w.sql, "insert into table (c) values (v)");
        let w = Wrapper::new(&DriverType::Postgres).limit(1);
        assert_eq!(w.sql, " limit 1 ");
        let w = Wrapper::new(&DriverType::Postgres).order_bys(&[("id", true), ("name", false)]);
        assert_eq!(w.sql, " order by id asc,name desc ");
    }

    #[test]
    fn test_all() {
        let mut m = BTreeMap::new();
        m.insert("id", 2);
        m.insert("name", 1);
        let mut w = Wrapper::new(&DriverType::Postgres);
        w = w.having("id");
        w = w.all_eq(m);
        w = w.eq("id", 1);
        w = w.ne("id", 1);
        w = w.gt("id", 1);
        w = w.ge("id", 1);
        w = w.lt("id", 1);
        w = w.le("id", 1);
        w = w.between("id", 1, 2);
        w = w.not_between("id", 1, 2);
        w = w.like("id", 1);
        w = w.like_left("id", 1);
        w = w.like_right("id", 1);
        w = w.not_like("id", 1);
        w = w.is_null("id");
        w = w.is_not_null("id");
        w = w.in_array("id", &[1]);
        w = w.not_in("id", &[1]);
        w = w.order_by(true, &["id"]);
        w = w.group_by(&["id"]);
        w = w.limit(1);
        w = w.order_bys(&[("id", true), ("name", false)]);
        assert_eq!(w.sql, "having id and (id = $1 and name = $2) and id = $3 and id <> $4 and id > $5 and id >= $6 and id < $7 and id <= $8 and id between $9 and $10 and id not between $11 and $12 and id like $13 and id like $14 and id like $15 and id not like $16 and id is NULL and id is not NULL and id in ( $17 ) and id not in ( $18 ) order by id asc group by id  limit 1 order by id asc,name desc ");
    }

    #[test]
    fn test_hash_map() {
        let mut n = HashMap::new();
        n.insert("id", 3);
        n.insert("name", 8);

        let mut w = Wrapper::new(&DriverType::Postgres);
        w = w.all_eq(n);
    }
}
