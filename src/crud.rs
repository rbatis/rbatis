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
            pub async fn insert(mut rb: $crate::executor::RbatisExecutor<'_>,table: &$table)->Result<rbdc::db::ExecResult,rbdc::Error>{
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
async fn do_insert(mut rb: $crate::executor::RbatisExecutor<'_>,table: &$table,table_name:String) -> Result<rbdc::db::ExecResult,rbdc::Error> {impled!()}
            let table_name = $table_name.to_string();
            do_insert(rb.into(),table,table_name).await
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
    ($table:ty,$fn_name:ident,$sql:expr,$param_key:ident:$param_value:ty) => {
        impl $table{
            pub async fn $fn_name(mut rb: $crate::executor::RbatisExecutor<'_>,$param_key:$param_value)->Result<Vec<$table>,rbdc::Error>{
                #[py_sql($sql)]
async fn do_select_all(mut rb: $crate::executor::RbatisExecutor<'_>,$param_key:$param_value) -> Result<Vec<$table>,rbdc::Error> {impled!()}
            do_select_all(rb,$param_key).await
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
macro_rules! impl_select_one {
    ($table:ty,$fn_name:ident,$sql:expr,$param_key:ident:$param_value:ty) => {
        impl $table{
            pub async fn $fn_name(mut rb: $crate::executor::RbatisExecutor<'_>,$param_key:$param_value)->Result<Option<$table>,rbdc::Error>{
                #[py_sql($sql)]
async fn do_select_one(mut rb: $crate::executor::RbatisExecutor<'_>,$param_key:$param_value) -> Result<Option<$table>,rbdc::Error> {impled!()}
            do_select_one(rb,$param_key).await
            }
        }
    };
}