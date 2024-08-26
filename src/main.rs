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

    let result = json_lexer(contents);
    
    match result {
        Ok(val) => {
           exit(0);
        },
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}

fn json_lexer(json: String) -> Result<bool, String> {
    let mut tokens = Vec::<char>::new();

    for c in json.chars() {
        if c.is_whitespace() {
            continue;
        }

        tokens.push(c);
    }

    if tokens.iter().collect::<String>() == "{}".to_string() {
        return Ok(true);
    }

    return Err("Invalid JSON".to_string());
}

