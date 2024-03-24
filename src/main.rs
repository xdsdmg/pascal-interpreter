use core::panic;
use error::Error;
use std::{env, fs};

mod ast;
mod error;
mod global_scope;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod utils;

fn main() {
    /* Read code from file */
    let mut args = env::args();
    args.next();
    let file_name = match args.next() {
        Some(f) => f,
        None => panic!("{}", Error::FileNotFound),
    };

    println!("Read code from {}", file_name);

    let code = fs::read_to_string(file_name).expect("Something went wrong when reading the file");

    /* Interpreter execute */
    let mut interpreter = match interpreter::Interpreter::new(&code) {
        Ok(interpreter) => interpreter,
        Err(e) => {
            panic!("Initialize interpreter failed, error: {}", e);
        }
    };

    if let Err(e) = interpreter.execute() {
        panic!("Interpreter execute failed, error: {}", e);
    }
}
