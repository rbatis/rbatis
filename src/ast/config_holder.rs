use std::rc::Rc;
use crate::engine;
use serde_json::Value;

use engine::node::Node;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone,Debug)]
pub struct ConfigHolder {
    pub engine: RbatisEngine,
}

impl ConfigHolder {
    pub fn new() -> Self{
        let engine= RbatisEngine::new();
        return ConfigHolder {
            engine:engine,
        }
    }
}