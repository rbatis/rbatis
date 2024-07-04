#![allow(deprecated)]

#[deprecated(note = "please use `rbatis::plugin::page`")]
pub mod page;
pub mod column;

#[deprecated(note = "please use `rbatis::plugin::page::Page`")]
pub use page::Page;
#[deprecated(note = "please use `rbatis::plugin::page::PageRequest`")]
pub use page::PageRequest;
