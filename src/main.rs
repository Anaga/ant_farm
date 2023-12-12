use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
   
    let file_path = ant_farm::parse_config(&args);
    
    println!("In file {}", file_path);
    
    if let Err(e) = ant_farm::run(file_path) {
        println!("Application error: {e}");
        process::exit(1);
    }    
}

