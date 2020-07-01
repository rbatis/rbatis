use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

//TODO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wrapper {
    pub sql: String,
    pub args: Vec<serde_json::Value>,
}

impl Wrapper {

    pub fn select(columns:&str)->Self{
         Self{
             sql: format!("SELECT {} ",columns),
             args: vec![]
         }
    }
    pub fn update(columns:&str)->Self{
        Self{
            sql:  format!("UPDATE {} ",columns),
            args: vec![]
        }
    }

    pub fn all_eq(&mut self, arg: &Map<String, serde_json::Value>) -> &mut Wrapper {


        self
    }

    pub fn order_by(&mut self, condition:bool,is_asc:bool,columns:&str) -> &mut Wrapper {


        self
    }
}

mod test{
    use crate::wrapper::Wrapper;
    use serde_json::Map;

    #[test]
    fn test_select(){
        let w=Wrapper::select("*")
            .all_eq(&Map::new())
            .order_by(true,true,"");
    }
}