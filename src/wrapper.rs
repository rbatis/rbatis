
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//TODO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wrapper{
    pub sql:String,
    pub args:Vec<serde_json::Value>
}

impl Wrapper{

}