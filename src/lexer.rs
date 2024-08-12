#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Symbol(Box<str>),
    Rule,           // Token for :rule command
    Apply,          // Token for :apply command
    Name(Box<str>), // Token for rule names or other names
}

fn extract_cons(input: &str) -> Option<(&str, &str)> {
    let input = input.trim(); // Trim any surrounding whitespace

    if input.starts_with('(') && input.ends_with(')') {
        let inner_content = &input[1..input.len() - 1]; // Extract the content inside the parentheses
        Some(("()", inner_content))
    } else {
        None // Return None if the input doesn't match the expected pattern
    }
}

fn extract_units(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current_symbol = String::new();

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                if !current_symbol.is_empty() {
                    tokens.push(Token::Symbol(current_symbol.clone().into_boxed_str()));
                    current_symbol.clear();
                }
                tokens.push(Token::LParen);
            }
            ')' => {
                if !current_symbol.is_empty() {
                    tokens.push(Token::Symbol(current_symbol.clone().into_boxed_str()));
                    current_symbol.clear();
                }
                tokens.push(Token::RParen);
            }
            ' ' => {
                if !current_symbol.is_empty() {
                    tokens.push(Token::Symbol(current_symbol.clone().into_boxed_str()));
                    current_symbol.clear();
                }
            }
            _ if c.is_alphanumeric() || c == '-' => {
                current_symbol.push(c);
            }
            _ => {}
        }
    }

    // Add any remaining symbol at the end of the input
    if !current_symbol.is_empty() {
        tokens.push(Token::Symbol(current_symbol.into_boxed_str()));
    }

    tokens
}

fn extract_command(input: &str) -> (Vec<Token>, Vec<String>) {
    let mut tokens = Vec::new();
    let mut sub_expressions = Vec::new();
    let mut chars = input.chars().peekable();
    let mut depth = 0;
    let mut current_expr = String::new();
    let mut current_name = String::new();
    let mut found_name = false;

    // First, parse the command (e.g., ":rule" or ":apply") and the name.
    while let Some(c) = chars.next() {
        match c {
            ':' => {
                let command: String = chars.by_ref().take_while(|&c| !c.is_whitespace()).collect();
                match command.as_str() {
                    "rule" => tokens.push(Token::Rule),
                    "apply" => tokens.push(Token::Apply),
                    _ => println!("Unknown command: :{}", command),
                }
            }
            _ if !found_name && c.is_alphanumeric() || c == '-' => {
                current_name.push(c);
                current_name.extend(
                    chars
                        .by_ref()
                        .take_while(|c| c.is_alphanumeric() || *c == '-'),
                );
                tokens.push(Token::Name(current_name.clone().into_boxed_str()));
                found_name = true;
            }
            '(' => {
                if depth == 0 {
                    current_expr.clear(); // Start a new S-expression
                }
                current_expr.push(c);
                depth += 1;
            }
            ')' => {
                current_expr.push(c);
                depth -= 1;
                if depth == 0 {
                    sub_expressions.push(current_expr.clone()); // Complete S-expression
                }
            }
            _ => {
                if depth > 0 {
                    current_expr.push(c);
                }
            }
        }
    }

    (tokens, sub_expressions)
}

pub fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    match extract_cons(input) {
        Some((_, content)) => {
            tokens.push(Token::LParen);
            tokens.extend(extract_units(content));
            tokens.push(Token::RParen);
        }
        None => match input {
            input if input.split_whitespace().count() == 1 => {
                tokens.push(Token::Symbol(input.into()));
            }
            _ => {
                let (command_group, sub_expressions) = extract_command(input);
                tokens.extend(command_group);
                for sub_expression in sub_expressions {
                    tokens.extend(lexer(&sub_expression));
                }
            }
        },
    }
    tokens
}
