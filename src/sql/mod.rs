#![allow(deprecated)]

#[deprecated(note = "please use `rbatis::plugin::page`")]
pub mod page;

#[deprecated(note = "please use `rbatis::plugin::page::Page`")]
pub use page::Page;
#[deprecated(note = "please use `rbatis::plugin::page::PageRequest`")]
pub use page::PageRequest;
