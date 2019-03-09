extern crate xml;

use std::fs::File;

use xml::reader::{EventReader, XmlEvent};
use std::io::{Read, BufReader};
use std::fs;


pub fn LoadXml(fileContent: String) {
//    let file = File::open(filePath).unwrap();
//    let file = BufReader::new(file);
    let parser = EventReader::from_str(fileContent.as_ref());
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes,.. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;

                //load attr
                if attributes.len()!=0 {
                    for item in attributes{
                        println!("attr>>>  key=\"{}\",value=\"{}\"",item.value,item.value)
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Ok(XmlEvent::Characters(data)) => {
                println!("{}-{}", indent(depth), data);
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
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>",filePath);
    let path = fs::read_to_string(filePath).unwrap();
    println!("Name: {}", path)
}

//load a xml file
#[test]
fn Test_load_file() {
    // --snip--
    let filePath = "./src/example/Example_ActivityMapper.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>",filePath);
    let content = fs::read_to_string(filePath).unwrap();
    println!("With text:/n{}", content);
}

//load xml
#[test]
fn Test_load_xml() {
    let filePath="./src/example/Example_ActivityMapper.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>",filePath);
    let content = fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap();
    LoadXml(content);
}