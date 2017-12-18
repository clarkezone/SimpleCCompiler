mod codegen {

    use ast::ast::AstNode;

    struct CodeGenState {
        current_node: Option<Box<AstNode>>,
    }

    impl CodeGenState {
        fn new(root_node: Box<AstNode>) -> CodeGenState {
            let state = CodeGenState {
                current_node: Some(root_node),
            };
            return state;
        }

        fn generate_code(&self) {
            match self.current_node {
                Some(ref valid) => {
                    self.generate_code_recurse(valid);
                }
                None => {
                    unreachable!();
                }
            }
        }

        fn generate_code_recurse(&self, node: &AstNode) {
            match node.child {
                Some(ref valid) => {
                    self.generate_code_recurse(valid);
                }
                None => {} //break recursion by not calling recurse, continuing execution
            }

            println!("CODEGEN: {:?}", &node.node_type);
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
                let result = ast::parse(&mut iter);

                let code_gen = super::CodeGenState::new(result);
                code_gen.generate_code();
            }
        }
    }
}
