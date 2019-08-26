#![feature(test)]
#![feature(fn_traits)]
extern crate test;

pub mod eval;
pub mod node;
pub mod parser;
pub mod runtime;

//test mod
 mod parser_test;
 mod node_test;
 mod eval_test;
 mod runtime_test;