
#[test]
pub fn test_raw_identifiers() {
    use rbatis::crud::CRUDEnable;
    #[crud_enable]
    #[derive(Clone, Debug)]
    pub struct BizActivity {
        pub id: Option<String>,
        // pub type: Option<String>, // type is a keyword, so need to named `r#type`.
        pub r#type: Option<String>,
    }
    assert_eq!("id,type".to_string(), BizActivity::table_columns());
}