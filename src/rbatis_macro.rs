use crate::tx::propagation::Propagation::NONE;

///代理实现服务,支持事务嵌套
#[macro_export(local_inner_macros)]
macro_rules! impl_service {
   ($($p:path,  $fn: ident (&self $(,$x:ident:$t:ty)*         ) -> Result<$return_type:ty,String> ),*) => {
    $(
    fn $fn(&self  $(,$x:$t)*    ) -> Result<$return_type,String> {
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
#[macro_export(local_inner_macros)]
macro_rules! impl_service_mut {
   ($($p:path,  $fn: ident (&mut self $(,$x:ident:$t:ty)*         ) -> Result<$return_type:ty,String> ),*) => {
    $(
    fn $fn(&mut self  $(,$x:$t)*    ) -> Result<$return_type,String> {
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
