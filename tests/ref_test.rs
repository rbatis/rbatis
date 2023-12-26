#[cfg(test)]
#[cfg(feature = "ref_model")]
mod test {
    use rbatis::{impl_insert_ref_exp, impl_update_ref_exp, table_own_ref, table_ref};
    use rbatis_macro_driver::RefModel;
    use std::borrow::Cow;

    #[test]
    fn test_ref_model() {
        /// This structure is consistent with the database table structure, add, delete, and check using the generated Ref structure
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, RefModel)]
        pub struct MockTable {
            /// Primary key id, can't not exist
            pub id: String,
            /// Remarks, can be empty
            pub name: Option<String>,
        }

        impl_insert_ref_exp!(MockTable{});
        impl_update_ref_exp!(MockTable{});


        // Generate shortcut macros for refs, reducing the need to write multiple None
        let new = table_ref!(MockTable {
            name: Cow::Owned(None)
        });
        let new2 = table_own_ref!(MockTable {
            name: None
        });

        // With this data insertion, you can specify that a lot inserts Null
        // For example, name
        let v2 = MockTableRef {
            id: None,
            name: Some(Cow::Owned(None)),
        };

        assert_eq!(new, v2);
        assert_eq!(new2, v2);
        println!("{:?}",v2);



    }
}
