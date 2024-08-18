mod lexer;
mod parser;
use lexer::*;
use parser::*;
use std::collections::HashMap;
use std::io::{self, Write};

// type Bindings = HashMap<Box<str>, (Box<LogExpr>, Box<LogExpr>)>;

// fn parse(input: &str) -> Result<LogExpr, String> {
//     let tokens = lexer(input);

//     // Check if the input is a command (starts with a command token)
//     if let Some(first_token) = tokens.first() {
//         match first_token {
//             Token::Rule | Token::Apply => {
//                 // Handle the result correctly
//                 let (expr, remaining) = parse_command(&tokens)?;
//                 if !remaining.is_empty() {
//                     Err("Unexpected tokens remaining".into())
//                 } else {
//                     Ok(expr)
//                 }
//             }
//             _ => {
//                 // Parse the expression
//                 let (expr, _) = parse_expr().parse(&tokens)?;
//                 Ok(expr)
//             }
//         }
//     } else {
//         Err("Empty input".into())
//     }
// }

// fn apply(rule_name: &str, expr: &LogExpr, rules: &Bindings) -> Result<Vec<LogExpr>, String> {
//     // Helper function to recursively apply the rule to all subexpressions
//     fn apply_recursively(rule_name: &str, expr: &LogExpr, rules: &Bindings) -> Vec<LogExpr> {
//         let mut results = Vec::new();

//         if let Some((lhs, rhs)) = rules.get(rule_name) {
//             // Check if the left-hand side of the rule matches the current expression
//             if **lhs == *expr {
//                 results.push(*rhs.clone());
//             }
//         }

//         match expr {
//             LogExpr::Not(sub_expr) => {
//                 let sub_results = apply_recursively(rule_name, sub_expr, rules);
//                 for result in sub_results {
//                     results.push(LogExpr::Not(Box::new(result)));
//                 }
//             }
//             LogExpr::And(sub_exprs) => {
//                 for (i, sub_expr) in sub_exprs.iter().enumerate() {
//                     let sub_results = apply_recursively(rule_name, sub_expr, rules);
//                     for result in sub_results.iter() {
//                         let mut new_and_exprs = sub_exprs.clone();
//                         new_and_exprs[i] = Box::new(result.clone());
//                         results.push(LogExpr::And(new_and_exprs.clone()));
//                     }
//                 }
//             }
//             LogExpr::Or(sub_exprs) => {
//                 for (i, sub_expr) in sub_exprs.iter().enumerate() {
//                     let sub_results = apply_recursively(rule_name, sub_expr, rules);
//                     for result in sub_results.iter() {
//                         let mut new_or_exprs = sub_exprs.clone();
//                         new_or_exprs[i] = Box::new(result.clone());
//                         results.push(LogExpr::Or(new_or_exprs.clone()));
//                     }
//                 }
//             }
//             LogExpr::Imp(sub_exprs) => {
//                 for (i, sub_expr) in sub_exprs.iter().enumerate() {
//                     let sub_results = apply_recursively(rule_name, sub_expr, rules);
//                     for result in sub_results.iter() {
//                         let mut new_imp_exprs = sub_exprs.clone();
//                         new_imp_exprs[i] = Box::new(result.clone());
//                         results.push(LogExpr::Imp(new_imp_exprs.clone()));
//                     }
//                 }
//             }
//             _ => {}
//         }

//         results
//     }

//     // Start the recursive application of the rule
//     let mut results = apply_recursively(rule_name, expr, rules);

//     // Add the original expression if no rule was applied
//     if results.is_empty() {
//         results.push(expr.clone());
//     }

//     Ok(results)
// }

// fn main2() {
//     // Use HashMap to store rules: key is the left-hand side of the rule
//     let mut rules: Bindings = HashMap::new();

//     loop {
//         print!("> ");
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();

//         let trimmed_input = input.trim();

//         if trimmed_input == "exit" || trimmed_input == ":quit" {
//             break;
//         }

//         // let tokens = lexer(trimmed_input);
//         // println!("{:?}", tokens);

//         match parse(trimmed_input) {
//             Ok(LogExpr::Rule(name, lhs, rhs)) => {
//                 // Insert the rule into the HashMap
//                 rules.insert(name.clone(), (lhs.clone(), rhs.clone()));
//                 println!("Rule added.");
//             }
//             Ok(LogExpr::Apply(rule_name, expr)) => match apply(&rule_name, &expr, &rules) {
//                 Ok(results) => {
//                     for (i, result) in results.iter().enumerate() {
//                         println!("Result {}: {}", i + 1, result);
//                     }
//                 }
//                 Err(err) => println!("Error applying rule: {}", err),
//             },
//             Ok(expr) => {
//                 println!("{}", expr);
//             }
//             Err(err) => {
//                 println!("Error parsing expression: {}", err);
//             }
//         }
//     }
// }

fn main() {
    // let input = "(:rule demorgans (not (and p q)) (or (not p) (not q)))";
    // let input = "(:load test.wff)";
    // println!("Input: {}", input);
    // let mut lexer = Lexer::new(input, None);
    // while !lexer.complete {
    //     let token = lexer.next();
    //     println!("{:?}", token);
    // }
    // let input2 = ":quit";
    // let mut lexer2 = Lexer::new(input2, None);
    // let command = Command::parse(&mut lexer2);
    // println!("First command: {:?}", command);
    // let command2 = Command::parse(&mut lexer2);
    // println!("Second command: {:?}", command2);
    loop {
        print!("RustyWFF> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(&input, None);
        while let Some(command) = Command::parse(&mut lexer) {
            match *command {
                Command::QuitRepl => {
                    println!("Exiting RustyWFF...");
                    return;
                },
                _ => {
                    println!("{:?}", command);
                }
            }
        }
    }
}
