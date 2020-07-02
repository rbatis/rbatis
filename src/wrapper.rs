use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use crate::crud::CRUDEntity;

//TODO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wrapper {
    pub sql: String,
    pub args: Vec<serde_json::Value>,
}

impl Wrapper{
    pub fn select(columns: &str,table_name: &str) -> Self {
        Self {
            sql: format!("SELECT {} FROM {}", columns, table_name),
            args: vec![],
        }
    }
    pub fn update(sets: &str,table_name: &str) -> Self{
        Self {
            sql: format!("UPDATE {} SET {}", table_name, sets),
            args: vec![],
        }
    }

    pub fn all_eq(&mut self, arg: &Map<String, serde_json::Value>) -> &mut Self {
        self
    }

    pub fn order_by(&mut self, condition: bool, is_asc: bool, columns: &str) -> &mut Self {
        self
    }
}

mod test {
    use crate::wrapper::Wrapper;
    use serde_json::Map;

    #[test]
    fn test_select() {
        let w = Wrapper::select("*","biz_activity")
            .all_eq(&Map::new())
            .order_by(true, true, "");
    }
}