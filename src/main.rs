use std::env;
use std::fs;
use std::error::Error;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
   
    let file_path = parse_config(&args);
    
    println!("In file {}", file_path);
    
    if let Err(e) = run(file_path) {
        println!("Application error: {e}");
        process::exit(1);
    }    
}

fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    println!("With text:\n{contents}");
    Ok(())
}

fn parse_config(args: &[String]) -> &str {
    if args.len() < 2 {
        println!("We need more args!"); 
        panic!("not enough arguments");            
    }
    let file_path = &args[1];
    file_path
}
