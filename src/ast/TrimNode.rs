use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes};
use serde_json::Value;

#[derive(Clone)]
pub struct TrimNode {
    pub childs: Vec<NodeType>,
    pub prefix: String,
    pub suffix: String,
    pub suffixOverrides: String,
    pub prefixOverrides: String,
}

impl SqlNode for TrimNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        let resultValue = DoChildNodes(&mut self.childs, env);
        let isError = resultValue.is_err();
        if isError {
            return Result::Err(resultValue.clone().err().unwrap());
        }
        let mut resultStr = resultValue.unwrap();
        let mut result = resultStr.as_str().trim();

        if !self.prefixOverrides.is_empty() {
            let splits: Vec<&str> = self.prefixOverrides.split("|").collect();
            for item in splits {
                result = result.trim_start_matches(item);
            }
        }
        if !self.suffixOverrides.is_empty() {
            let splits: Vec<&str> = self.suffixOverrides.split("|").collect();
            for item in splits {
                result = result.trim_end_matches(item);
            }
        }

        let mut newBuffer = String::new();
        newBuffer = newBuffer + " " + self.prefix.as_str() + " " + result + " " + self.suffix.as_str();
        return Result::Ok(newBuffer);
    }

    fn print(&self) -> String {
        let mut result="<trim ".to_string();
        result=result+self.prefix.as_str();
        result=result+self.suffix.as_str();
        result=result+self.suffixOverrides.as_str();
        result=result+self.prefixOverrides.as_str();
        for x in &self.childs {
            result=result+x.print().as_str();
        }
        return result;
    }
}