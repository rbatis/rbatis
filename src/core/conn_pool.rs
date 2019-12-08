use std::collections::HashMap;

pub struct ConnPool {
    pub mysql_map: HashMap<String, mysql::Conn>,
    pub pg_map: HashMap<String, postgres::Client>,
}

#[test]
pub fn test_pool() {
    let mut conn = ConnPool {
        mysql_map: HashMap::new(),
        pg_map: HashMap::new(),
    };
    conn.mysql_map.remove(&"1".to_string());
    //  let v=conn.mysql_map.get_mut(&"s".to_string()).unwrap();
}

