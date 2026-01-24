mod interpreter;
mod lexer;
mod parser;

use crate::interpreter::{Interpreter, Value};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Foydalanish: uzlang <fayl_nomi>");
        return;
    }

    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Faylni o'qishda xatolik");

    let mut lexer = Lexer::new(&code);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    // Demo uchun 'raqam' o'zgaruvchisini qo'shamiz (Python versiyadagidek)
    interpreter.set_variable("raqam", Value::Number(5));

    interpreter.execute(&ast);
}
