mod parser;
mod nodes;

use parser::tokenize;
use parser::parse_expr;

fn main() {

    let source = std::fs::read_to_string("./test.clj").expect("Could not open file");

    let tokens = tokenize(&source).unwrap();

    println!("{}", source);
    println!("{:?}", tokens);

    let node = parse_expr(&mut tokens.iter().peekable()).unwrap();
    node.print();

}
