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

    //let output_file = Path::new("output.S");
    let output_path = thepath.with_extension("S");

    // output_path = match thepath.parent() {
    //     Some(p) => {
    //         //p.join(output_file);
    //         output_file.with_file_name(file_name)
    //     }
    //     None => {
    //         output_file.as_ref();
    //     }
    // };

    let token_list = lex(thepath);

    let mut iter = token_list.iter();
    let result = parse(&mut iter, &|x: String| {
        println!("Error: {}", x);
        std::process::exit(-1);
    });

    let mut code_gen = CodeGenState::new(result);
    code_gen.generate_code_to_file(output_path);
}
