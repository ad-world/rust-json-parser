use std::{iter::Peekable, str::Chars};

pub fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        // While you can consume whitespace, consume it
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

pub fn parse_object(chars: &mut Peekable<Chars>) -> Result<bool, String> {
    if let Some('{') = chars.peek() {
        chars.next(); // Consume the opening brace
        skip_whitespace(chars); // Consume any whitespace
    } else {
        return Err("Invalid JSON".to_string());
    }
 

    if let Some('}') = chars.peek() {
        chars.next(); // Consume the closing brace
        return Ok(true); // Empty object
    }

    while let Some(_) = chars.peek() {
        // First thing we see should be a key, which is always a string. Try parse a string
        if !parse_string(chars) {
            return Err("Invalid JSON: failed to parse key".to_string());
        }

        // After successfully parsing a string, skip all whitespcae
        skip_whitespace(chars);

        // Now, we should see a colon
        if let Some(':') = chars.next() {
            // Skip all whitespace after seeing colon
            skip_whitespace(chars);
            // Now, we should see a value
            if !parse_value(chars) { 
                return Err("Invalid JSON: failed to parse value".to_string());
            }
        } else {
            // Didn't see a colon, invalid JSON
            return Err("Invalid JSON: no colon after key".to_string());
        }

        // Skip all whitespace
        skip_whitespace(chars);

        match chars.next() {
            // If we see a comma, skip all whitespace and continue parsing
            Some(',') => skip_whitespace(chars),
            // If we see a closing brace, we're done
            Some('}') => return Ok(true),
            // If we see anything else, it's invalid JSON
            _ => {
                return Err("Invalid JSON: expected comma or closing brace".to_string());
            }
        }
    }

    return Err("Invalid JSON".to_string());
}

pub fn parse_string(chars: &mut Peekable<Chars>) -> bool {
    if let Some('"') = chars.peek() {
        chars.next();
    } else {
        return false;
    }
    while let Some(c) = chars.peek() {
        match c {
            '"' => {
                chars.next();
                return true;
            }
            _ => {
                chars.next();
            }
        };
    }

    false
}

fn parse_number(chars: &mut Peekable<Chars>) -> bool {
    // This should able to parse decimal
    // Consume leading minus sign if present
    if let Some('-') = chars.peek() {
        chars.next();
    }

    // Consume digits
    while let Some(c) = chars.peek() {
       if c.is_ascii_digit() {
           chars.next();
       } else {
           break;
       }
    }

    return true;
}

fn parse_literal(chars: &mut Peekable<Chars>, literal: &str) -> bool { 
    for char in literal.chars() {
        if let Some(c) = chars.peek() {
            if c.eq(&char) {
                chars.next();
            } else {
                return false;
            }
        } 
    }

    return true;
}

fn parse_array(chars: &mut Peekable<Chars>) -> bool {
    if let Some('[') = chars.peek() {
        chars.next();
    } else {
        return false;
    }

    skip_whitespace(chars);

    if let Some(']') = chars.peek() {
        chars.next();
        return true;
    }

    while let Some(_) = chars.peek() {
        if !parse_value(chars) {
            return false;
        }

        skip_whitespace(chars);

        match chars.next() {
            Some(',') => skip_whitespace(chars),
            Some(']') => return true,
            _ => return false,
        }
    }

    return false;
}

pub fn parse_value(chars: &mut Peekable<Chars>) -> bool {
    skip_whitespace(chars);
    match chars.peek() {
        Some('{') => match parse_object(chars) {
            Ok(_) => return true,
            Err(_) => return false,
        },
        Some('[') => parse_array(chars),
        Some('"') => parse_string(chars),
        Some('-') | Some('0'..='9') => parse_number(chars),
        Some('t') => parse_literal(chars, "true"),
        Some('f') => parse_literal(chars, "false"),
        Some('n') => parse_literal(chars, "null"),
        _ => false,
    }
}