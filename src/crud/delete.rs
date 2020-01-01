use serde_json::Value;
use crate::core::rbatis::Rbatis;
use std::borrow::BorrowMut;
use crate::engine::node::NodeType;
use std::fs;
use crate::ast::xml::result_map_node::ResultMapNode;

impl Rbatis{
    pub fn delete(&mut self, mapper_name: &str, id: &str, arg: &mut Value)-> Result<String, String>{
        if arg.is_null() {
            return Result::Err("[rbatis] arg is null value".to_string());
        }

        let result_map_node=self.get_result_map_node(mapper_name,id)?;
        println!("{:?}",result_map_node);

        //TODO delete by id
        self.do_delete_by_id(mapper_name,id,arg);
        //TODO delete by map
        //TODO delete by ids

        return Result::Err("[rbatis] arg is null value".to_string());
    }

    fn do_delete_by_id(&mut self, mapper_name: &str, id: &str, env: &mut Value){
        let mut sql = "DELETE FROM #{table} #{where}".to_string();

    }
}

#[test]
fn test_delete() {
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    rbatis.delete("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!(r#"1234123123"#).borrow_mut());
}
