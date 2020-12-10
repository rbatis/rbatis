use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TokenMap<'a> {
    pub all_token: HashMap<&'a str, bool>,
    pub group_token_map: HashMap<&'a str, bool>,
    pub single_token_map: HashMap<&'a str, bool>,
    //sorted
    pub allow_calculate_token: Vec<&'a str>,
}

impl<'a> TokenMap<'a> {
    pub fn new() -> Self {
        let mut all = HashMap::new();
        let mut mul_ops_map = HashMap::new();
        let mut single_token_map = HashMap::new();

        //all token
        let list = vec![
            "(", ")",
            "%", "^", "*", "**", "/", "+", "-",
            "@", "#", "$", "=", "!", ">", "<", "&", "|",
            "==", "!=", ">=", "<=", "&&", "||"
        ];

        //all token map
        for item in &list {
            all.insert(item.to_owned(), true);
        }
        //single token and mul token
        for item in &list {
            if item.len() > 1 {
                mul_ops_map.insert(item.to_owned(), true);
            } else {
                single_token_map.insert(item.to_owned(), true);
            }
        }

        Self {
            all_token: all,
            group_token_map: mul_ops_map,
            single_token_map: single_token_map,
            allow_calculate_token: vec!["%", "^", "*", "**", "/", "+", "-", "<=", "<", ">=", ">", "!=", "==", "&&", "||"],
        }
    }

    ///The or operation in the nonoperational > arithmetic operator > relational operator > logical operator and operation > logical operator
    pub fn priority_array(&self) -> &Vec<&str> {
        return &self.allow_calculate_token;
    }

    pub fn is_token(&self, arg: &str) -> bool {
        let token = self.all_token.get(arg);
        return token.is_none() == false;
    }

    pub fn is_allow_token(&self, arg: &str) -> bool {
        for item in &self.allow_calculate_token {
            if arg == *item {
                return true;
            }
        }
        return false;
    }
}