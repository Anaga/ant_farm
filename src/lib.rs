use std::error::Error;
use std::fs;

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    println!("With text:\n{contents}");
    Ok(())
}

pub fn parse_config(args: &[String]) -> &str {
    if args.len() < 2 {
        println!("We need more args."); 
        panic!("not enough arguments");            
    }
    let file_path = &args[1];
    file_path
}
