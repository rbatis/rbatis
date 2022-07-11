///We can use rbatis built-in macros to improve maintainability
#[cfg(test)]
mod test {
    use rbatis::wrapper::Wrapper;
    use rbatis::DriverType;

    #[crud_table]
    #[derive(Clone, Debug)]
    pub struct BizActivity {
        pub id: Option<String>,
        pub name: Option<String>,
        pub delete_flag: Option<i32>,
    }
    // this macro will create impl BizActivity{ pub fn id()->&str ..... }
    impl_field_name_method!(BizActivity {
        id,
        name,
        delete_flag
    });
    #[test]
    fn test_improve_maintainability() {
        // When name is changed to user_name, the code becomes unmaintainable,
        // compiling well but with logic errors
        let w = Wrapper::new(&DriverType::Mysql)
            .eq("id", "1")
            .eq("name", "xiao ming")
            .eq("delete_flag", 1);

        assert_eq!(w.sql, "id = ? and name = ? and delete_flag = ?");

        // so we change "name" to column_name!(BizActivity::name),
        // when field name change to user_name ,it will be Compilation fails
        let w = Wrapper::new(&DriverType::Mysql)
            .eq(field_name!(BizActivity.id), "1")
            .eq(field_name!(BizActivity.name), "xiao ming")
            .eq(field_name!(BizActivity.delete_flag), 1);

        assert_eq!(w.sql, "id = ? and name = ? and delete_flag = ?");

        let w = Wrapper::new(&DriverType::Mysql)
            .eq(BizActivity::id(), "1")
            .eq(BizActivity::name(), "xiao ming")
            .eq(BizActivity::delete_flag(), 1);

        assert_eq!(w.sql, "id = ? and name = ? and delete_flag = ?");
    }
}
