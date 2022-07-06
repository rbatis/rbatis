pub trait Decode {
    fn decode_i32(&mut self, v: Option<i32>);
    fn decode_i64(&mut self, v: Option<i64>);
    fn decode_f32(&mut self, v: Option<f32>);
    fn decode_f64(&mut self, v: Option<f64>);
    fn decode_u32(&mut self, v: Option<u32>);
    fn decode_u64(&mut self, v: Option<u64>);
    fn decode_str(&mut self, v: Option<&str>);
    fn decode_bool(&mut self, v: Option<bool>);
    fn decode_bin(&mut self, v: Option<&[u8]>);
    fn decode_null(&mut self);
}