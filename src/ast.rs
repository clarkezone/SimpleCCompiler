#![allow(dead_code)]

pub mod ast {

    use std::slice::Iter;
    use lexer::*;

    #[derive(Debug, PartialEq)]
    pub enum AstNodeType {
        Program,
        Function,
        Statement,
        Expression,
    }

    pub struct AstNode {
        pub node_type: AstNodeType,
        pub data: String,
        pub child: Option<Box<AstNode>>,
    }

    impl AstNode {
        fn new(nt: AstNodeType) -> AstNode {
            let fun = AstNode {
                node_type: nt,
                child: None,
                data: String::new(),
            };
            return fun;
        }

        fn new_named(nt: AstNodeType, nm: String) -> AstNode {
            let fun = AstNode {
                node_type: nt,
                child: None,
                data: nm,
            };
            return fun;
        }

        fn add_child(&mut self, ch: Box<AstNode>) {
            self.child = Some(ch);
        }
    }

    pub fn parse(arg: &mut Iter<lexer::TokenInfo>) -> Box<AstNode> {
        let root = AstNode::new(AstNodeType::Program);

        let mut boxedRoot = Box::new(root);

        parse_ast(&mut boxedRoot, arg);

        return boxedRoot;
    }

    fn parse_ast(node: &mut Box<AstNode>, tokens: &mut Iter<lexer::TokenInfo>) {
        match node.node_type {
            AstNodeType::Program => {
                let mut boxed_node = Box::new(AstNode::new((AstNodeType::Function)));
                parse_ast(&mut boxed_node, tokens);

                if boxed_node.data != "main" {
                    panic!("main function has wrong name");
                }
                //verify that fun has correct name
                node.add_child(boxed_node);
            }
            AstNodeType::Function => {
                //consume intkeyword
                let token = tokens.next();
                if token.unwrap().token_type != lexer::TokenType::KeywordInt {
                    panic!("Wrong token type");
                }

                //consume name
                let token_wrapped = tokens.next();

                //TODO error check
                let token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::Identifier {
                    panic!("Wrong token type");
                }
                println!("Found function name:{}", token.data);
                node.data = token.data.clone();

                let token_wrapped = tokens.next();
                //TODO error check
                let token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::OpenParen {
                    panic!("Wrong token type {}", token.token_type);
                }

                let token_wrapped = tokens.next();
                //TODO error check
                let token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::CloseParen {
                    panic!("Wrong token type");
                }

                let token_wrapped = tokens.next();
                //TODO error check
                let token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::OpenBrace {
                    panic!("Wrong token type");
                }

                let mut boxed_node = Box::new(AstNode::new((AstNodeType::Statement)));
                parse_ast(&mut boxed_node, tokens);

                let token_wrapped = tokens.next();
                //TODO error check
                let token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::CloseBrace {
                    panic!("Wrong token type");
                }

                node.add_child(boxed_node);
            }
            AstNodeType::Statement => {
                //consume retkeyword
                let token = tokens.next();
                if token.unwrap().token_type != lexer::TokenType::KeywordRet {
                    panic!("Wrong token type");
                }

                let mut boxed_node = Box::new(AstNode::new((AstNodeType::Expression)));
                parse_ast(&mut boxed_node, tokens);
                node.add_child(boxed_node);

                //consume semicolon
                let token_wrapped = tokens.next();
                let tk2 = token_wrapped.unwrap();
                if tk2.token_type != lexer::TokenType::SemiColon {
                    panic!("Wrong token type");
                }
            }
            AstNodeType::Expression => {
                //consume value
                let mut token_wrapped = tokens.next();
                //TODO error check
                let mut token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::Intliteral {
                    panic!("Wrong token type");
                }

                println!("Found number:{}", token.data);
                node.data = token.data.clone();
            }
        }
    }

    mod test {
        use lexer::*;
        use std::fs;
        use std::borrow::Borrow;

        #[test]
        #[ignore]
        fn test_succeeding() {
            let paths = fs::read_dir("test\\valid").unwrap();
            //let path = paths.next();
            for path in paths {
                let thepath = path.unwrap();
                println!("Testing parser against name: {}", thepath.path().display());
                let token_list = lexer::lex(thepath.path());

                let mut iter = token_list.iter();
                let result = super::parse(&mut iter);

                println!(
                    "Program NodeType:{:?} Data:{}",
                    &result.node_type, &result.data
                );
                assert_eq!(&result.node_type, &super::AstNodeType::Program);

                match result.child {
                    Some(ref x) => {
                        //unwrap() is doing Some(x) we need a reference via Some(ref x)
                        assert_eq!(x.node_type, super::AstNodeType::Function);
                        assert_eq!(x.data, String::from("main"));

                        match x.child {
                            Some(ref y) => {
                                assert_eq!(y.node_type, super::AstNodeType::Statement);

                                match y.child {
                                    Some(ref z) => {
                                        assert_eq!(z.node_type, super::AstNodeType::Expression);
                                    }
                                    None => unreachable!(),
                                }
                            }
                            None => unreachable!(),
                        }
                    }
                    None => unreachable!(),
                }
            }
        }

    }
}
