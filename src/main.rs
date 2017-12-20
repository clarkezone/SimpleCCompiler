mod test;
mod fileutils;
mod lexer;
mod ast;
mod codegen;

use std::ops::Index;
use ast::ast::parse;
use lexer::lexer::lex;
use std::path::Path;
use std::fs;
use codegen::codegen::CodeGenState;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("missing source file argument");
        std::process::exit(-1);
    }

    let thepath = Path::new(args.index(1));
    let outputPath = Path::new("output.S");

    let token_list = lex(thepath);

    let mut iter = token_list.iter();
    let result = parse(&mut iter, &|x: String| {
        println!("Error: {}", x);
        std::process::exit(-1);
    });

    let mut emit_stack: &mut Vec<String> = &mut Vec::new();
    let mut code_gen = CodeGenState::new(result);
    code_gen.generate_code_to_file(outputPath);
}
