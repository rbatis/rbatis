#[cfg(test)]
mod test {
    use rbatis::interpreter::sql::ast::RbatisAST;
    use rbatis::core::db::DriverType;
    use serde_json::Value;
    use rbatis::interpreter::sql::node::proxy_node::{CustomNodeGenerate, ProxyNode};
    use rbatis::core::Error;
    use rbatis::interpreter::sql::node::node_type::NodeType;
    use rbatis::rbatis::Rbatis;
    use crate::BizActivity;
    use rbatis::plugin::page::{Page, PageRequest};
    use rexpr::runtime::RExprRuntime;

    #[derive(Debug)]
    pub struct MyNode {}

    impl RbatisAST for MyNode {
        fn name() -> &'static str {
            "custom"
        }

        fn eval(&self, convert: &DriverType, env: &mut Value, engine: &RExprRuntime, arg_result: &mut Vec<Value>, arg_sql: &mut String) -> Result<serde_json::Value, Error> {
            *arg_sql = " AND id = 1 ".to_string();
            Ok(serde_json::Value::Null)
        }
    }

    #[derive(Debug)]
    pub struct MyGen {}

    impl CustomNodeGenerate for MyGen {
        fn generate(&self, express: &str, child_nodes: Vec<NodeType>) -> Result<Option<ProxyNode>, Error> {
            if express.starts_with(MyNode::name()) {
                return Ok(Option::from(ProxyNode::from(MyNode {}, child_nodes)));
            }
            //skip
            return Ok(None);
        }
    }

    #[async_std::test]
    pub async fn test_py_sql_custom() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let mut rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        rb.runtime_py.add_gen(MyGen {});
        let py = "
    SELECT * FROM biz_activity
    WHERE delete_flag = 0
    custom :
    ";
        let data: Page<BizActivity> = rb.py_fetch_page("", py, &serde_json::json!({}), &PageRequest::new(1, 20)).await.unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    }
}