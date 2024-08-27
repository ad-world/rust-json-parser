mod parsers;

use std::{env, fs, process::exit};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    // Ensure we pass a filename
    if args.len() < 2 {
        println!("Usage: {} <name>", args[0]);
        exit(1);
    }

    // Get the filename
    let filename = &args[1];

    // Get the filecontents, or exit
    let contents = match fs::read_to_string(filename) {
        Ok(val) => val,
        Err(e) => {
            println!("Could not open file. Error: {}", e);
            exit(1);
        }
    };

    let parse_result = parsers::parse_object(&mut contents.chars().peekable());

    match parse_result {
        Ok(_) => exit(0),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
        
    }
}


