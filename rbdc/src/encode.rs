use std::mem;

/// Encode a single value to be sent to the database.
pub trait Encode {
    fn encode_i32(&self,v: Option<i32>) -> Vec<u8>;
    fn encode_i64(&self,v: Option<i64>) -> Vec<u8>;
    fn encode_f32(&self,v: Option<f32>) -> Vec<u8>;
    fn encode_f64(&self,v: Option<f64>) -> Vec<u8>;
    fn encode_u32(&self,v: Option<u32>) -> Vec<u8>;
    fn encode_u64(&self,v: Option<u64>) -> Vec<u8>;
    fn encode_str(&self,v: Option<&str>) -> Vec<u8>;
    fn encode_bool(&self,v: Option<bool>) -> Vec<u8>;
    fn encode_bin(&self,v: Option<&[u8]>) -> Vec<u8>;
    fn encode_null(&self) -> Vec<u8>;
}