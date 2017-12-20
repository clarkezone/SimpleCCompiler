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

    pub fn parse<F>(arg: &mut Iter<lexer::TokenInfo>, errorf: &F) -> Box<AstNode>
    where
        F: Fn(String),
    {
        let root = AstNode::new(AstNodeType::Program);

        let mut boxedRoot = Box::new(root);

        parse_ast(&mut boxedRoot, arg, errorf);

        return boxedRoot;
    }

    fn parse_ast<F>(node: &mut Box<AstNode>, tokens: &mut Iter<lexer::TokenInfo>, errorf: &F)
    where
        F: Fn(String),
    {
        match node.node_type {
            AstNodeType::Program => {
                let mut boxed_node = Box::new(AstNode::new((AstNodeType::Function)));
                parse_ast(&mut boxed_node, tokens, errorf);

                if boxed_node.data != "main" {
                    errorf(String::from("main function has wrong name"));
                }
                //verify that fun has correct name
                node.add_child(boxed_node);
            }
            AstNodeType::Function => {
                //consume intkeyword
                let token = tokens.next();
                if token.unwrap().token_type != lexer::TokenType::KeywordInt {
                    errorf(String::from("Wrong token type: expected Int keyword"));
                }

                //consume name
                let token_wrapped = tokens.next();

                //TODO error check
                let token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::Identifier {
                    errorf(String::from("Wrong token type: expected identifier"));
                }
                //println!("Found function name:{}", token.data);
                node.data = token.data.clone();

                let token_wrapped = tokens.next();
                //TODO error check
                let token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::OpenParen {
                    errorf(String::from(format!(
                        "Wrong token type {}: expected open paren",
                        token.token_type,
                    )));
                }

                let token_wrapped = tokens.next();
                //TODO error check
                let token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::CloseParen {
                    errorf(String::from("Wrong token type: expected close paren"));
                }

                let token_wrapped = tokens.next();
                //TODO error check
                let token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::OpenBrace {
                    errorf(String::from("Wrong token type: expected open brace"));
                }

                let mut boxed_node = Box::new(AstNode::new((AstNodeType::Statement)));
                parse_ast(&mut boxed_node, tokens, errorf);

                let token_wrapped = tokens.next();
                //TODO error check
                let token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::CloseBrace {
                    errorf(String::from("Wrong token type: expected close brace"));
                }

                node.add_child(boxed_node);
            }
            AstNodeType::Statement => {
                //consume retkeyword
                //TODO error check
                let token = tokens.next();
                if token.unwrap().token_type != lexer::TokenType::KeywordRet {
                    errorf(String::from("Wrong token type: expected return keyword"));
                }

                let mut boxed_node = Box::new(AstNode::new((AstNodeType::Expression)));
                parse_ast(&mut boxed_node, tokens, errorf);
                node.add_child(boxed_node);

                //consume semicolon
                let token_wrapped = tokens.next();
                let tk2 = token_wrapped.unwrap();
                if tk2.token_type != lexer::TokenType::SemiColon {
                    errorf(String::from("Wrong token type: expected semi-colon"));
                }
            }
            AstNodeType::Expression => {
                //consume value
                let mut token_wrapped = tokens.next();
                //TODO error check
                let mut token = token_wrapped.unwrap();
                if token.token_type != lexer::TokenType::Intliteral {
                    errorf(String::from("Wrong token type: expceted int literal"));
                }

                //println!("Found number:{}", token.data);
                node.data = token.data.clone();
            }
        }
    }

    mod test {
        use lexer::*;
        use std::fs;
        use std::borrow::Borrow;

        #[test]
        //#[ignore]
        fn test_succeeding() {
            let paths = fs::read_dir("test\\valid").unwrap();
            for path in paths {
                let thepath = path.unwrap();
                println!("Testing parser against name: {}", thepath.path().display());
                let token_list = lexer::lex(thepath.path());

                let mut iter = token_list.iter();
                let result = super::parse(&mut iter, &|x: String| {
                    panic!(x);
                });

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
