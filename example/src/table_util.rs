use rbatis::rbdc::DateTime;
use rbatis::{table, table_field_btree, table_field_map, table_field_vec};

#[derive(serde::Serialize, serde::Deserialize,Default,Debug,Clone)]
pub struct Activity {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

fn main() {
    let tables: Vec<Activity> = vec![
        table!(Activity {
            id: Some(3),
            name: Some("3".to_string()),
        }),
        table!(Activity {
            id: Some(2),
            name: Some("2".to_string()),
        }),
        table!(Activity {
            id: Some(1),
            name: Some("1".to_string()),
        })];
    //map ref
    let hash = table_field_map!(&tables,id);
    println!("---hash={}", rbs::to_value!(hash));
    //map owned
    let hash_owned = table_field_map!(tables.clone(),id);
    println!("---hash={}", rbs::to_value!(hash_owned));
    //btree ref
    let btree = table_field_btree!(&tables,id);
    println!("---btree={}", rbs::to_value!(btree));
    //btree owned
    let btree_owned = table_field_btree!(tables.clone(),id);
    println!("---btree_owned={}", rbs::to_value!(btree_owned));
    //vec<ref>
    let ids= table_field_vec!(&tables,id);
    println!("---ids={}", rbs::to_value!(ids));
    //vec<owned>
    let ids= table_field_vec!(tables,id);
    println!("---ids owned={:?}", ids);
}