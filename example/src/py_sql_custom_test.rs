#[cfg(test)]
mod test {
    use crate::BizActivity;
    use py_sql::ast::RbatisAST;
    use py_sql::node::node_type::NodeType;
    use py_sql::node::proxy_node::{NodeFactory, ProxyNode};
    use py_sql::StringConvert;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;
    use rexpr::runtime::RExprRuntime;
    use serde_json::Value;

    #[derive(Debug)]
    pub struct MyNode {}

    impl RbatisAST for MyNode {
        fn name() -> &'static str {
            "custom"
        }

        fn eval(
            &self,
            convert: &dyn StringConvert,
            env: &mut Value,
            engine: &RExprRuntime,
            arg_result: &mut Vec<Value>,
            arg_sql: &mut String,
        ) -> Result<serde_json::Value, py_sql::error::Error> {
            *arg_sql = " AND id = 1 ".to_string();
            Ok(serde_json::Value::Null)
        }
    }

    #[derive(Debug)]
    pub struct MyNodeFactory {}

    impl NodeFactory for MyNodeFactory {
        fn try_new(
            &self,
            express: &str,
            child_nodes: Vec<NodeType>,
        ) -> Result<Option<ProxyNode>, py_sql::error::Error> {
            if express.starts_with(MyNode::name()) {
                return Ok(Option::from(ProxyNode::from(MyNode {}, child_nodes)));
            }
            //skip
            return Ok(None);
        }
    }

    #[async_std::test]
    pub async fn test_py_sql_custom() {
        let wait=fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();
        let mut rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        rb.runtime_py.add_gen(MyNodeFactory {});
        let data: Page<BizActivity> = rb
            .py_fetch_page("", "
            SELECT * FROM biz_activity WHERE delete_flag = 0
            custom :
    ", &serde_json::json!({}), &PageRequest::new(1, 20))
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
        wait.exit_and_wait();
    }
}
