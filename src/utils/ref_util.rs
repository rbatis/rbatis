/// Used for concise syntax to create a Ref structure corresponding to a data table for data querying.
#[macro_export]
macro_rules! table_own_ref {
    ($struct_name:ident {$($key:ident:$val:expr),*$(,)?})=>{
        {
            paste::paste!{
                [<$struct_name Ref>]{
                    $(
                        $key:Some(std::borrow::Cow::Owned($val)),
                    )*
                    ..[<$struct_name Ref>]::default()
                }
            }
        }
    };
}

/// Used for concise syntax to create a Ref structure corresponding to a data table for data querying.
#[macro_export]
macro_rules! table_ref {
    ($struct_name:ty {$($key:ident:$val:expr),*$(,)?})=>{
        {
            paste::paste!{
                [<$struct_name Ref>]{
                    $(
                        $key:Some($val),
                    )*
                    ..[<$struct_name Ref>]::default()
                }
            }
        }
    };
}
