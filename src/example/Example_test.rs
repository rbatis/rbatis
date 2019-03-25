
struct Example{
   pub f:fn()->String
}
impl Example{
    pub fn Print(&self){
        println!("{}",self.f.call(()))
    }
}


#[test]
fn testWriteMethod(){
    let e=Example{ f: ||String::from("sad") };
   e.Print();
}