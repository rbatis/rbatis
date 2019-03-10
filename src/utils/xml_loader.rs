extern crate xml;

use std::fs::File;

use xml::reader::{EventReader, XmlEvent};
use std::io::{Read, BufReader};
use std::fs;
use std::thread::park;
use std::fmt::Error;
use core::borrow::Borrow;


pub fn load_xml(mut file_content: &mut String) {
    let mut parser = EventReader::from_str(file_content);
    parserFunc(parser);
//    parserFunc(parser.borrow());
}

fn parserFunc(parser: EventReader<&[u8]>) {
    let mut depth = 0;
    for item in parser {
        match item {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                print!("{} <{} ", indent(depth), name);
                depth += 1;

                //load attr
                if attributes.len() != 0 {
                    for item in attributes {
                        print!("{}=\"{}\" ", item.name, item.value)
                    }
                }
                println!(">");
            }
            Ok(XmlEvent::Characters(data)) => {
                println!("{} {}", indent(depth), data);
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{} </{}>", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}


#[test]
fn Test_load() {
    let filePath = "./src/example/Example_ActivityMapper.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>", filePath);
    let path = fs::read_to_string(filePath).unwrap();
    println!("Name: {}", path)
}

//load a xml file
#[test]
fn Test_load_file() {
    // --snip--
    let filePath = "./src/example/Example_ActivityMapper.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>", filePath);
    let content = fs::read_to_string(filePath).unwrap();
    println!("With text:/n{}", content);
}

//load xml
#[test]
fn Test_load_xml() {
    let filePath = "./src/example/Example_ActivityMapper.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>", filePath);
    let mut content = fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap();

    load_xml(&mut content);
}