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
                    let ret_arg = argstack.pop();
                    &mut emitstack.push(String::from("ret"));
                    &mut emitstack.push(String::from(format!("mov ${}, %eax", ret_arg.unwrap())));
                }
                AstNodeType::Function => {
                    &mut emitstack.push(String::from("_main:"));
                    &mut emitstack.push(String::from(".globl _main"));
                }
                AstNodeType::Program => {}
            }
        }
    }

    mod test {
        use lexer::*;
        use std::fs;
        use ast::*;
        use std::path::Path;

        #[test]
        fn test_good_codegen() {
            let thepath = Path::new("test\\valid\\return_2.c");
            let token_list = lexer::lex(thepath);

            let mut iter = token_list.iter();
            let result = ast::parse(&mut iter, &|_s| {});

            let mut emit_stack: &mut Vec<String> = &mut Vec::new();
            let mut code_gen = super::CodeGenState::new(result);
            code_gen.generate_code(&mut emit_stack);

            let expectedlines = vec![
                String::from(".globl main"),
                String::from("main:"),
                String::from("mov $2, %eax"),
                String::from("ret"),
            ];

            let mut itt = expectedlines.iter();

            for i in emit_stack.into_iter().rev() {
                let emmit_res = itt.next();
                assert_eq!(i, emmit_res.unwrap());
            }
        }
    }
}
