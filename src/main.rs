mod test;
mod fileutils;
mod lexer;
mod ast;
mod codegen;

use std::ops::Index;
use ast::ast::parse;
use lexer::lexer::lex;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use codegen::codegen::CodeGenState;
use std::process::Command;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("missing source file argument");
        std::process::exit(-1);
    }

    let thepath = Path::new(args.index(1));

    let output_path = thepath.with_extension("S");

    let token_list = lex(thepath);

    let mut iter = token_list.iter();
    let result = parse(&mut iter, &|x: String| {
        println!("Error: {}", x);
        std::process::exit(-1);
    });

    let mut code_gen = CodeGenState::new(result);
    code_gen.generate_code_to_file(&output_path);

    gcc_compile(&output_path);
}

fn gcc_compile(file_path: &PathBuf) {
    let mut naked = file_path.clone();
    naked.set_extension("");
    let clean_str = match naked.to_str() {
        Some(s) => s,
        None => {
            panic!("broken");
        }
    };

    let compile = format!("gcc {}.S -o {}", clean_str, clean_str);
    if cfg!(target_os = "windows") {
        println!("<WARNING> Running on windows.. skipping gcc step");
        println!("{}", compile);
        return;
    }

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "gcc file"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(clean_str)
            .output()
            .expect("failed to execute process")
    };

    let hello = std::str::from_utf8(&output.stdout);
    match hello {
        Ok(s) => {
            println!("{}", s);
        }
        Err(_) => {
            panic!("Can't read string");
        }
    }
}
