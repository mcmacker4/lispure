mod parser;
mod nodes;
mod context;
mod eval;

use parser::tokenize;
use crate::parser::{parse_file, TokenKind};
use std::io::Write;
use crate::eval::eval_file;
use crate::context::EvalContext;


fn main() {
    repl().unwrap();
}

fn repl() -> std::io::Result<()> {

    let mut context = EvalContext::new();

    loop {

        print!("> ");
        std::io::stdout().flush()?;

        let mut code = String::new();
        std::io::stdin().read_line(&mut code)?;

        if code.trim().eq("exit") {
            println!("Goodbye!");
            break;
        }

        let tokens = match tokenize(&code) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        let expr = match parse_file(&mut tokens.iter().peekable()) {
            Ok(expr) => expr,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        let result = eval::eval_file(&mut context, &expr);
        match result {
            Ok(result) => {
                result.print();
                println!();
            },
            Err(err) => println!("Error: {}", err)
        }

        std::io::stdout().flush()?;

    }

    Ok(())

}

fn _run_file() {

    let source = std::fs::read_to_string("./test.clj").expect("Could not open file");
    let tokens = tokenize(&source).unwrap();

    println!("Source:\n\t{}", source.trim());
    println!("Tokens:\n\t{:?}", tokens.iter().map(|token| &token.0).collect::<Vec<&TokenKind>>());

    print!("Nodes:\n\t");
    let parse_result = parse_file(&mut tokens.iter().peekable());

    let mut context = EvalContext::new();

    match &parse_result {
        Ok(node) => {
            let result = eval_file(&mut context, node).unwrap();
            result.print()
        },
        Err(error) => eprintln!("{}", error)
    }

}
