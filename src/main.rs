mod parser;
mod nodes;

use parser::tokenize;
use parser::parse_expr;
use crate::parser::TokenKind;

fn main() {

    let source = std::fs::read_to_string("./test.clj").expect("Could not open file");
    let tokens = tokenize(&source).unwrap();

    println!("Source:\n\t{}", source.trim());
    println!("Tokens:\n\t{:?}", tokens.iter().map(|token| &token.0).collect::<Vec<&TokenKind>>());

    print!("Nodes:\n\t");
    let parseResult = parse_expr(&mut tokens.iter().peekable());

    match parseResult {
        Ok(node) => node.print(),
        Err(error) => eprintln!("{}", error)
    }

}
