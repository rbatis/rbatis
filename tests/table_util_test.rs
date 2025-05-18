use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TestTable {
    pub id: Option<String>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub active: Option<bool>,
    pub tags: Option<Vec<String>>,
}

#[test]
fn test_table_macro() {
    // 使用table!创建对象并设置部分字段
    let test = rbatis::table!(TestTable {
        id: "123".to_string(),
        name: "test_name".to_string(),
    });
    
    // 验证设置的字段
    assert_eq!(test.id, Some("123".to_string()));
    assert_eq!(test.name, Some("test_name".to_string()));
    
    // 验证未设置的字段为默认值None
    assert_eq!(test.age, None);
    assert_eq!(test.active, None);
    assert_eq!(test.tags, None);
    
    // 使用table!创建对象并设置所有字段
    let test2 = rbatis::table!(TestTable {
        id: "456".to_string(),
        name: "test_name2".to_string(),
        age: 30,
        active: true,
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    });
    
    // 验证所有字段
    assert_eq!(test2.id, Some("456".to_string()));
    assert_eq!(test2.name, Some("test_name2".to_string()));
    assert_eq!(test2.age, Some(30));
    assert_eq!(test2.active, Some(true));
    assert_eq!(test2.tags, Some(vec!["tag1".to_string(), "tag2".to_string()]));
}

#[test]
fn test_table_field_vec() {
    // 创建测试数据
    let tables = vec![
        TestTable {
            id: Some("1".to_string()),
            name: Some("name1".to_string()),
            ..Default::default()
        },
        TestTable {
            id: Some("2".to_string()),
            name: Some("name2".to_string()),
            ..Default::default()
        },
        TestTable {
            id: None,
            name: Some("name3".to_string()),
            ..Default::default()
        },
    ];
    
    // 测试从引用中获取字段集合
    let ids_ref: Vec<&String> = rbatis::table_field_vec!(&tables, id);
    assert_eq!(ids_ref.len(), 2);
    assert_eq!(*ids_ref[0], "1".to_string());
    assert_eq!(*ids_ref[1], "2".to_string());
    
    // 测试从值中获取字段集合
    let ids: Vec<String> = rbatis::table_field_vec!(tables.clone(), id);
    assert_eq!(ids.len(), 2);
    assert_eq!(ids[0], "1".to_string());
    assert_eq!(ids[1], "2".to_string());
    
    // 测试从引用中获取另一个字段
    let names_ref: Vec<&String> = rbatis::table_field_vec!(&tables, name);
    assert_eq!(names_ref.len(), 3);
    assert_eq!(*names_ref[0], "name1".to_string());
    assert_eq!(*names_ref[1], "name2".to_string());
    assert_eq!(*names_ref[2], "name3".to_string());
}

#[test]
fn test_table_field_set() {
    // 创建测试数据
    let tables = vec![
        TestTable {
            id: Some("1".to_string()),
            name: Some("name1".to_string()),
            ..Default::default()
        },
        TestTable {
            id: Some("2".to_string()),
            name: Some("name2".to_string()),
            ..Default::default()
        },
        TestTable {
            id: Some("1".to_string()), // 重复的id，在Set中应该只出现一次
            name: Some("name3".to_string()),
            ..Default::default()
        },
    ];
    
    // 测试从引用中获取字段Set
    let ids_ref: HashSet<&String> = rbatis::table_field_set!(&tables, id);
    assert_eq!(ids_ref.len(), 2); // 因为有重复，所以只有2个元素
    assert!(ids_ref.contains(&"1".to_string()));
    assert!(ids_ref.contains(&"2".to_string()));
    
    // 测试从值中获取字段Set
    let ids: HashSet<String> = rbatis::table_field_set!(tables.clone(), id);
    assert_eq!(ids.len(), 2);
    assert!(ids.contains("1"));
    assert!(ids.contains("2"));
    
    // 测试从引用中获取另一个字段
    let names_ref: HashSet<&String> = rbatis::table_field_set!(&tables, name);
    assert_eq!(names_ref.len(), 3); // name都不同，所以有3个元素
    assert!(names_ref.contains(&"name1".to_string()));
    assert!(names_ref.contains(&"name2".to_string()));
    assert!(names_ref.contains(&"name3".to_string()));
}

#[test]
fn test_table_field_map() {
    // 创建测试数据
    let tables = vec![
        TestTable {
            id: Some("1".to_string()),
            name: Some("name1".to_string()),
            ..Default::default()
        },
        TestTable {
            id: Some("2".to_string()),
            name: Some("name2".to_string()),
            ..Default::default()
        },
        TestTable {
            id: None,
            name: Some("name3".to_string()),
            ..Default::default()
        },
    ];
    
    // 测试从引用创建引用Map
    let map_ref: HashMap<String, &TestTable> = rbatis::table_field_map!(&tables, id);
    assert_eq!(map_ref.len(), 2); // id为None的不会被包含
    assert!(map_ref.contains_key("1"));
    assert!(map_ref.contains_key("2"));
    assert_eq!(map_ref["1"].name, Some("name1".to_string()));
    assert_eq!(map_ref["2"].name, Some("name2".to_string()));
    
    // 测试从值创建所有权Map
    let map: HashMap<String, TestTable> = rbatis::table_field_map!(tables.clone(), id);
    assert_eq!(map.len(), 2);
    assert!(map.contains_key("1"));
    assert!(map.contains_key("2"));
    assert_eq!(map["1"].name, Some("name1".to_string()));
    assert_eq!(map["2"].name, Some("name2".to_string()));
}

#[test]
fn test_table_field_btree() {
    // 创建测试数据
    let tables = vec![
        TestTable {
            id: Some("1".to_string()),
            name: Some("name1".to_string()),
            ..Default::default()
        },
        TestTable {
            id: Some("2".to_string()),
            name: Some("name2".to_string()),
            ..Default::default()
        },
        TestTable {
            id: None,
            name: Some("name3".to_string()),
            ..Default::default()
        },
    ];
    
    // 测试从引用创建引用BTreeMap
    let btree_ref: BTreeMap<String, &TestTable> = rbatis::table_field_btree!(&tables, id);
    assert_eq!(btree_ref.len(), 2); // id为None的不会被包含
    assert!(btree_ref.contains_key("1"));
    assert!(btree_ref.contains_key("2"));
    assert_eq!(btree_ref["1"].name, Some("name1".to_string()));
    assert_eq!(btree_ref["2"].name, Some("name2".to_string()));
    
    // 验证BTreeMap有序性
    let keys = btree_ref.keys().collect::<Vec<&String>>();
    assert_eq!(*keys[0], "1".to_string());
    assert_eq!(*keys[1], "2".to_string());
    
    // 测试从值创建所有权BTreeMap
    let btree: BTreeMap<String, TestTable> = rbatis::table_field_btree!(tables.clone(), id);
    assert_eq!(btree.len(), 2);
    assert!(btree.contains_key("1"));
    assert!(btree.contains_key("2"));
    assert_eq!(btree["1"].name, Some("name1".to_string()));
    assert_eq!(btree["2"].name, Some("name2".to_string()));
}

#[test]
fn test_field_name() {
    // 测试字段名称获取
    let id_name = rbatis::field_name!(TestTable.id);
    assert_eq!(id_name, "id");
    
    let name_name = rbatis::field_name!(TestTable.name);
    assert_eq!(name_name, "name");
    
    let age_name = rbatis::field_name!(TestTable.age);
    assert_eq!(age_name, "age");
} 