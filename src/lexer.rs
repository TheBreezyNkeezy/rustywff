#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Success tokens
    LParen,
    RParen,
    String,
    Rule,
    Delete,
    Apply,
    Quit,
    Load,
    Save,
    End,

    // Error tokens
    UnclosedParen,
    ParenOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Hash)]
pub struct Token {
    pub kind: Box<TokenKind>,
    pub text: Box<String>,
    pub loc: Box<Loc>,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.text == other.text
    }
}

impl Eq for Token {} // Automatically derived based on PartialEq

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
                        let token = Box::new(Token {
                            kind: Box::new(TokenKind::RParen),
                            text: Box::new(c.to_string()),
                            loc: loc,
                        });
                        *self.paren_layer -= 1;
                        token
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
                        ":delete" => Box::new(Token {
                            kind: Box::new(TokenKind::Delete),
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

    pub fn peek_token(&mut self) -> &Box<Token> {
        let token = self.next_token();
        self.peeked.insert(token)
    }

    pub fn next_token(&mut self) -> Box<Token> {
        self.peeked.take().unwrap_or_else(|| self.lex())
    }
}

impl Iterator for Lexer {
    type Item = Box<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.complete {
            None
        } else {
            Some(self.next_token())
        }
    }
}
