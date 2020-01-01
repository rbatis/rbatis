use crate::ast::ast::Ast;
use serde_json::Value;
use crate::ast::config_holder::ConfigHolder;
use std::process::exit;

pub struct YamlNode{
  //TODO bind,choose,delete,foreach if,include insert otherwise,result_map,select,set,string,trim,update,when,where

}

impl Ast for YamlNode{

    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder) -> Result<String, String> {
        unimplemented!()
    }

}


//for index,item in ids:
//for index in ids:
//<for col=ids item =item index =index >
//for_trim 'index','item',',' in ids


#[test]
pub fn test_md_eval(){

}