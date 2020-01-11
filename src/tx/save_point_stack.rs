use std::collections::LinkedList;

pub struct SavePointStack{
    pub len:usize,
    pub data:LinkedList<String>,
}

impl SavePointStack{
    pub fn new()->Self{
        return Self{
            len: 0,
            data: LinkedList::new()
        }
    }
    pub fn push(&mut self,k:&str){
        self.len+=1;
        self.data.push_back(k.to_string());
    }
    pub  fn pop(&mut self,k:&str)->Option<String>{
        if self.len==0{
            return None;
        }
        self.len -= 1;
        return self.data.pop_back();
    }
}