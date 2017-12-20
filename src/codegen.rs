#![allow(dead_code)]

pub mod codegen {

    use std::path::Path;
    use fileutils::write_lines;
    use ast::ast::*;

    pub struct CodeGenState {
        current_node: Option<Box<AstNode>>,
    }

    impl CodeGenState {
        pub fn new(root_node: Box<AstNode>) -> CodeGenState {
            let state = CodeGenState {
                current_node: Some(root_node),
            };
            return state;
        }

        pub fn generate_code_to_file<P>(&mut self, filename: P)
        where
            P: AsRef<Path>,
        {
            let mut emit_stack: &mut Vec<String> = &mut Vec::new();
            self.generate_code(&mut emit_stack);
            write_lines(filename, &emit_stack);
        }

        fn generate_code(&mut self, emit_stack: &mut Vec<String>) {
            let arg_stack: &mut Vec<String> = &mut Vec::new();

            match self.current_node {
                Some(ref valid) => {
                    self.generate_code_recurse(valid, arg_stack, emit_stack);
                    // for i in &mut emit_stack.iter().rev() {
                    //     println!("EMMIT:{}", i);
                    // }
                }
                None => {
                    unreachable!();
                }
            }
        }

        fn generate_code_recurse(
            &self,
            node: &AstNode,
            argstack: &mut Vec<String>,
            emitstack: &mut Vec<String>,
        ) {
            match node.child {
                Some(ref valid) => {
                    self.generate_code_recurse(valid, argstack, emitstack);
                }
                None => {} //break recursion by not calling recurse, continuing execution
            }

            //println!("CODEGEN: {:?}", &node.node_type);
            match node.node_type {
                AstNodeType::Expression => {
                    argstack.push(node.data.clone());
                }
                AstNodeType::Statement => {
                    let retArg = argstack.pop();
                    &mut emitstack.push(String::from("ret"));
                    &mut emitstack.push(String::from(format!("mov ${}, %eax", retArg.unwrap())));
                }
                AstNodeType::Function => {
                    &mut emitstack.push(String::from("main:"));
                    &mut emitstack.push(String::from(".globl main"));
                }
                AstNodeType::Program => {}
            }
        }
    }

    mod test {
        use lexer::*;
        use std::fs;
        use ast::*;

        #[test]
        fn test_succeeding() {
            let paths = fs::read_dir("test\\valid").unwrap();
            for path in paths {
                let thepath = path.unwrap();
                println!("Testing parser against name: {}", thepath.path().display());
                let token_list = lexer::lex(thepath.path());

                let mut iter = token_list.iter();
                let result = ast::parse(&mut iter, &|x: String| {});

                let mut emit_stack: &mut Vec<String> = &mut Vec::new();
                let mut code_gen = super::CodeGenState::new(result);
                code_gen.generate_code(&mut emit_stack);

                let expectedlines = vec![
                    String::from(".globl main"),
                    String::from("main:"),
                    String::from("mov "),
                    String::from(r"she{int}ss"),
                    String::from(r"she{ int }ssintss"),
                    String::from(r"int 111;"),
                ];

                let itt = expectedlines.iter();
                //TODO:
                // for i in emit_stack.into_iter() {
                //     itt.
                //     print!("{}:{},", i.token_type as i32, i.data);
                // }

                //verify emit_stack
            }
        }

        #[test]
        fn test_succeeding_write() {
            //TODO test writing to file
        }
    }
}
