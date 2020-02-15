#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#[forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;
extern crate rbatis_drivers;
extern crate serde_yaml;

#[macro_use]
pub mod rbatis_macro;

pub mod example;
pub mod ast;
pub mod utils;
pub mod crud;
pub mod security;
pub mod convert;
pub mod server;
pub mod engine;
pub mod decode;
pub mod tx;
pub mod plugin;
pub mod error;

pub mod rbatis;
pub mod db_config;
pub mod local_session;
pub mod abstract_session;
pub mod session_factory;
pub mod adapter;
