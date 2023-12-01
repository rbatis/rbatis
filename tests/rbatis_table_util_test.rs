#[cfg(test)]
mod test {
    use rbatis::{
        field_key, field_name, make_table, make_table_field_map, make_table_field_map_btree,
        make_table_field_vec,
    };
    use std::collections::{BTreeMap, HashMap};

    #[derive(Clone, Debug, Eq, PartialEq, Default)]
    pub struct Base {
        pub pc_banner_img: Option<String>,
        pub h5_banner_img: Option<String>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Default)]
    pub struct MockTable {
        pub base: Base,
        pub id: Option<String>,
    }
    #[test]
    fn test_make_table() {
        let t = MockTable {
            base: Base {
                pc_banner_img: Some("1".to_string()),
                h5_banner_img: None,
            },
            id: Some("id".to_string()),
        };
        let c = make_table!(MockTable {
            base: Base {
                pc_banner_img: Some("1".to_string()),
                h5_banner_img: None
            },
            id: "id".to_string(),
        });
        assert_eq!(c, t);
        println!("{:?}", c);
    }
    #[test]
    fn test_make_table_field_vec() {
        let t = MockTable {
            base: Base {
                pc_banner_img: Some("1".to_string()),
                h5_banner_img: None,
            },
            id: Some("id".to_string()),
        };
        let arr = vec![t];
        let c = make_table_field_vec!(&arr, id);
        assert_eq!(c, vec!["id".to_string()]);
        println!("{:?}", c);
    }
    #[test]
    fn test_make_table_field_vec_2() {
        let t = MockTable {
            base: Base {
                pc_banner_img: Some("1".to_string()),
                h5_banner_img: None,
            },
            id: None,
        };
        let arr = vec![t];
        let c = make_table_field_vec!(&arr, base.pc_banner_img);
        assert_eq!(c, vec!["1".to_string()]);
        println!("{:?}", c);
    }
    #[test]
    fn test_make_table_field_map() {
        let t = MockTable {
            base: Base {
                pc_banner_img: Some("1".to_string()),
                h5_banner_img: None,
            },
            id: Some("id".to_string()),
        };
        let arr = vec![t];
        let c = make_table_field_map!(&arr, id);
        assert_eq!(c, {
            let mut m = HashMap::new();
            m.insert(
                "id".to_string(),
                MockTable {
                    base: Base {
                        pc_banner_img: Some("1".to_string()),
                        h5_banner_img: None,
                    },
                    id: Some("id".to_string()),
                },
            );
            m
        });
        println!("{:?}", c);
    }
    #[test]
    fn test_make_table_field_map_2() {
        let t = MockTable {
            base: Base {
                pc_banner_img: Some("1".to_string()),
                h5_banner_img: None,
            },
            id: Some("id".to_string()),
        };
        let arr = vec![t];
        let c = make_table_field_map!(&arr, base.pc_banner_img);
        assert_eq!(c, {
            let mut m = HashMap::new();
            m.insert(
                "1".to_string(),
                MockTable {
                    base: Base {
                        pc_banner_img: Some("1".to_string()),
                        h5_banner_img: None,
                    },
                    id: Some("id".to_string()),
                },
            );
            m
        });
        println!("{:?}", c);
    }
    #[test]
    fn test_make_table_field_btree() {
        let t = MockTable {
            base: Base {
                pc_banner_img: Some("1".to_string()),
                h5_banner_img: None,
            },
            id: Some("id".to_string()),
        };
        let arr = vec![t];
        let c = make_table_field_map_btree!(&arr, id);
        assert_eq!(c, {
            let mut m = BTreeMap::new();
            m.insert(
                "id".to_string(),
                MockTable {
                    base: Base {
                        pc_banner_img: Some("1".to_string()),
                        h5_banner_img: None,
                    },
                    id: Some("id".to_string()),
                },
            );
            m
        });
        println!("{:?}", c);
    }
    #[test]
    fn test_make_table_field_btree_2() {
        let t = MockTable {
            base: Base {
                pc_banner_img: Some("1".to_string()),
                h5_banner_img: None,
            },
            id: Some("id".to_string()),
        };
        let arr = vec![t];
        let c = make_table_field_map_btree!(&arr, base.pc_banner_img);
        assert_eq!(c, {
            let mut m = BTreeMap::new();
            m.insert(
                "1".to_string(),
                MockTable {
                    base: Base {
                        pc_banner_img: Some("1".to_string()),
                        h5_banner_img: None,
                    },
                    id: Some("id".to_string()),
                },
            );
            m
        });
        println!("{:?}", c);
    }

    #[test]
    fn test_field_name() {
        let name = field_name!(MockTable.id);
        assert_eq!(name, "id");
    }
    #[test]
    fn test_field_name_2() {
        let name = field_name!(MockTable.base.pc_banner_img);
        assert_eq!(name, "pc_banner_img");
    }
    #[test]
    fn test_field_key() {
        let name = field_key!(MockTable::id);
        assert_eq!(name, "id");
    }
    #[test]
    fn test_field_key_2() {
        let name = field_key!(MockTable::base::pc_banner_img);
        assert_eq!(name, "pc_banner_img");
    }
}
