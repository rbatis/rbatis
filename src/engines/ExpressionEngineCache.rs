use std::collections::HashMap;

pub struct ExpressionEngineCache<'a> {
    cache: HashMap<&'a str, &'a str>,
}

impl<'a> ExpressionEngineCache<'a> {
    pub fn new() -> Self {
        return Self {
            cache: HashMap::new(),
        };
    }

    pub fn put(&mut self, k: &'a str, v: &'a str) {
        &self.cache.insert(k, v);
    }

    pub fn get(&mut self, k: &str) -> &'a str {
        return &self.cache.get(k).unwrap_or(&"null");
    }
}

#[test]
fn TestCache() {
    let mut cache = ExpressionEngineCache::new();
    let v;
    {

        cache.put("sadf", "vvv");
        v = cache.get("sadf");
    }
    println!("{}", v);
}