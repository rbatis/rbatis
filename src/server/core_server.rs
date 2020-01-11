use std::thread;

#[test]
pub fn test_future(){
  //  main();
    let thread_id=thread::current().id();
    println!("{:?}",thread_id);
}