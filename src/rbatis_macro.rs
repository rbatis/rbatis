use crate::tx::propagation::Propagation::NONE;
use crate::error::RbatisError;

///将嵌套Result await调用后 转换为标准的 Result<T,RbatisError>
/// 使用方法
///
///  //data 即可被推断为 data:Result<Activity,RbatisError>
///  let data=to_tokio_await!(Activity,{ singleton().raw_sql(format!("{:?}",std::thread::current().id()).as_str(),"select * from biz_activity where id  = '2';")  });
///
#[macro_export(local_inner_macros)]
macro_rules! to_tokio_await {
    ($t:ty,$b:block) => {
        {
          let data=task::spawn_blocking(move || $b).await;
          let mut result_ok_data:Result<$t,RbatisError>;
           if data.is_ok(){
              result_ok_data=data.ok().unwrap();
              result_ok_data
             }else{
               Err(RbatisError::from(data.err().unwrap().description()))
            }
        }
    };
}

///代理实现服务,支持事务嵌套
/// 使用方法：
/// struct ServiceImpl {
///    select_activity: fn(s: &ServiceImpl) -> Result<Activity, RbatisError>,
/// }
///
///impl Service for ServiceImpl {
///    impl_service! {
///      //这里逗号前面是事务传播行为(详见tx::Propagation枚举中的定义)，后面是结构体定义的方法
///      REQUIRED,  select_activity(&self) -> Result<Activity,RbatisError>
///    }
///  }
#[macro_export(local_inner_macros)]
macro_rules! impl_service {
   ($($p:path,  $fn: ident (&self $(,$x:ident:$t:ty)*         ) -> Result<$return_type:ty,$return_type_error:ty> ),*) => {
    $(
    fn $fn(&self  $(,$x:$t)*    ) -> Result<$return_type,$return_type_error> {
           //TODO 判断是否启用事务，启用则根据事务最后一条传播行为创建。
           if $p!=crate::tx::propagation::Propagation::NONE{
              singleton().begin( "", $p)?;
           }
           let data = (self.$fn)(self  $(,$x)*    );
           if $p!=crate::tx::propagation::Propagation::NONE{
              if data.is_ok(){
                singleton().commit("")?;
              }else{
                singleton().rollback("")?;
              }
           }
           return data;
        }
    )*
   }
}

///代理实现服务,支持事务嵌套
/// 使用方法：
/// struct ServiceImpl {
///    update_activity: fn(s: &mut ServiceImpl) -> Result<String, RbatisError>,
///}
///
///impl Service for ServiceImpl {
///    impl_service_mut! {
///      //这里逗号前面是事务传播行为(详见tx::Propagation枚举中的定义)，后面是结构体定义的方法
///      NONE,  update_activity(&mut self) -> Result<String, RbatisError>
///    }
///  }
#[macro_export(local_inner_macros)]
macro_rules! impl_service_mut {
   ($($p:path,  $fn: ident (&mut self $(,$x:ident:$t:ty)*         ) -> Result<$return_type:ty,$return_type_error:ty> ),*) => {
    $(
    fn $fn(&mut self  $(,$x:$t)*    ) -> Result<$return_type,$return_type_error> {
            //TODO 判断是否启用事务，启用则根据事务最后一条传播行为创建。
            if $p!=crate::tx::propagation::Propagation::NONE{
              singleton().begin( "", $p)?;
           }
            let data = (self.$fn)(self  $(,$x)*    );
            if $p!=crate::tx::propagation::Propagation::NONE{
              if data.is_ok(){
                singleton().commit("")?;
              }else{
                singleton().rollback("")?;
              }
           }
            return data;
        }
    )*
   }
}
