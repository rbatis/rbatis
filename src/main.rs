mod utils;

use utils::TimeUtil;
use chrono::Local;

fn main(){
    TimeUtil::Count_TPS(1,Local::now());

    println!("{}",1);

}