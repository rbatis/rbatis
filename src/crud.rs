
///gen sql => INSERT INTO table_name (column1,column2,column3,...) VALUES (value1,value2,value3,...);
///
/// example:
/// pub struct BizActivity{}
///
/// crud_insert!(BizActivity,"biz_activity");
///
#[macro_export]
macro_rules! crud_insert {
    ($table:ty,$table_name:tt) => {
        impl $table{
            pub async fn insert(&self,mut rb: $crate::executor::RbatisExecutor<'_>)->Result<rbdc::db::ExecResult,rbdc::Error>{
                #[py_sql(
"insert into ${table_name} (
             trim ',':
               for k,v in table:
                  if k == 'id' && v== null:
                    #{continue}
                 ${k},
             ) VALUES (
             trim ',':
               for k,v in table:
                  if k == 'id' && v== null:
                    #{continue}
                 #{v},
             )")]
async fn do_save(mut rb: $crate::executor::RbatisExecutor<'_>,table: &$table,table_name:String) -> Result<rbdc::db::ExecResult,rbdc::Error> {impled!()}
            let table_name = $table_name.to_string();
            do_save(rb.into(),&self,table_name).await
            }
        }
    };
}