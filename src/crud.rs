
///gen sql => INSERT INTO table_name (column1,column2,column3,...) VALUES (value1,value2,value3,...);
///
/// example:
/// pub struct BizActivity{}
///
/// impl_insert!(BizActivity,"biz_activity");
///
/// let table = BizActivity{}
/// table.insert()
///
#[macro_export]
macro_rules! impl_insert {
    ($table:ty) => {
        $crate::impl_insert!($table,$crate::utils::string_util::to_snake_name(stringify!($table)));
    };
    ($table:ty,$table_name:expr) => {
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


///gen sql => SELECT (column1,column2,column3,...) FROM table_name (column1,column2,column3,...)  *** WHERE ***
///
/// example:
/// pub struct BizActivity{}
///
/// impl_insert!(BizActivity,"biz_activity");
///
/// let table = BizActivity{}
/// BizActivity::select()
///
#[macro_export]
macro_rules! impl_select_all {
    ($table:ty) => {
        $crate::impl_select_all!($table,$crate::utils::string_util::to_snake_name(stringify!($table)));
    };
    ($table:ty,$table_name:expr) => {
        impl $table{
            pub async fn select_all(mut rb: $crate::executor::RbatisExecutor<'_>)->Result<Vec<$table>,rbdc::Error>{
                #[py_sql(
"select * from ${table_name}")]
async fn do_select_all(mut rb: $crate::executor::RbatisExecutor<'_>,table_name:String) -> Result<Vec<$table>,rbdc::Error> {impled!()}
            let table_name = $table_name.to_string();
            do_select_all(rb,table_name).await
            }
        }
    };
}