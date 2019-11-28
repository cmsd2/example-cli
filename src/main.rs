extern crate clap; 
use clap::App; 

fn main() { 
    App::new("example-cli")
       .version("1.0")
       .about("Does great things!")
       .get_matches();
    
    println!("hello, world!");
}