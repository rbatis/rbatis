pub mod string;
pub mod value;
/// this mod All common types

pub trait CommonType {
    fn common_type(&self) -> &'static str;
}
