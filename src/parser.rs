use crate::lexer::*;

#[derive(Debug, Clone)]
pub enum Command {
    QuitRepl,
    DefineRule {
        name: Box<String>,
        lhs: Box<LogExpr>,
        rhs: Box<LogExpr>,
    },
    ApplyRule {
        name: Box<String>,
        expr: Box<LogExpr>,
    },
    LoadFile {
        file_path: Box<String>,
    },
    SaveFile {
        file_path: Box<String>,
    },
    Eval {
        expr: Box<LogExpr>,
    },
}

impl Command {
    pub fn parse(lexer: &mut Lexer) -> Option<Box<Command>> {
        let token = lexer.peek_token();
        match *token.kind {
            TokenKind::Quit => {
                lexer.next();
                Some(Box::new(Command::QuitRepl))
            }
            TokenKind::Load => {
                lexer.next();
                let file_path = lexer.next().expect("Expected a file path").text.clone();
                Some(Box::new(Command::LoadFile { file_path }))
            }
            TokenKind::Save => {
                lexer.next();
                let file_path = lexer.next().expect("Expected a file path").text.clone();
                Some(Box::new(Command::SaveFile { file_path }))
            }
            TokenKind::Rule => {
                lexer.next();
                let name = lexer.next().expect("Expected a rule name").text.clone();
                let lhs = LogExpr::parse(lexer).expect("Expected a left-hand side expression");
                let rhs = LogExpr::parse(lexer).expect("Expected a right-hand side expression");
                Some(Box::new(Command::DefineRule { name, lhs, rhs }))
            }
            TokenKind::Apply => {
                lexer.next();
                let name = lexer.next().expect("Expected a rule name").text.clone();
                let expr =
                    LogExpr::parse(lexer).expect("Expected an expression to apply the rule to");
                Some(Box::new(Command::ApplyRule { name, expr }))
            }
            TokenKind::End => {
                lexer.next();
                None
            }
            _ => {
                let expr = LogExpr::parse(lexer)?;
                Some(Box::new(Command::Eval { expr }))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Not { paren_layer: Box<usize> },
    And { paren_layer: Box<usize> },
    Or { paren_layer: Box<usize> },
    Imp { paren_layer: Box<usize> },
}

impl Operator {
    fn from_str(s: &str, l: &Box<usize>) -> Option<Operator> {
        match s {
            "not" | "~" | "N" | "[-]" | "!" => Some(Operator::Not {
                paren_layer: l.clone(),
            }),
            "and" | "&" | "K" | "[*]" | "/\\" => Some(Operator::And {
                paren_layer: l.clone(),
            }),
            "or" | "||" | "A" | "[+}" | "\\/" => Some(Operator::Or {
                paren_layer: l.clone(),
            }),
            "imp" | "=>" | "C" => Some(Operator::Imp {
                paren_layer: l.clone(),
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogExpr {
    Atom(Box<Token>),
    Var(Box<Token>),
    UnaryOp(Box<Operator>, Box<LogExpr>),
    BinaryOp(Box<Operator>, Vec<Box<LogExpr>>),
}

impl LogExpr {
    pub fn parse(lexer: &mut Lexer) -> Option<Box<LogExpr>> {
        let token = lexer.next_token();
        match *token.kind {
            TokenKind::LParen => {
                let next_token = lexer.next_token();
                match *next_token.kind {
                    TokenKind::String => {
                        let op =
                            Operator::from_str(next_token.text.as_str(), &next_token.paren_layer)?;
                        let mut args: Vec<Box<LogExpr>> = Vec::new();
                        while *lexer.peek_token().kind != TokenKind::RParen {
                            if let Some(arg) = LogExpr::parse(lexer) {
                                args.push(arg);
                            } else {
                                return None;
                            }
                        }
                        // Skip closing RParen
                        lexer.next();

                        match op {
                            Operator::Not { .. } => {
                                if args.len() == 1 {
                                    Some(Box::new(LogExpr::UnaryOp(Box::new(op), args.pop()?)))
                                } else {
                                    None // Return None if "not" has an invalid number of arguments
                                }
                            }
                            _ => Some(Box::new(LogExpr::BinaryOp(Box::new(op), args))),
                        }
                    }
                    _ => None,
                }
            }
            TokenKind::String => Some(Box::new(LogExpr::Atom(token))),
            _ => None, // Return None for any other unexpected token
        }
    }
}
