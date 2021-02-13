mod parser;
mod nodes;
mod eval;

use parser::tokenize;
use crate::parser::parse_file;
use std::io::Write;


fn main() {
    repl().unwrap();
}

fn repl() -> std::io::Result<()> {

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

        let result = eval::eval_file(&expr);
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

#[test]
fn parse_test() {

    let source = std::fs::read_to_string("./test.clj").expect("Could not open file");
    let tokens = tokenize(&source).unwrap();

    println!("Source:\n\t{}", source.trim());
    println!("Tokens:\n\t{:?}", tokens.iter().map(|token| &token.0).collect::<Vec<&TokenKind>>());

    print!("Nodes:\n\t");
    let parse_result = parse_expr(&mut tokens.iter().peekable());

    match parse_result {
        Ok(node) => {
            let result = run_code(node);
            result.print()
        },
        Err(error) => eprintln!("{}", error)
    }

}
