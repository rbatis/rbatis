use serde_json::Value;
use crate::core::rbatis::Rbatis;
use std::borrow::BorrowMut;
use crate::engine::node::NodeType;
use crate::ast::node::SqlNode;
use std::fs;

impl Rbatis{
    pub fn delete(&mut self, mapper_name: &str, id: &str, arg: &mut Value)-> Result<String, String>{
        if arg.is_null() {
            return Result::Err("[rbatis] arg is null value".to_string());
        }

        let table_name=self.get_table_name(mapper_name,id)?;

        //TODO delete by id
        self.do_delete_by_id(mapper_name,id,arg);
        //TODO delete by map
        //TODO delete by ids

        return Result::Err("[rbatis] arg is null value".to_string());
    }

    fn do_delete_by_id(&mut self, mapper_name: &str, id: &str, env: &mut Value){
        let mut sql = "DELETE FROM #{table} #{where}".to_string();

    }

    fn get_table_name(&self,mapper_name:&str,id:&str)-> Result<String, String>{
        let result_map_opt = self.mapper_map.get(mapper_name);
        if result_map_opt.is_none(){
            return Result::Err("[rbatis]  can not be find ".to_string()+mapper_name);
        }

        println!("{}",result_map_opt.clone().unwrap().len());

        let result_map = result_map_opt.unwrap();
        let base_result_map_opt= result_map.get("BaseResultMap");
        if base_result_map_opt.is_none(){
            return Result::Err("[rbatis] BaseResultMap can not be null!".to_string());
        }
        let base_result_map=base_result_map_opt.unwrap();
        println!("{}",base_result_map.print_node());

        return Result::Err("[rbatis] BaseResultMap can not be null!".to_string());
    }
}

#[test]
fn test_delete() {
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    rbatis.delete("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!(r#"1234123123"#).borrow_mut());
}
