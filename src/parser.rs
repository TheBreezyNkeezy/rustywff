use crate::lexer::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogExpr {
    Atom(Box<str>),
    Not(Box<LogExpr>),
    And(Vec<Box<LogExpr>>),
    Or(Vec<Box<LogExpr>>),
    Imp(Vec<Box<LogExpr>>),
}

// use std::fmt;

// type ParseResult<'a, T> = Result<(T, &'a [Token]), String>;

// pub struct Parser<'a, T> {
//     parse: Box<dyn Fn(&'a [Token]) -> ParseResult<'a, T> + 'a>,
// }

// impl<'a, T: 'a> Parser<'a, T> {
//     fn new<F>(f: F) -> Self
//     where
//         F: Fn(&'a [Token]) -> ParseResult<'a, T> + 'a,
//     {
//         Parser { parse: Box::new(f) }
//     }

//     pub fn parse(&self, input: &'a [Token]) -> ParseResult<'a, T> {
//         (self.parse)(input)
//     }

//     // fn map<U: 'a>(self, f: impl Fn(T) -> U + 'a) -> Parser<'a, U> {
//     //     Parser::new(move |input| self.parse(input).map(|(value, tokens)| (f(value), tokens)))
//     // }

//     // fn and_then<U: 'a>(self, f: impl Fn(T) -> Parser<'a, U> + 'a) -> Parser<'a, U> {
//     //     Parser::new(move |input| {
//     //         let (value, remaining) = self.parse(input)?;
//     //         f(value).parse(remaining)
//     //     })
//     // }

//     fn or_else(self, other: impl Fn() -> Parser<'a, T> + 'a) -> Parser<'a, T> {
//         Parser::new(move |input| self.parse(input).or_else(|_| other().parse(input)))
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum LogExpr {
//     Atom(Box<str>),
//     Not(Box<LogExpr>),
//     And(Vec<Box<LogExpr>>),
//     Or(Vec<Box<LogExpr>>),
//     Imp(Vec<Box<LogExpr>>),
//     Rule(Box<str>, Box<LogExpr>, Box<LogExpr>), // New variant for rules
//     Apply(Box<str>, Box<LogExpr>),              // New variant for applying rules
// }

// impl fmt::Display for LogExpr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             LogExpr::Atom(s) => write!(f, "{}", s),
//             LogExpr::Not(expr) => write!(f, "(not {})", expr),
//             LogExpr::And(exprs) => {
//                 let exprs_str: Vec<String> = exprs.iter().map(|e| format!("{}", e)).collect();
//                 write!(f, "(and {})", exprs_str.join(" "))
//             }
//             LogExpr::Or(exprs) => {
//                 let exprs_str: Vec<String> = exprs.iter().map(|e| format!("{}", e)).collect();
//                 write!(f, "(or {})", exprs_str.join(" "))
//             }
//             LogExpr::Imp(exprs) => {
//                 let exprs_str: Vec<String> = exprs.iter().map(|e| format!("{}", e)).collect();
//                 write!(f, "(imp {})", exprs_str.join(" "))
//             }
//             LogExpr::Rule(name, lhs, rhs) => write!(f, ":rule {} {} {}", name, lhs, rhs),
//             LogExpr::Apply(rule, expr) => write!(f, ":apply {} {}", rule, expr),
//         }
//     }
// }

// fn atom<'a>() -> Parser<'a, LogExpr> {
//     Parser::new(move |tokens| match tokens {
//         [Token::Symbol(s), rest @ ..] => Ok((LogExpr::Atom(s.clone()), rest)),
//         _ => Err("Expected an atom (single symbol)".into()),
//     })
// }

// fn list<'a>() -> Parser<'a, LogExpr> {
//     Parser::new(move |tokens| {
//         if tokens.is_empty() {
//             return Err("Unexpected end of input".into());
//         }

//         match tokens {
//             [Token::LParen, rest @ ..] => {
//                 let mut exprs = Vec::new();
//                 let mut remaining_tokens = rest;

//                 while !remaining_tokens.is_empty() && remaining_tokens[0] != Token::RParen {
//                     let (expr, rem) = parse_expr().parse(remaining_tokens)?;
//                     exprs.push(expr);
//                     remaining_tokens = rem;
//                 }

//                 // Check if we found the closing parenthesis
//                 if remaining_tokens.is_empty() {
//                     return Err("Expected closing parenthesis".into());
//                 }

//                 // Skip the closing parenthesis
//                 remaining_tokens = &remaining_tokens[1..];

//                 // Match the structure of the list
//                 match exprs.as_slice() {
//                     [LogExpr::Atom(op), expr] if &**op == "not" => {
//                         Ok((LogExpr::Not(Box::new(expr.clone())), remaining_tokens))
//                     }
//                     [LogExpr::Atom(op), rest @ ..] if &**op == "and" => Ok((
//                         LogExpr::And(rest.iter().map(|e| Box::new(e.clone())).collect()),
//                         remaining_tokens,
//                     )),
//                     [LogExpr::Atom(op), rest @ ..] if &**op == "or" => Ok((
//                         LogExpr::Or(rest.iter().map(|e| Box::new(e.clone())).collect()),
//                         remaining_tokens,
//                     )),
//                     [LogExpr::Atom(op), rest @ ..] if &**op == "imp" => Ok((
//                         LogExpr::Imp(rest.iter().map(|e| Box::new(e.clone())).collect()),
//                         remaining_tokens,
//                     )),
//                     [single] => Ok((single.clone(), remaining_tokens)),
//                     _ => Err("Unrecognized list structure".into()),
//                 }
//             }
//             _ => Err("Expected a list (an expression enclosed in parentheses)".into()),
//         }
//     })
// }

// pub fn parse_expr<'a>() -> Parser<'a, LogExpr> {
//     list().or_else(atom)
// }

// pub fn parse_command(tokens: &[Token]) -> Result<(LogExpr, &[Token]), String> {
//     match tokens {
//         [Token::Rule, Token::Name(name), rest @ ..] => {
//             let (lhs, rest) = parse_expr().parse(rest)?;
//             let (rhs, rest) = parse_expr().parse(rest)?;

//             Ok((
//                 LogExpr::Rule(name.clone(), Box::new(lhs), Box::new(rhs)),
//                 rest,
//             ))
//         }
//         [Token::Apply, Token::Name(rule), rest @ ..] => {
//             let (expr, rest) = parse_expr().parse(rest)?;
//             Ok((LogExpr::Apply(rule.clone(), Box::new(expr)), rest))
//         }
//         _ => Err("Unrecognized command or syntax error".into()),
//     }
// }
