#[derive(Debug)]
pub enum Token {
    // Existing variants
    // Data types
    // Int,
    // Int8,
    // Int16,
    // Int32,
    // Int64,
    // UInt8,
    // UInt16,
    // UInt32,
    // UInt64,

    // Keywords
    Let,
    If,
    Else,
    Loop,
    Return,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,

    // Assignment
    Assign,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    // Literals
    Integer(i32),
    Float(f64),
    String(String),

    // Identifier
    Ident(String),

    // New variant for variable declaration
    Variable(String, String),

    Karya, // Keyword for function definition
    Function(String),
    ReturnType(String), // Return type
    Expretion(String),  //exprestion

    // End of File
    Eof,
}

fn get_data_type(var_type: &str) -> Option<&str> {
    match var_type {
        "int" | "int8" | "int16" | "int32" | "int64" | "uint8" | "uint16" | "uint32" | "uint64" => {
            Some(var_type)
        }
        _ => None,
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' | '\n' => {} // Ignore whitespace
            // Handle variable declaration
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                ident.push(c);
                while let Some(char) = chars.peek() {
                    if char.is_alphanumeric() || *char == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if get_data_type(&ident) != None {
                    // Parse variable declaration
                    let mut var_name = String::new();
                    chars.next();
                    while let Some(char) = chars.peek() {
                        if char.is_alphanumeric() || *char == '_' {
                            var_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    // Add Variable token
                    if var_name == "" {
                        tokens.push(Token::ReturnType(ident.to_string()));
                    } else {
                        tokens.push(Token::Variable(var_name, ident.to_string()));
                    }
                } else if ident == "karya" {
                    tokens.push(Token::Karya);
                    let mut function_name = String::new();
                    while let Some(&' ') = chars.peek() {
                        chars.next(); // Consume spaces
                    }
                    while let Some(char) = chars.peek() {
                        if char.is_alphanumeric() || *char == '_' {
                            function_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    // Add function token
                    tokens.push(Token::Function(function_name.to_string()));
                    while let Some(&' ') = chars.peek() {
                        chars.next(); // Consume spaces
                    }
                    let mut return_type = String::new();
                    while let Some(char) = chars.peek() {
                        if char.is_alphanumeric() || *char == '_' {
                            return_type.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                } else if ident == "loop" {
                    tokens.push(Token::Loop);

                } else {
                    // Check for other keywords or identifiers
                    tokens.push(match ident.as_str() {
                        "let" => Token::Let,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        _ => Token::Ident(ident),
                    });
                }
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(if let Some('=') = chars.peek() {
                chars.next(); // Consume the '=' character
                Token::NotEqual
            } else {
                Token::Minus
            }),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '=' => tokens.push(if let Some('=') = chars.peek() {
                chars.next(); // Consume the '=' character
                Token::Equal
            } else {
                Token::Assign
            }),
            '<' => tokens.push(if let Some('=') = chars.peek() {
                chars.next(); // Consume the '=' character
                Token::LessThanOrEqual
            } else {
                Token::LessThan
            }),
            '>' => tokens.push(if let Some('=') = chars.peek() {
                chars.next(); // Consume the '=' character
                Token::GreaterThanOrEqual
            } else {
                Token::GreaterThan
            }),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            ',' => tokens.push(Token::Comma),
            ';' => tokens.push(Token::Semicolon),
            '0'..='9' => {
                let mut num_str = String::new();
                num_str.push(c);
                while let Some(digit) = chars.peek() {
                    if digit.is_numeric() || *digit == '.' {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Ok(num) = num_str.parse::<i32>() {
                    tokens.push(Token::Integer(num));
                } else if let Ok(num) = num_str.parse::<f64>() {
                    tokens.push(Token::Float(num));
                } else {
                    // Handle parsing error
                    println!("Error: Failed to parse number: {}", num_str);
                }
            }
            '"' => {
                let mut string_val = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    string_val.push(c);
                }
                tokens.push(Token::String(string_val));
            }
            _ => println!("Unknown character: {}", c), // Handle unknown characters (optional)
        }
    }

    tokens.push(Token::Eof);
    tokens
}
