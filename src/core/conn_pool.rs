use std::collections::HashMap;

pub struct ConnPool {
    pub mysql_map: HashMap<String, mysql::Conn>,
    pub pg_map: HashMap<String, postgres::Client>,
}

impl ConnPool{
    pub fn new()->ConnPool{
        return Self{
            mysql_map: HashMap::new(),
            pg_map: HashMap::new(),
        }
    }
}

#[test]
pub fn test_pool() {
    let mut conn = ConnPool::new();
    let c=conn.mysql_map.get_mut(&"default".to_string());
    println!("{}",c.is_none());
    //  let v=conn.mysql_map.get_mut(&"s".to_string()).unwrap();
}

