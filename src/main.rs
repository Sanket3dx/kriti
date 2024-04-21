mod token;
mod parser;
use std::fs;

fn main() {
    // Read the filename from the command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Attempt to read the contents of the file
    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let tokens = token::tokenize(&code);
    println!("Tokens: {:?}", tokens);

    let err = parser::parse(&tokens); // Basic parsing call
    println!("Tokens: {:?}", err);

}
