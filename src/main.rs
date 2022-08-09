pub mod ast_printer;
pub mod exceptions;
pub mod expression;
pub mod parser;
pub mod scanner;
pub mod token;

use exceptions::Exce;
use scanner::Scanner;
use std::{env, fs::File, io::Read, path::Path};

use crate::ast_printer::ast_printer;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() > 2 {
        println!("Usage: cargo run [script]");
        return;
    } else if args.len() == 2 {
        run_file(args[1].clone()).unwrap();
    } else {
        todo!();
        // run_prompt();
    }
}

fn run_file(filename: String) -> Result<(), Exce> {
    let path = Path::new(&filename);
    if !path.exists() {
        println!("File doesn\'t exist!");
    }

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);

    run(contents)
}

fn run(source: String) -> Result<(), Exce> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens.iter() {
        println!("{:?}", token);
    }

    Ok(())
}

#[test]
fn test_file() {
    assert!(run_file("test.lox".to_string()).is_ok());
}

#[test]
fn test_ast_printer() {
    use expression::Expr::*;
    use token::Token::*;

    let expression = Binary {
        left: Box::new(Unary {
            op: Minus { line: 1 },
            expr: Box::new(Literal {
                val: Number {
                    line: 1,
                    value: 123.0,
                },
            }),
        }),
        op: Star { line: 1 },
        right: Box::new(Grouping {
            expr: Box::new(Literal {
                val: Number {
                    line: 1,
                    value: 45.67,
                },
            }),
        }),
    };

    println!("{}", ast_printer(expression));
}
