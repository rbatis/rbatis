use rdbc::Connection;

pub struct Tx{
    pub coon:Box<dyn Connection>,
    pub is_start:bool,
    pub is_close:bool,
}

impl Tx{

}





#[test]
fn test_tx(){
    
}