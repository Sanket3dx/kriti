use crate::token::Token;

pub fn parse(tokens: &[Token]) -> Result<(), String> {
    let mut index = 0;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Karya => {
                // Parse function definition
                index += 1;
                match &tokens.get(index) {
                    Some(Token::Function(function_name)) => {
                        if function_name == "" {
                            return Err("Function name is missing after 'karya'".to_string());
                        }
                        println!("Parsing function: {}", function_name);
                    }
                    _ => return Err("Expected function name after 'karya'".to_string()),
                }

                index += 1; // Move to the next token
                match &tokens.get(index) {
                    Some(Token::LParen) => {
                        index += 1; // Move to the next token
                                    // Parse parameters
                        while let Some(Token::Variable(param_name, param_type)) = tokens.get(index)
                        {
                            println!("Parameter: {} of type {}", param_name, param_type);
                            index += 1; 
                            match &tokens.get(index) {
                                Some(Token::Comma) => index += 1, // Move past the comma
                                _ => break,                       // No more parameters
                            }
                        }

                        match &tokens.get(index) {
                            Some(Token::RParen) => {
                                index += 1; 
                                            // Check for return type
                                if let Some(Token::Minus) = tokens.get(index) {
                                    index += 1; // Move past the minus sign
                                    if let Some(Token::GreaterThan) = tokens.get(index) {
                                        index += 1; // Move past the greater than sign
                                        if let Some(Token::ReturnType(return_type)) =
                                            tokens.get(index)
                                        {
                                            println!("Return type: {}", return_type);
                                            index += 1; 
                                        } else {
                                            return Err("Expected return type".to_string());
                                        }
                                    } else {
                                        return Err("Expected '->' for return type".to_string());
                                    }
                                }
                            }
                            _ => return Err("Expected ')' after function parameters".to_string()),
                        }
                    }
                    _ => return Err("Expected '(' after function name".to_string()),
                }

                // Parse function body
                if let Some(Token::LBrace) = tokens.get(index) {
                    index += 1; 
                    while let Some(token) = tokens.get(index) {
                        match token {
                            Token::If => {
                                // Parse if statement
                                parse_if_statement(tokens, index)?;
                                // Update index after parsing if statement
                                while let Some(token) = tokens.get(index) {
                                    match token {
                                        Token::RBrace => {
                                            index += 1; // Move past the closing brace
                                            break; // Exit the function body
                                        }
                                        _ => index += 1, 
                                    }
                                }
                            }
                            Token::Loop => {
                                // Parse loop statement
                                index += 1; 
                                            // You can implement parsing of loop statement here
                            }
                            Token::Match => {
                                // Parse match expression
                                index += 1; 
                            }
                            Token::Ident(_) => {
                                // Parse assignment or function call
                                index += 1; 
                            }
                            Token::Return => {
                                // Parse return statement
                                index += 1;
                            }
                            Token::RBrace => {
                                index += 1;
                                break; // Exit the function body
                            }
                            _ => {
                                // Skip other tokens for now
                                index += 1;
                            }
                        }
                    }
                } else {
                    return Err("Expected '{' to start function body".to_string());
                }
            }
            _ => {
                // Skip other tokens for now
                index += 1;
            }
        }
    }

    Ok(())
}

pub fn parse_if_statement(tokens: &[Token], mut index: usize) -> Result<(), String> {
    if let Some(Token::If) = tokens.get(index) {
        index += 1;
    } else {
        return Err("Expected 'if' keyword".to_string());
    }

    match tokens.get(index) {
        Some(Token::Expression(expr_tokens)) => {
            // Parse the condition expression
            println!("Condition: {:?}", expr_tokens);
            index += 1;
        }
        Some(Token::Boolean(condition)) => {
            // Parse the condition expression
            println!("Condition: {:?}", condition);
            index += 1;
        }
        _ => return Err("Expected Expression after 'if' keyword".to_string()),
    }

    // Check for the opening brace
    match tokens.get(index) {
        Some(Token::LBrace) => index += 1, // Move past the left brace
        _ => return Err("Expected '{' after condition expression".to_string()),
    }

    // Skip tokens inside the 'if' block until reaching the closing brace
    let mut brace_count = 1;
    let mut brace_stack = vec![Token::LBrace];
    let mut if_tokens = Vec::new();
    while let Some(token) = tokens.get(index) {
        match token {
            Token::RBrace => {
                brace_count -= 1;
                brace_stack.pop(); // Pop the matching left brace
                if brace_count == 0 {
                    // Exit loop if closing brace found
                    index += 1; // Move past the closing brace
                    break;
                }
            }
            Token::LBrace => {
                brace_count += 1;
                brace_stack.push(Token::LBrace); // Push nested left brace
            }
            Token::If if brace_stack.len() == 1 => {
                // If there's a nested 'if', parse it recursively
                parse_if_statement(tokens, index)?;
                // Adjust the index to skip the tokens inside the nested 'if' block
                while let Some(token) = tokens.get(index) {
                    match token {
                        Token::RBrace => {
                            brace_count -= 1;
                            brace_stack.pop(); // Pop the matching left brace
                            if brace_count == 0 {
                                break; // Exit loop if closing brace found
                            }
                        }
                        Token::LBrace => {
                            brace_count += 1;
                            brace_stack.push(Token::LBrace); // Push nested left brace
                        }
                        _ => (),
                    }
                    index += 1;
                }
                continue; // Continue to the next token after the nested 'if' block
            }
            _ => {
                if_tokens.push(token.clone()); // Collect tokens inside the 'if' block
            }
        }
        index += 1;
    }

    // Now parse the collected tokens inside the 'if' block
    parse(&if_tokens)?;

    Ok(())
}
