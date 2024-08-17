#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Success tokens
    LParen,
    RParen,
    String,
    Rule,
    Apply,
    Quit,
    Load,
    Save,
    End,

    // Error tokens
    UnclosedParen,
    ParenOverflow,
}

#[derive(Debug, Clone)]
pub struct ReplLoc {
    pub col: Box<usize>,
}

#[derive(Debug, Clone)]
pub enum Loc {
    FileLoc {
        path: Box<String>,
        row: Box<usize>,
        col: Box<usize>,
    },
    ReplLoc {
        col: Box<usize>,
    },
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Box<TokenKind>,
    pub text: Box<String>,
    pub loc: Box<Loc>,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    input: Box<Vec<char>>,
    peeked: Option<Box<Token>>,
    pub complete: bool,
    file_path: Option<Box<String>>,
    row_number: Box<usize>,
    line_beginning: Box<usize>,
    line_current: Box<usize>,
    paren_layer: Box<usize>,
}

impl Lexer {
    pub fn new(input: &str, file_path: Option<&str>) -> Lexer {
        Lexer {
            input: Box::new(input.chars().collect()),
            peeked: None,
            complete: false,
            file_path: file_path.map(|s| Box::new(s.to_string())),
            row_number: Box::new(0),
            line_beginning: Box::new(0),
            line_current: Box::new(0),
            paren_layer: Box::new(0),
        }
    }

    pub fn loc(&self) -> Box<Loc> {
        let loc = match &self.file_path {
            Some(file_path) => Loc::FileLoc {
                path: file_path.clone(),
                row: Box::new(*self.row_number + 1),
                col: Box::new(*self.line_current - *self.line_beginning),
            },
            None => Loc::ReplLoc {
                col: Box::new(*self.line_current - *self.line_beginning),
            },
        };
        Box::new(loc)
    }

    pub fn drop_char(&mut self) -> Option<char> {
        if self.input.is_empty() {
            None
        } else {
            let c = self.input.remove(0);
            if c == '\n' {
                *self.row_number += 1;
                *self.line_beginning = *self.line_current;
            }
            *self.line_current += 1;
            Some(c)
        }
    }

    pub fn lex_whitespaces(&mut self) {
        while let Some(c) = self.input.get(0) {
            if c.is_whitespace() {
                self.drop_char();
            } else {
                break;
            }
        }
    }

    pub fn lex(&mut self) -> Box<Token> {
        self.lex_whitespaces(); // Skip any leading whitespace

        let loc = self.loc();
        match self.drop_char() {
            Some(c) => match c {
                '(' => {
                    *self.paren_layer += 1;
                    Box::new(Token {
                        kind: Box::new(TokenKind::LParen),
                        text: Box::new(c.to_string()),
                        loc: loc,
                    })
                }
                ')' => {
                    if *self.paren_layer == 0 {
                        Box::new(Token {
                            kind: Box::new(TokenKind::ParenOverflow),
                            text: Box::new(")".to_string()),
                            loc: loc,
                        })
                    } else {
                        *self.paren_layer -= 1;
                        Box::new(Token {
                            kind: Box::new(TokenKind::RParen),
                            text: Box::new(c.to_string()),
                            loc: loc,
                        })
                    }
                }
                ':' => {
                    let mut text = c.to_string();
                    while let Some(&next_char) = self.input.get(0) {
                        if next_char.is_whitespace() || next_char == '(' || next_char == ')' {
                            break;
                        }
                        text.push(self.drop_char().unwrap());
                    }

                    match text.as_str() {
                        ":rule" => Box::new(Token {
                            kind: Box::new(TokenKind::Rule),
                            text: Box::new(text),
                            loc: loc,
                        }),
                        ":apply" => Box::new(Token {
                            kind: Box::new(TokenKind::Apply),
                            text: Box::new(text),
                            loc: loc,
                        }),
                        ":quit" => Box::new(Token {
                            kind: Box::new(TokenKind::Quit),
                            text: Box::new(text),
                            loc: loc,
                        }),
                        ":load" => Box::new(Token {
                            kind: Box::new(TokenKind::Load),
                            text: Box::new(text),
                            loc: loc,
                        }),
                        ":save" => Box::new(Token {
                            kind: Box::new(TokenKind::Save),
                            text: Box::new(text),
                            loc: loc,
                        }),
                        _ => Box::new(Token {
                            kind: Box::new(TokenKind::String),
                            text: Box::new(text),
                            loc: loc,
                        }),
                    }
                }
                _ => {
                    // Handle symbols: accumulate characters until we hit a non-symbol character
                    let mut text = c.to_string();
                    while let Some(&next_char) = self.input.get(0) {
                        if next_char.is_whitespace() || next_char == '(' || next_char == ')' {
                            break;
                        }
                        text.push(self.drop_char().unwrap());
                    }

                    Box::new(Token {
                        kind: Box::new(TokenKind::String),
                        text: Box::new(text),
                        loc: loc,
                    })
                }
            },
            None => {
                self.complete = true;
                if *self.paren_layer == 0 {
                    Box::new(Token {
                        kind: Box::new(TokenKind::End),
                        text: Box::new("".to_string()),
                        loc: loc,
                    })
                } else {
                    Box::new(Token {
                        kind: Box::new(TokenKind::UnclosedParen),
                        text: Box::new("".to_string()),
                        loc: loc,
                    })
                }
            }
        }
    }

    pub fn peek(&mut self) -> &Box<Token> {
        let token = self.next();
        self.peeked.insert(token)
    }

    pub fn next(&mut self) -> Box<Token> {
        self.peeked.take().unwrap_or_else(|| self.lex())
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum Token {
//     LParen,
//     RParen,
//     String(Box<str>),
//     Rule,           // Token for :rule command
//     Apply,          // Token for :apply command
//     Name(Box<str>), // Token for rule names or other names
// }

// fn extract_cons(input: &str) -> Option<(&str, &str)> {
//     let input = input.trim(); // Trim any surrounding whitespace

//     if input.starts_with('(') && input.ends_with(')') {
//         let inner_content = &input[1..input.len() - 1]; // Extract the content inside the parentheses
//         Some(("()", inner_content))
//     } else {
//         None // Return None if the input doesn't match the expected pattern
//     }
// }

// fn extract_units(input: &str) -> Vec<Token> {
//     let mut tokens = Vec::new();
//     let mut chars = input.chars().peekable();
//     let mut current_symbol = String::new();

//     while let Some(c) = chars.next() {
//         match c {
//             '(' => {
//                 if !current_symbol.is_empty() {
//                     tokens.push(Token::String(current_symbol.clone().into_boxed_str()));
//                     current_symbol.clear();
//                 }
//                 tokens.push(Token::LParen);
//             }
//             ')' => {
//                 if !current_symbol.is_empty() {
//                     tokens.push(Token::String(current_symbol.clone().into_boxed_str()));
//                     current_symbol.clear();
//                 }
//                 tokens.push(Token::RParen);
//             }
//             ' ' => {
//                 if !current_symbol.is_empty() {
//                     tokens.push(Token::String(current_symbol.clone().into_boxed_str()));
//                     current_symbol.clear();
//                 }
//             }
//             _ if c.is_alphanumeric() || c == '-' => {
//                 current_symbol.push(c);
//             }
//             _ => {}
//         }
//     }

//     // Add any remaining symbol at the end of the input
//     if !current_symbol.is_empty() {
//         tokens.push(Token::String(current_symbol.into_boxed_str()));
//     }

//     tokens
// }

// fn extract_command(input: &str) -> (Vec<Token>, Vec<String>) {
//     let mut tokens = Vec::new();
//     let mut sub_expressions = Vec::new();
//     let mut chars = input.chars().peekable();
//     let mut depth = 0;
//     let mut current_expr = String::new();
//     let mut current_name = String::new();
//     let mut found_name = false;

//     // First, parse the command (e.g., ":rule" or ":apply") and the name.
//     while let Some(c) = chars.next() {
//         match c {
//             ':' => {
//                 let command: String = chars.by_ref().take_while(|&c| !c.is_whitespace()).collect();
//                 match command.as_str() {
//                     "rule" => tokens.push(Token::Rule),
//                     "apply" => tokens.push(Token::Apply),
//                     _ => println!("Unknown command: :{}", command),
//                 }
//             }
//             _ if !found_name && c.is_alphanumeric() || c == '-' => {
//                 current_name.push(c);
//                 current_name.extend(
//                     chars
//                         .by_ref()
//                         .take_while(|c| c.is_alphanumeric() || *c == '-'),
//                 );
//                 tokens.push(Token::Name(current_name.clone().into_boxed_str()));
//                 found_name = true;
//             }
//             '(' => {
//                 if depth == 0 {
//                     current_expr.clear(); // Start a new S-expression
//                 }
//                 current_expr.push(c);
//                 depth += 1;
//             }
//             ')' => {
//                 current_expr.push(c);
//                 depth -= 1;
//                 if depth == 0 {
//                     sub_expressions.push(current_expr.clone()); // Complete S-expression
//                 }
//             }
//             _ => {
//                 if depth > 0 {
//                     current_expr.push(c);
//                 }
//             }
//         }
//     }

//     (tokens, sub_expressions)
// }

// pub fn lexer(input: &str) -> Vec<Token> {
//     let mut tokens = Vec::new();

//     match extract_cons(input) {
//         Some((_, content)) => {
//             tokens.push(Token::LParen);
//             tokens.extend(extract_units(content));
//             tokens.push(Token::RParen);
//         }
//         None => match input {
//             input if input.split_whitespace().count() == 1 => {
//                 tokens.push(Token::String(input.into()));
//             }
//             _ => {
//                 let (command_group, sub_expressions) = extract_command(input);
//                 tokens.extend(command_group);
//                 for sub_expression in sub_expressions {
//                     tokens.extend(lexer(&sub_expression));
//                 }
//             }
//         },
//     }
//     println!("{:?}", tokens);
//     tokens
// }
