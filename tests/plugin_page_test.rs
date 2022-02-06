#[cfg(test)]
mod test {
    use rbatis_core::db::DriverType;
    use rbatis::{PagePlugin, PageRequest, RbatisPackPagePlugin, RbatisPagePlugin, RbatisReplacePagePlugin};

    #[test]
    fn test_page() {
        let pack_page = RbatisReplacePagePlugin {};
        let sql = "select
        b.name,
        count (*) as total
    from
        (select * from demands where deleted_at is null) a
    left join
        (select * from demand_kinds where deleted_at is null) b
    on
        a.demand_kind_id = b.id
    group by
        b.name
    order by
        total desc";
        let (count, select) = pack_page.make_page_sql(&DriverType::Postgres, &sql, &vec![], &PageRequest::new(1, 10)).unwrap();
        println!("{}", count);
        println!("/////////////////");
        println!("{}", select);
    }

    #[test]
    fn test_all() {}
}
