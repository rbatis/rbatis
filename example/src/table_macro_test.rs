#[cfg(test)]
mod test {
    use bigdecimal::BigDecimal;
    use chrono::NaiveDateTime;

    use rbatis::utils::table_util::FatherChildRelationship;

    #[crud_enable]
    #[derive(Clone, Debug)]
    pub struct BizActivity {
        pub id: Option<String>,
        pub name: Option<String>,
        pub pc_link: Option<String>,
        pub h5_link: Option<String>,
        pub pc_banner_img: Option<String>,
        pub h5_banner_img: Option<String>,
        pub sort: Option<String>,
        pub status: Option<i32>,
        pub remark: Option<String>,
        pub create_time: Option<NaiveDateTime>,
        pub version: Option<BigDecimal>,
        pub delete_flag: Option<i32>,
    }

    impl Default for BizActivity {
        fn default() -> Self {
            Self {
                id: None,
                name: None,
                pc_link: None,
                h5_link: None,
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: None,
                remark: None,
                create_time: None,
                version: None,
                delete_flag: None,
            }
        }
    }

    #[test]
    fn test_make_table() {
        let table = rbatis::make_table!(BizActivity{
              id:"1".to_string(),
        });
        println!("{:#?}", table);
    }

    #[test]
    fn test_table_field_map() {
        let table = rbatis::make_table!(BizActivity{
              id:"1".to_string(),
              name:"a".to_string()
        });
        let table_vec = vec![table];
        let map = rbatis::make_table_field_map!(&table_vec,name);
        println!("{:#?}", map);
        assert_eq!(map.len(), table_vec.len());
    }

    #[test]
    fn test_table_field_vec() {
        let table = rbatis::make_table!(BizActivity{
              id:"1".to_string(),
              name:"a".to_string()
        });
        let table_vec = vec![table];
        let names = rbatis::make_table_field_vec!(&table_vec,name);
        println!("{:#?}", names);
        assert_eq!(names.len(), table_vec.len());
    }


    #[crud_enable]
    #[derive(Clone, Debug)]
    pub struct FatherChildVO {
        pub id: Option<i32>,
        pub father_id: Option<i32>,
        pub childs: Vec<FatherChildVO>,
    }

    impl FatherChildRelationship for FatherChildVO {
        fn get_father_id(&self) -> Option<&Self::IdType> {
            self.father_id.as_ref()
        }
        fn set_childs(&mut self, arg: Vec<Self>) {
            self.childs = arg;
        }
    }

    #[test]
    fn test_to_father_child_relationship() {
        let mut father = FatherChildVO {
            id: Some(1),
            father_id: None,
            childs: vec![],
        };
        let child = FatherChildVO {
            id: Some(2),
            father_id: Some(1),
            childs: vec![],
        };
        let all_record = rbatis::make_table_field_map!(vec![father.clone(),child.clone()],id);
        father.recursive_set_childs(&all_record);
        println!("{:#?}", father);
    }
}