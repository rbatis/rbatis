mod ast;
mod utils;
mod engines;
mod lib;
mod SqlBuilder;

use self::utils::time_util;
use self::ast::NodeString::NodeString;
//use utils::TimeUtil;
use chrono::Local;
use std::collections::HashMap;
use ast::Node::Node;
use std::fs::File;
use std::io::{Read, BufReader};
use xml::EventReader;
use xml::reader::XmlEvent;
use std::fs;

//fn main() {
//    utils::print_util::print_rust_mybatis();
//    let now=Local::now();
//
//
//
//    time_util::count_time(1, now);
//}



#[derive(Debug)]
struct Foo(i32);
#[derive(Debug)]
struct FooBar(i32,i32);

trait New:Clone{
    fn new(i:i32)->Self;
}

impl New for Foo{
    fn new(i:i32)->Foo{
        Foo(i)
    }
}

impl New for FooBar{
    fn new(i:i32)->FooBar{
        FooBar(i,i+10)
    }
}
impl Clone for FooBar{
    fn clone(&self) -> Self {
        return FooBar{
            0: self.0,
            1: self.1
        }
    }
}

impl Clone for Foo{
    fn clone(&self) -> Self {
        return Self{
            0: self.0,
        }
    }
}

struct Bar;

impl Bar{
    fn bar<T:New>(&self,i:i32)->T{
        T::new(i)
    }
}

fn bar2<T:New>(i:i32)->T{
    T::new(i)
}

fn main() {

    let b = Bar;
    //b.bar(1)返回的类型根据f1的类型提示自动选择
    let f1:Foo = b.bar(1);
    println!("{:?}",f1);

    let fb:FooBar = b.bar(2);
    println!("{:?}",fb);

    let f2:Foo = bar2(10);
    println!("{:?}",f2);

    let fb2:FooBar = b.bar(20);
    println!("{:?}",fb2);
}