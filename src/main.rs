mod lexer;
mod parser;
use lexer::*;
use parser::*;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut rule_set = RuleSet::new();
    loop {
        print!("RustyWFF> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(&input, None);
        match Command::parse(&mut lexer) {
            Ok(command) => match *command {
                Command::QuitRepl => {
                    println!("Exiting RustyWFF...");
                    return;
                }
                Command::DefineRule { name, lhs, rhs } => {
                    rule_set.add_rule(name, lhs, rhs);
                    println!("\tRule added:\n{}", rule_set);
                }
                Command::DeleteRule { name } => {
                    rule_set.delete_rule(name);
                    println!("\tRule deleted: {}.", rule_set);
                }
                Command::ApplyRule { name, expr } => {
                    let results = expr.apply_rule(&rule_set, &name);
                    if results.is_empty() {
                        println!("\tNo match found.");
                    } else {
                        for (i, (result, bindings)) in results.iter().enumerate() {
                            println!("\tResult {}: {}", i + 1, result);
                            if !bindings.is_empty() {
                                println!("\tVAR MATCHES:");
                                for (var, bound_expr) in bindings {
                                    println!("\t\t{} => {}", var, bound_expr);
                                }
                            }
                        }
                    }
                }
                _ => {
                    println!("{:?}", command);
                }
            },
            Err(e) => {
                println!("Error parsing command: {:?}", e);
            }
        }
    }
}
