#[deny(missing_docs)]
mod ast;
mod eval;
mod parser;

use std::fs;
use std::io::prelude::*;

fn main() {
    let args = std::env::args();
    if args.len() > 1 {
        for arg in args.skip(1) {
            let source = fs::read_to_string(&arg).expect("Could not read source file");
            print(eval::eval(parser::parse(&source)));
        }
    } else {
        let mut env = eval::make_global_env();
        loop {
            print(eval::eval_with_env(read(), &mut env));
        }
    }
}

fn read() -> ast::Expr {
    let mut buff = String::new();
    print!("\u{1F9D9}  > ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buff).unwrap();
    parser::parse(&buff)
}

fn print(result: eval::EvalResult) {
    match result {
        Ok(value) => println!(" ~> {}", value),
        Err(error) => println!(" !! {}", error),
    }
}