use juice::{flow::Flow, interpreter::Interpreter};
use lexer::Lexer;
use parser::Parser;

pub mod ast;
pub mod juice;
pub mod lexer;
pub mod parser;
pub mod token;

fn eval(source: &str) {
    let tokens = Lexer::new(source).lex();
    let statements = Parser::new(tokens).parse();
    let result = Interpreter::new()
        .with_std()
        .unwrap()
        .interprete(&statements);

    if let Err(flow) = &result {
        if let Flow::Error(msg) = flow {
            panic!("{}", msg);
        }
    }
}

fn main() {
    if let Some(filename) = std::env::args().nth(1) {
        if !filename.ends_with(".juice") {
            panic!("File must have .juice extension");
        }
        match std::fs::read_to_string(filename) {
            Ok(source) => eval(&source),
            Err(err) => panic!("Error reading file: {}", err),
        }
    } else {
        println!("Please provide a filename as argument");
    }
}
