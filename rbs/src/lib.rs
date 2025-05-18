#[macro_use]
extern crate serde;
extern crate core;

pub mod index;
pub mod value;

mod value_serde;
mod error;
mod macros;

pub use crate::error::Error;
pub use value_serde::{from_value, from_value_ref};
pub use value_serde::{to_value, to_value_def};
pub use value::Value;

impl Value {
    pub fn into_ext(self, name: &'static str) -> Self {
        match self {
            Value::Ext(_, _) => self,
            _ => Value::Ext(name, Box::new(self)),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Value::Null => true,
            Value::Bool(_) => false,
            Value::I32(_) => false,
            Value::I64(_) => false,
            Value::U32(_) => false,
            Value::U64(_) => false,
            Value::F32(_) => false,
            Value::F64(_) => false,
            Value::String(v) => v.is_empty(),
            Value::Binary(v) => v.is_empty(),
            Value::Array(v) => v.is_empty(),
            Value::Map(v) => v.is_empty(),
            Value::Ext(_, v) => v.is_empty(),
        }
    }

    /// return array/map/string's length
    pub fn len(&self) -> usize {
        match self {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::I32(_) => 0,
            Value::I64(_) => 0,
            Value::U32(_) => 0,
            Value::U64(_) => 0,
            Value::F32(_) => 0,
            Value::F64(_) => 0,
            Value::String(v) => v.len(),
            Value::Binary(v) => v.len(),
            Value::Array(v) => v.len(),
            Value::Map(v) => v.len(),
            Value::Ext(_, v) => v.len(),
        }
    }
}


/// is debug mode
pub fn is_debug_mode() -> bool {
    if cfg!(debug_assertions) {
        #[cfg(feature = "debug_mode")]
        {
            true
        }
        #[cfg(not(feature = "debug_mode"))]
        {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod test_utils {
    use crate::value::map::ValueMap;
    use crate::Value;
    
    #[test]
    fn test_nested_structure() {
        // 使用手动构建的方式来测试嵌套结构
        let mut street_map = ValueMap::new();
        street_map.insert("number".into(), 123.into());
        
        let mut address_map = ValueMap::new();
        address_map.insert("city".into(), "Beijing".into());
        address_map.insert("street".into(), Value::Map(street_map));
        
        let mut user_map = ValueMap::new();
        user_map.insert("name".into(), "Alice".into());
        user_map.insert("address".into(), Value::Map(address_map));
        
        let mut root_map = ValueMap::new();
        root_map.insert("id".into(), 1.into());
        root_map.insert("user".into(), Value::Map(user_map));
        
        let value = Value::Map(root_map);
        
        // 验证结构正确
        assert!(value.is_map());
        let map = value.as_map().unwrap();
        assert_eq!(map["id"].as_i64().unwrap(), 1);
        
        // 验证嵌套的user结构
        assert!(map["user"].is_map());
        let user = map["user"].as_map().unwrap();
        assert_eq!(user["name"].as_str().unwrap(), "Alice");
        
        // 验证嵌套的address结构
        assert!(user["address"].is_map());
        let address = user["address"].as_map().unwrap();
        assert_eq!(address["city"].as_str().unwrap(), "Beijing");
        
        // 验证嵌套的street结构
        assert!(address["street"].is_map());
        let street = address["street"].as_map().unwrap();
        assert_eq!(street["number"].as_i64().unwrap(), 123);
    }
} 