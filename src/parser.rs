use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

use crate::lexer::*;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEndOfInput,
    ExpectedFilePath,
    ExpectedRuleName,
    ExpectedExpression,
}

#[derive(Debug, Clone)]
pub enum Command {
    QuitRepl,
    DeleteRule {
        name: Box<String>,
    },
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

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::QuitRepl => write!(f, "quit"),
            Command::DefineRule { name, lhs, rhs } => {
                write!(f, "rule {} {} {}", name, lhs, rhs)
            }
            Command::DeleteRule { name } => write!(f, "delete {}", name),
            Command::ApplyRule { name, expr } => write!(f, "apply {} {}", name, expr),
            Command::LoadFile { file_path } => write!(f, "load {}", file_path),
            Command::SaveFile { file_path } => write!(f, "save {}", file_path),
            Command::Eval { expr } => write!(f, "{}", expr),
        }
    }
}

impl Command {
    pub fn parse(lexer: &mut Lexer) -> Result<Box<Command>, ParserError> {
        let token = lexer.peek_token();
        match *token.kind {
            TokenKind::Quit => {
                lexer.next();
                Ok(Box::new(Command::QuitRepl))
            }
            TokenKind::Load => {
                lexer.next();
                let file_path = lexer
                    .next()
                    .ok_or(ParserError::ExpectedFilePath)?
                    .text
                    .clone();
                Ok(Box::new(Command::LoadFile { file_path }))
            }
            TokenKind::Save => {
                lexer.next();
                let file_path = lexer
                    .next()
                    .ok_or(ParserError::ExpectedFilePath)?
                    .text
                    .clone();
                Ok(Box::new(Command::SaveFile { file_path }))
            }
            TokenKind::Rule => {
                lexer.next();
                let name = lexer
                    .next()
                    .ok_or(ParserError::ExpectedRuleName)?
                    .text
                    .clone();
                let lhs = LogExpr::parse(lexer).ok_or(ParserError::ExpectedExpression)?;
                let rhs = LogExpr::parse(lexer).ok_or(ParserError::ExpectedExpression)?;
                Ok(Box::new(Command::DefineRule { name, lhs, rhs }))
            }
            TokenKind::Delete => {
                lexer.next();
                let name = lexer
                    .next()
                    .ok_or(ParserError::ExpectedRuleName)?
                    .text
                    .clone();
                Ok(Box::new(Command::DeleteRule { name }))
            }
            TokenKind::Apply => {
                lexer.next();
                let name = lexer
                    .next()
                    .ok_or(ParserError::ExpectedRuleName)?
                    .text
                    .clone();
                let expr = LogExpr::parse(lexer).ok_or(ParserError::ExpectedExpression)?;
                Ok(Box::new(Command::ApplyRule { name, expr }))
            }
            TokenKind::End => {
                lexer.next();
                Err(ParserError::UnexpectedEndOfInput)
            }
            _ => {
                let expr = LogExpr::parse(lexer).ok_or(ParserError::ExpectedExpression)?;
                Ok(Box::new(Command::Eval { expr }))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Not,
    And,
    Or,
    Imp,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Not { .. } => write!(f, "not"),
            Operator::And { .. } => write!(f, "and"),
            Operator::Or { .. } => write!(f, "or"),
            Operator::Imp { .. } => write!(f, "imp"),
        }
    }
}

impl Operator {
    fn from_str(s: &str) -> Option<Operator> {
        match s {
            "not" | "~" | "N" | "[-]" | "!" => Some(Operator::Not),
            "and" | "&" | "K" | "[*]" | "/\\" => Some(Operator::And),
            "or" | "||" | "A" | "[+}" | "\\/" => Some(Operator::Or),
            "imp" | "=>" | "C" => Some(Operator::Imp),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub lhs: Box<LogExpr>,
    pub rhs: Box<LogExpr>,
}

#[derive(Debug)]
pub struct RuleSet {
    pub rules: HashMap<Box<String>, Box<Rule>>,
}

impl Display for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rules_iter = self.rules.iter();
        if let Some((name, rule)) = rules_iter.next() {
            write!(f, "\t\t{}: {} => {}\n", name, rule.lhs, rule.rhs)?;
            for (name, rule) in rules_iter {
                write!(f, "\t\t{}: {} => {}\n", name, rule.lhs, rule.rhs)?;
            }
        }
        Ok(())
    }
}

impl RuleSet {
    pub fn new() -> RuleSet {
        RuleSet {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, name: Box<String>, lhs: Box<LogExpr>, rhs: Box<LogExpr>) {
        self.rules.insert(name, Box::new(Rule { lhs, rhs }));
    }

    pub fn get_rule(&self, name: Box<String>) -> Option<&Box<Rule>> {
        self.rules.get(&name)
    }

    pub fn delete_rule(&mut self, name: Box<String>) -> Option<Box<Rule>> {
        self.rules.remove(&name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogExpr {
    Atom(Box<Token>),
    Var(Box<Token>),
    UnaryOp(Box<Operator>, Box<LogExpr>),
    BinaryOp(Box<Operator>, Vec<Box<LogExpr>>),
    True,
    False,
}

impl Display for LogExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogExpr::Atom(token) => write!(f, "{}", token.text),
            LogExpr::Var(token) => write!(f, "{}", token.text),
            LogExpr::UnaryOp(op, expr) => write!(f, "({} {})", op, expr),
            LogExpr::BinaryOp(op, exprs) => {
                let mut exprs_iter = exprs.iter();
                let first_expr = exprs_iter.next().unwrap();
                write!(f, "({} {}", op, first_expr)?;
                for expr in exprs_iter {
                    write!(f, " {}", expr)?;
                }
                write!(f, ")")
            }
            LogExpr::True => write!(f, "true"),
            LogExpr::False => write!(f, "false"),
        }
    }
}

impl LogExpr {
    fn string_true(s: &str) -> bool {
        matches!(s.to_lowercase().as_str(), "1" | "t" | "true")
    }

    fn string_false(s: &str) -> bool {
        matches!(s.to_lowercase().as_str(), "0" | "f" | "false")
    }

    pub fn parse(lexer: &mut Lexer) -> Option<Box<LogExpr>> {
        let token = lexer.next_token();
        match *token.kind {
            TokenKind::LParen => {
                let next_token = lexer.next_token();
                match *next_token.kind {
                    TokenKind::String => {
                        let op = Operator::from_str(next_token.text.as_str())?;
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
            TokenKind::String => {
                let text = token.text.as_str();
                if LogExpr::string_false(text) {
                    Some(Box::new(LogExpr::False))
                } else if LogExpr::string_true(text) {
                    Some(Box::new(LogExpr::True))
                } else {
                    let first_char = text.chars().next()?;
                    if first_char.is_lowercase() || first_char.is_digit(10) {
                        Some(Box::new(LogExpr::Atom(token)))
                    } else if first_char.is_uppercase() {
                        Some(Box::new(LogExpr::Var(token)))
                    } else {
                        None
                    }
                }
            }
            _ => None, // Return None for any other unexpected token
        }
    }

    pub fn match_with(
        &self,
        other: &LogExpr,
        bindings: &mut HashMap<Box<String>, Box<LogExpr>>,
    ) -> bool {
        match (self, other) {
            // Atoms should directly match
            (LogExpr::Atom(token1), LogExpr::Atom(token2)) => token1 == token2,

            // Variables can bind to expressions
            (LogExpr::Var(var_token), expr) => {
                let var_name = var_token.text.clone();
                if let Some(bound_expr) = bindings.get(&var_name) {
                    *bound_expr == Box::new(expr.clone())
                } else {
                    bindings.insert(var_name, Box::new(expr.clone()));
                    true
                }
            }

            // Unary operations must match their operation type and subexpression
            (LogExpr::UnaryOp(op1, expr1), LogExpr::UnaryOp(op2, expr2)) => {
                op1 == op2 && expr1.match_with(expr2, bindings)
            }

            // Binary operations must match their operation type and subexpressions
            (LogExpr::BinaryOp(op1, exprs1), LogExpr::BinaryOp(op2, exprs2)) => {
                if op1 == op2 && exprs1.len() == exprs2.len() {
                    exprs1
                        .iter()
                        .zip(exprs2.iter())
                        .all(|(e1, e2)| e1.match_with(e2, bindings))
                } else {
                    false
                }
            }

            // Other combinations do not match
            _ => false,
        }
    }

    pub fn apply_rule(
        &self,
        rule_set: &RuleSet,
        rule_name: &str,
    ) -> Vec<(Box<LogExpr>, HashMap<Box<String>, Box<LogExpr>>)> {
        let mut results = Vec::new();
        let mut all_bindings = HashMap::new();
        let mut unique_results = HashSet::new(); // Set to ensure uniqueness

        if let Some(rule) = rule_set.get_rule(Box::new(rule_name.to_string())) {
            let mut bindings = HashMap::new();

            // Direct match of the entire expression
            if rule.lhs.match_with(self, &mut bindings) {
                let substituted_expr = rule.rhs.substitute(&bindings);
                if unique_results.insert(substituted_expr.clone()) {
                    results.push((substituted_expr, bindings.clone()));
                }
                all_bindings.extend(bindings.clone());
            }

            // Apply rule to subexpressions if no direct match
            match self {
                LogExpr::UnaryOp(op, expr) => {
                    let sub_results = expr.apply_rule(rule_set, rule_name);
                    for (sub_expr, sub_bindings) in sub_results {
                        let new_expr = Box::new(LogExpr::UnaryOp(op.clone(), sub_expr));
                        if unique_results.insert(new_expr.clone()) {
                            results.push((new_expr, sub_bindings.clone()));
                        }
                        all_bindings.extend(sub_bindings);
                    }
                }
                LogExpr::BinaryOp(op, exprs) => {
                    for (i, expr) in exprs.iter().enumerate() {
                        let other_exprs = exprs
                            .iter()
                            .enumerate()
                            .filter(|(j, _)| *j != i)
                            .map(|(_, e)| e.clone())
                            .collect::<Vec<_>>();
                        let sub_results = expr.apply_rule(rule_set, rule_name);
                        for (sub_result, sub_bindings) in sub_results {
                            let mut new_exprs = other_exprs.clone();
                            new_exprs.insert(i, sub_result);
                            let combined_expr = Box::new(LogExpr::BinaryOp(op.clone(), new_exprs));
                            if unique_results.insert(combined_expr.clone()) {
                                results.push((combined_expr.clone(), sub_bindings.clone()));
                            }
                            all_bindings.extend(sub_bindings.clone());

                            // Apply rule to the resulting combined expression recursively
                            let combined_sub_results =
                                combined_expr.apply_rule(rule_set, rule_name);
                            for (final_expr, final_bindings) in combined_sub_results {
                                if unique_results.insert(final_expr.clone()) {
                                    results.push((final_expr.clone(), final_bindings.clone()));
                                }
                                all_bindings.extend(final_bindings);
                            }
                        }
                    }
                }
                _ => {}
            }

            // If there are multiple results, generate the combined result
            if results.len() > 1 {
                let mut combined_expr = rule.rhs.clone();
                let mut combined_bindings = HashMap::new();

                for (i, (_, bindings)) in results.iter().enumerate() {
                    for (var, bound_expr) in bindings {
                        let new_var_name: Box<String> = format!("{}_{}", var, i + 1).into();
                        combined_bindings.insert(new_var_name.clone(), bound_expr.clone());
                        combined_expr = combined_expr.substitute(&combined_bindings);
                    }
                }

                results.push((combined_expr, combined_bindings));
            }
        }
        results
    }

    fn substitute(&self, bindings: &HashMap<Box<String>, Box<LogExpr>>) -> Box<LogExpr> {
        match self {
            LogExpr::Atom(token) => {
                if let Some(subst) = bindings.get(&token.text) {
                    subst.clone()
                } else {
                    Box::new(LogExpr::Atom(token.clone()))
                }
            }
            LogExpr::Var(token) => {
                if let Some(subst) = bindings.get(&token.text) {
                    subst.clone()
                } else {
                    Box::new(LogExpr::Var(token.clone()))
                }
            }
            LogExpr::UnaryOp(op, expr) => {
                Box::new(LogExpr::UnaryOp(op.clone(), expr.substitute(bindings)))
            }
            LogExpr::BinaryOp(op, exprs) => Box::new(LogExpr::BinaryOp(
                op.clone(),
                exprs.iter().map(|e| e.substitute(bindings)).collect(),
            )),
            _ => Box::new(self.clone()),
        }
    }
}
