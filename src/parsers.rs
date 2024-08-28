use std::{iter::Peekable, str::Chars};

fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        // While you can consume whitespace, consume it
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn parse_object(chars: &mut Peekable<Chars>) -> Result<bool, String> {
    if let Some('{') = chars.peek() {
        chars.next(); // Consume the opening brace
        skip_whitespace(chars); // Consume any whitespace
    } else {
        return Err("Invalid JSON: expected opening brace".to_string());
    }
 

    if let Some('}') = chars.peek() {
        chars.next(); // Consume the closing brace
        return Ok(true); // Empty object
    }

    while let Some(_) = chars.peek() {
        // First thing we see should be a key, which is always a string. Try parse a string
        if parse_string(chars).is_err() {
            return Err("Invalid JSON: failed to parse key".to_string());
        }

        // After successfully parsing a string, skip all whitespcae
        skip_whitespace(chars);

        // Now, we should see a colon
        if let Some(':') = chars.next() {
            // Skip all whitespace after seeing colon
            skip_whitespace(chars);
            // Now, we should see a value
            if parse_value(chars).is_err() { 
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

fn parse_string(chars: &mut Peekable<Chars>) -> Result<bool, String> {
    if let Some('"') = chars.peek() {
        chars.next();
    } else {
        return Err("Invalid JSON: Expected opening quote".to_string());
    }
    while let Some(c) = chars.peek() {
        match c {
            '"' => {
                chars.next();
                return Ok(true);
            }
            _ => {
                chars.next();
            }
        };
    }

    return Err("Invalid JSON: expected string to close".to_string());
}

fn parse_number(chars: &mut Peekable<Chars>) -> Result<bool, String> {
    // This should able to parse decimal
    // Consume leading minus sign if present
    let mut is_decimal = false;

    if let Some('-') = chars.peek() {
        chars.next();
    }

    // Consume digits
    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            chars.next();
        } else if c == &'.' {
            if is_decimal {
                return Err("Invalid JSON: failed to parse number".to_string());
            } else {
                is_decimal = true;
                chars.next();
            }
        } else {
            break;
       }
    }

    return Ok(true);
}

fn parse_literal(chars: &mut Peekable<Chars>, literal: &str) -> Result<bool, String> { 
    let mut count = 0;
    for char in literal.chars() {
        if let Some(c) = chars.peek() {
            if c.eq(&char) {
                chars.next();
                count += 1;
            } else {
                return Err("Invalid JSON: failed to parse literal".to_string());
            }
        } 
    }

    // error if the entire literal is not consumed
    if count != literal.len() {
        return Err("Invalid JSON: failed to parse literal".to_string());
    }

    return Ok(true);
}

fn parse_array(chars: &mut Peekable<Chars>) -> Result<bool, String> {
    if let Some('[') = chars.peek() {
        chars.next();
    } else {
        return Err("Invalid JSON: Expected opening bracket".to_string());
    }

    skip_whitespace(chars);

    if let Some(']') = chars.peek() {
        chars.next();
        return Ok(true);
    }

    while let Some(_) = chars.peek() {
        if parse_value(chars).is_err() {
            return Err("Invalid JSON: failed to parse array value".to_string());
        }

        skip_whitespace(chars);

        match chars.next() {
            Some(',') => skip_whitespace(chars),
            Some(']') => return Ok(true),
            _ => return Err("Invalid JSON: expected comma or closing bracket after value".to_string())
        }
    }

    return Err("Invalid JSON: unexpected end of array".to_string());
}

fn parse_value(chars: &mut Peekable<Chars>) -> Result<bool, String> {
    skip_whitespace(chars);
    match chars.peek() {
        Some('{') => parse_object(chars),
        Some('[') => parse_array(chars),
        Some('"') => parse_string(chars),
        Some('-') | Some('0'..='9') => parse_number(chars),
        Some('t') => parse_literal(chars, "true"),
        Some('f') => parse_literal(chars, "false"),
        Some('n') => parse_literal(chars, "null"),
        _ => Err("Invalid JSON: unknown value".to_string()),
    }
}

pub fn parse(json: &str) -> Result<bool, String> {
    let mut chars = json.chars().peekable();
    parse_value(&mut chars)
}