pub mod string;
pub mod value;
/// this mod All common types

pub trait TypeName {
    fn type_name(&self) -> &'static str;
}
