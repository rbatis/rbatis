///We can use rbatis built-in macros to improve maintainability
#[cfg(test)]
mod test {
    use rbatis::DriverType;
    use rbatis::wrapper::Wrapper;

    #[crud_table]
    #[derive(Clone, Debug)]
    pub struct BizActivity {
        pub id: Option<String>,
        pub name: Option<String>,
        pub delete_flag: Option<i32>,
    }

    #[test]
    fn test_improve_maintainability() {
        // When name is changed to user_name, the code becomes unmaintainable,
        // compiling well but with logic errors
        let w = Wrapper::new(&DriverType::Mysql)
            .eq("name", "xiao ming");
        // so we change "name" to column_name!(BizActivity::name),
        // when field name change to user_name ,it will be Compilation fails
        let w = Wrapper::new(&DriverType::Mysql)
            .eq(field_name!(BizActivity.name), "xiao ming");
    }
}