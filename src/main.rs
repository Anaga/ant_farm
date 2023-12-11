use std::{env, process::exit};
use std::fs;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("We need more args!");        
        exit(0);
    }
    
    let file_path = &args[1];
    
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
