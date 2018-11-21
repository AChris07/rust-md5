extern crate byteorder;

use std::env;

mod md5;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    if args.len() < 2 {
        println!("Please pass a valid argument");
    }
    else {
        let input = &args[1];
        let digest = md5::compute(input);
        
        println!("{:x}", digest);
    }
}
