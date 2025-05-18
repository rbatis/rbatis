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
    // Handle empty object case
    ({}) => {
        $crate::Value::Map($crate::value::map::ValueMap::new())
    };
    
    // Handle empty input
    () => {
        $crate::Value::Map($crate::value::map::ValueMap::new())
    };
    
    // Handle nested objects with brace syntax {"key": {nested object}}
    // This is a general rule for handling objects with nested objects
    // Note: This rewrites the handling method, focusing on the internal block {}
    {$($k:tt: $v:tt),* $(,)*} => {
        {
            let mut map = $crate::value::map::ValueMap::new();
            $(
                $crate::to_value!(@map_entry map $k $v);
            )*
            $crate::Value::Map(map)
        }
    };
    
    // Handle object form with parentheses: to_value!({k:v})
    ({$($k:tt: $v:tt),* $(,)*}) => {
        {
            let mut map = $crate::value::map::ValueMap::new();
            $(
                $crate::to_value!(@map_entry map $k $v);
            )*
            $crate::Value::Map(map)
        }
    };
    
    // Handle key-value pairs: to_value!(k:v)
    ($($k:tt: $v:expr),* $(,)?) => {
        {
            let mut map = $crate::value::map::ValueMap::new();
            $(
                map.insert($crate::to_value!($k), $crate::to_value!($v));
            )*
            $crate::Value::Map(map)
        }
    };
    
    // Internal helper rule: handle key-value pairs in a map
    (@map_entry $map:ident $k:tt {$($ik:tt: $iv:tt),* $(,)*}) => {
        // Process nested object
        let inner_map = $crate::to_value!({$($ik: $iv),*});
        $map.insert($crate::to_value!($k), inner_map);
    };
    
    // Handle regular key-value pairs
    (@map_entry $map:ident $k:tt $v:tt) => {
        $map.insert($crate::to_value!($k), $crate::to_value!($v));
    };
    
    // Handle single expression
    ($arg:expr) => {
        $crate::to_value($arg).unwrap_or_default()
    };
    
    // Array syntax: to_value![a, b, c]
    [$($v:expr),* $(,)*] => {
        {
            // Use to_value function directly to handle arrays, avoiding recursive expansion
            $crate::to_value(vec![$($v),*]).unwrap_or_default()
        }
    };
}