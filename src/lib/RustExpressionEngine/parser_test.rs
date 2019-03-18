use crate::lib::RustExpressionEngine::parser;
use crate::lib::RustExpressionEngine::parser::OptMap;

#[test]
fn TestParser() {
    parser::Parser();
}

#[test]
fn TestParserString() {
    let s = String::from("ab + b");
    println!("{}", s);

    //let optMap = OptMap::new();

    let mut tokens: Vec<String> = vec![];

    let mut temp = String::new();
    for c in (&s).chars() {
        if c == ' ' {
            continue;
        }
//        let op = optMap.OpsMap.get(c + tokens[tokens.len() - 1]);
//        if op.unwrap_or_else(false)==true{
//            //is opt
//            tokens[c + tokens[tokens.len() - 1]]=
//        }
        temp.push(c);

        println!("{}", temp);

    }
}