#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unreachable_code)]
#![allow(clippy::len_zero)]
#![allow(clippy::modulo_one)]
#![allow(clippy::match_like_matches_macro)]

pub mod error;
#[macro_use]
pub mod bencher;
pub mod ops;
pub mod ops_cmp;
pub mod ops_eq;

pub mod from_bool;
pub mod ops_add;
pub mod ops_bit_and;
pub mod ops_bit_or;
pub mod ops_div;
pub mod ops_mul;
pub mod ops_not;
pub mod ops_rem;
pub mod ops_sub;
pub mod ops_xor;
pub mod codegen;

pub use codegen::{rb_html, rb_py};
