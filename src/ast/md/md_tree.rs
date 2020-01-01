use crate::ast::ast::Ast;
use serde_json::Value;
use crate::ast::config_holder::ConfigHolder;

pub struct MDTree{
  //TODO bind,choose,delete,foreach if,include insert otherwise,result_map,select,set,string,trim,update,when,where

}

impl Ast for MDTree{

    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder) -> Result<String, String> {
        unimplemented!()
    }

}

#[test]
pub fn test_md_eval(){

}