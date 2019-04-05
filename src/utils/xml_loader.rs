extern crate xml;

use std::fs::File;

use xml::reader::{EventReader, XmlEvent};
use std::io::{Read, BufReader};
use std::fs;
use std::thread::park;
use std::fmt::Error;
use core::borrow::Borrow;
use self::xml::attribute::OwnedAttribute;

#[derive(Clone, Debug)]
pub struct Element {
    pub tag: String,
    pub data: String,
    pub attributes: Vec<OwnedAttribute>,
    pub childs: Vec<Element>,
}

impl Element {
    pub fn reset(&mut self) {
        self.tag.clear();
        self.data.clear();
        self.attributes.clear();
        self.childs.clear();
    }
}


pub fn load_xml(mut file_content: &mut String) -> Vec<Element> {
    let mut parser = EventReader::from_str(file_content);
    return parserFunc(parser);
}

fn parserFunc(parser: EventReader<&[u8]>) -> Vec<Element> {
    let mut depth = 0;

    let mut tempElement = &mut Element {
        tag: "".to_string(),
        data: "".to_string(),
        attributes: vec![],
        childs: vec![],
    };

    let mut fathers = vec![];

    for item in parser {
        match item {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                //load attr
                tempElement.tag = name.local_name;
                tempElement.attributes = attributes.clone();

                &fathers.push(tempElement.clone());
                depth += 1;
            }
            Ok(XmlEvent::Characters(data)) => {
                let last = fathers.last_mut().unwrap();
                (*last).childs.push(Element {
                    tag: "".to_string(),
                    data: data.clone(),
                    attributes: vec![],
                    childs: vec![],
                })
            }
            Ok(XmlEvent::EndElement { name }) => {
                let pop = fathers.pop().unwrap();
                let last = fathers.last_mut();
                if last.is_some() {
                    last.unwrap().childs.push(pop);
                } else {
                    fathers.push(pop)
                }
                tempElement.reset();

                depth -= 1;
            }
            Err(e) => {
                println!("Error: {},{}", e, tempElement.tag);
                break;
            }
            _ => {}
        }
    }

    println!("result>>>>>>>  {}", fathers.len());
    return fathers;
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
    println!("With text:/n{}", content);

    load_xml(&mut content);
}