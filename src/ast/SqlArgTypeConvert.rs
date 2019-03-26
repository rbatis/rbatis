use serde_json::Value;

pub trait SqlArgTypeConvert {
    fn convert(arg: Value) -> String;
}
