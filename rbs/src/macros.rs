/// to_value macro
///
/// to_value! map
///```rust
/// let v=  rbs::to_value! {"1":"1",};
///```
/// to_value! expr
///```rust
/// let arg="1";
/// let v =  rbs::to_value!(arg);
///```
/// 
/// JSON example:
/// ```ignore
/// let v = rbs::to_value! {
///     "id": 1, 
///     "user": {
///         "name": "Alice",
///         "address": {
///             "city": "Beijing",
///             "street": {
///                 "number": 123
///             }
///         }
///     }
/// };
/// ```
#[macro_export]
macro_rules! to_value {

    // object inner {}
    ($($k:tt: {$($ik:tt: $iv:tt),* $(,)*}),* $(,)*) => {
        {
            let mut map = $crate::value::map::ValueMap::new();
            $(
                let inner_value = $crate::to_value!({$($ik: $iv),*});
                map.insert($crate::to_value!($k), inner_value);
            )*
            $crate::Value::Map(map)
        }
    };
    
    // object
    ({$($k:tt: $v:tt),* $(,)*}) => {
        {
            let mut map = $crate::value::map::ValueMap::new();
            $(
                map.insert($crate::to_value!($k), $crate::to_value!($v));
            )*
            $crate::Value::Map(map)
        }
    };
    
    // k-v
    ($($k:tt: $v:expr),* $(,)?) => {
        {
            let mut map = $crate::value::map::ValueMap::new();
            $(
                map.insert($crate::to_value!($k), $crate::to_value!($v));
            )*
            $crate::Value::Map(map)
        }
    };
    
    // expr/ident
    ($arg:expr) => {
        $crate::to_value($arg).unwrap_or_default()
    };
    
    // array [*,*]
    [$($arg:tt),*] => {
        $crate::to_value([$($arg),*]).unwrap_or_default()
    };
}