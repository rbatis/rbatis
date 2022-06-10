#[cfg(test)]
mod test {
    use std::collections::{BTreeMap};
    use rbson::Bson;
    use rbatis::core::db::DriverType;
    use rbatis::wrapper::Wrapper;

    #[test]
    fn test_item() {
        let w = Wrapper::new(&DriverType::Postgres).having("id");
        assert_eq!(w.sql, "having id");
        let mut m = BTreeMap::new();
        m.insert("id", 2);
        m.insert("name", 1);
        let w = Wrapper::new(&DriverType::Postgres).eq_all(m);
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
        let w = Wrapper::new(&DriverType::Postgres)
            .having("id").and()
            .eq_all(m).and()
            .eq("id", 1).and()
            .ne("id", 1).and()
            .gt("id", 1).and()
            .ge("id", 1).and()
            .lt("id", 1).and()
            .le("id", 1).and()
            .between("id", 1, 2).and()
            .not_between("id", 1, 2).and()
            .like("id", 1).and()
            .like_left("id", 1).and()
            .like_right("id", 1).and()
            .not_like("id", 1).and()
            .is_null("id").and()
            .is_not_null("id").and()
            .in_array("id", &[1]).and()
            .not_in("id", &[1]).and()
            .order_by(true, &["id"])
            .group_by(&["id"])
            .limit(1)
            .order_bys(&[("id", true), ("name", false)]);
        assert_eq!(w.sql, "having id and (id = $1 and name = $2) and id = $3 and id <> $4 and id > $5 and id >= $6 and id < $7 and id <= $8 and id between $9 and $10 and id not between $11 and $12 and id like $13 and id like $14 and id like $15 and id not like $16 and id is NULL and id is not NULL and id in ( $17 ) and id not in ( $18 ) order by id asc group by id  limit 1 order by id asc,name desc ");
    }

    #[test]
    fn test_wrapper_add() {
        let w = Wrapper::new(&DriverType::Postgres)
            + "and id = 1 "
            + (" and id = ? ", Bson::Int32(2))
            + (" and id = ? or name = ? ", vec![Bson::Int32(3), Bson::String("joe".to_string())]);
        println!("{}", w.sql);
        println!("{:?}", w.args);
        assert_eq!(w.sql, "and id = 1  and id = ?  and id = ? or name = ? ");
        assert_eq!(w.args, vec![Bson::Int32(2), Bson::Int32(3), Bson::String("joe".to_string())])
    }
}
