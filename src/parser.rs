use crate::token::Token;

pub fn parse(tokens: &[Token]) -> Result<(), String> {
    let mut index = 0; // Initialize index to keep track of the current position in the token stream

    while index < tokens.len() {
        match &tokens[index] {
            Token::Karya => {
                // Parse function definition
                index += 1; // Move to the next token
                if let Token::Function(function_name) = &tokens[index] {
                    if function_name == "" {
                        println!("Parsing function error after karya <name>()");
                        return Err("Parsing function error add <name> after karya like  karya <name>()".to_string());
                    }
                    println!("Parsing function: {:?}", function_name);

                } else {
                    println!("Parsing function error after karya <name>()");
                    return Err("Parsing function error after karya <name>()".to_string());
                }
                index += 1; // Move to the next token
                if let Token::LParen = &tokens[index] {
                    index += 1; // Move to the next token
                    // Parse parameters
                    while let Token::Variable(param_name, param_type) = &tokens[index] {
                        println!("Parameter: {} of type {}", param_name, param_type);
                        index += 1; // Move to the next token
                        if let Token::Comma = &tokens[index] {
                            index += 1; // Move past the comma
                        } else {
                            break; // No more parameters
                        }
                    }
                    if let Token::RParen = &tokens[index] {
                        index += 1; // Move to the next token
                        // Check for return type
                        if let Token::Minus = &tokens[index] {
                            index += 1; // Move past the minus sign
                            if let Token::GreaterThan = &tokens[index] {
                                index += 1; // Move past the greater than sign
                                if let Token::ReturnType(return_type) = &tokens[index] {
                                    println!("Return type: {}", return_type);
                                    index += 1; // Move to the next token
                                } else {
                                    println!("Expected return type");
                                    return Err("Expected return type".to_string());
                                }
                            } else {
                                return Err("Expected '->' for return type".to_string());
                            }
                        }
                    } else {
                        return Err("Expected ')' after function parameters".to_string());
                    }
                } else {
                    return Err("Expected '(' after function name".to_string());
                }
            }
            Token::If => {
                // Parse if statement
                println!("Parsing if statement");
                // You can implement the parsing logic for if statements here
                index += 1; // Move to the next token
            }
            Token::Return => {
                // Parse return statement
                println!("Parsing return statement");
                // You can implement the parsing logic for return statements here
                index += 1; // Move to the next token
            }
            _ => {
                // Skip other tokens for now
                index += 1; // Move to the next token
            }
        }
    }

    Ok(())
}

