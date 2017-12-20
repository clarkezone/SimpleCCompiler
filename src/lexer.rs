#![allow(dead_code)]

pub mod lexer {
    use std::fmt::*;
    use std::path::Path;
    use fileutils::lines_from_file;
    use std::iter::FromIterator;
    use std::*;

    #[derive(Clone, PartialEq, Debug)]
    pub enum TokenType {
        OpenBrace,
        CloseBrace,
        OpenParen,
        CloseParen,
        SemiColon,
        KeywordInt,
        KeywordRet,
        Identifier,
        Intliteral,
    }

    impl Display for TokenType {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{:?}", self)
        }
    }

    pub struct TokenInfo {
        pub token_type: TokenType,
        source_line: u32,
        start_char: u32,
        end_char: u32,
        pub data: String,
    }

    pub fn lex<P>(filename: P) -> Vec<TokenInfo>
    where
        P: AsRef<Path>,
    {
        let lines = lines_from_file(filename);
        let mut collector: Vec<TokenInfo> = Vec::new();
        lexinternal(lines, &mut collector);
        return collector;
    }

    fn lexinternal(lines: Vec<String>, collector: &mut Vec<TokenInfo>) {
        let mut line_num: u32 = 1;
        for line in lines {
            get_tokens(&line, line_num, collector);
            line_num = line_num + 1;
        }
    }

    fn get_tokens(line: &String, line_num: u32, collector: &mut Vec<TokenInfo>) {
        let mut bb = LexerState::from(line.chars());
        let mut slast_char;

        match bb.next() {
            None => return,
            Some(i) => slast_char = i,
        }
        loop {
            //TODO: lastchar in LexerState
            //TODO: improve EOD detection using LexerState
            //TODO: remove repeated token init code

            let classified = classify(&slast_char);
            let mut advanced: bool = false;

            match classified {
                CharClassified::Alpha(_al) => {
                    //TODO: use AL rather than lastchar
                    let result = readallconf(&mut bb, &mut slast_char, |x: &char| {
                        return is_word_char(x);
                    });

                    match result.0.as_str() {
                        "int" => {
                            //println!("kw: int");
                            let new_token_info = TokenInfo {
                                token_type: TokenType::KeywordInt,
                                source_line: line_num,
                                start_char: result.2,
                                end_char: result.3,
                                data: String::new(),
                            };
                            collector.push(new_token_info);
                        }
                        "return" => {
                            //println!("kw: return");
                            let new_token_info = TokenInfo {
                                token_type: TokenType::KeywordRet,
                                source_line: line_num,
                                start_char: result.2,
                                end_char: result.3,
                                data: String::new(),
                            };
                            collector.push(new_token_info);
                        }
                        _ => {
                            //println!("Found Char Literal:{}", result.0);
                            let new_token_info = TokenInfo {
                                token_type: TokenType::Identifier,
                                source_line: line_num,
                                start_char: result.2,
                                end_char: result.3,
                                data: result.0,
                            };
                            collector.push(new_token_info);
                        }
                    }

                    advanced = result.1;
                }
                CharClassified::Number(_al) => {
                    let result =
                        readallconf(&mut bb, &mut slast_char, |x: &char| return is_number(x));
                    //println!("Found number:{}", result.0);
                    let new_token_info = TokenInfo {
                        token_type: TokenType::Intliteral,
                        source_line: line_num,
                        start_char: result.2,
                        end_char: result.3,
                        data: result.0,
                    };
                    collector.push(new_token_info);
                    advanced = result.1;
                }
                CharClassified::Symbol('(') => {
                    //println!("Found ( {}", bb.cnt);
                    let new_token_info = TokenInfo {
                        token_type: TokenType::OpenParen,
                        source_line: line_num,
                        start_char: bb.cnt,
                        end_char: bb.cnt + 1,
                        data: String::new(),
                    };
                    collector.push(new_token_info);
                }
                CharClassified::Symbol(')') => {
                    //println!("Found )");
                    let new_token_info = TokenInfo {
                        token_type: TokenType::CloseParen,
                        source_line: line_num,
                        start_char: bb.cnt,
                        end_char: bb.cnt + 1,
                        data: String::new(),
                    };
                    collector.push(new_token_info);
                }
                CharClassified::Symbol('{') => {
                    //println!("Found {{");
                    let new_token_info = TokenInfo {
                        token_type: TokenType::OpenBrace,
                        source_line: line_num,
                        start_char: bb.cnt,
                        end_char: bb.cnt + 1,
                        data: String::new(),
                    };
                    collector.push(new_token_info);
                }
                CharClassified::Symbol('}') => {
                    //println!("Found }}");
                    let new_token_info = TokenInfo {
                        token_type: TokenType::CloseBrace,
                        source_line: line_num,
                        start_char: bb.cnt,
                        end_char: bb.cnt + 1,
                        data: String::new(),
                    };
                    collector.push(new_token_info);
                }
                CharClassified::Symbol(';') => {
                    //println!("Found ;");
                    let new_token_info = TokenInfo {
                        token_type: TokenType::SemiColon,
                        source_line: line_num,
                        start_char: bb.cnt,
                        end_char: bb.cnt + 1,
                        data: String::new(),
                    };
                    collector.push(new_token_info);
                }
                CharClassified::Symbol(_al) => {} //println!("Found other symbol {}", al),
            }

            if !advanced {
                match bb.next() {
                    None => break,
                    Some(i) => slast_char = i,
                }
            }
        }
    }

    fn readallconf<F>(iter: &mut LexerState, last: &mut char, f: F) -> (String, bool, u32, u32)
    where
        F: Fn(&char) -> bool,
    {
        let mut literal: Vec<char> = Vec::new();
        let start = iter.cnt;
        literal.push(*last);
        let mut advanced = true;
        loop {
            match iter.next() {
                None => {
                    advanced = false; //parent function needs to advance in order to break out of it's loop
                    break;
                }
                Some(i) => *last = i,
            }
            if f(&last) {
                literal.push(*last);
            } else {
                break;
            }
        }
        let end = iter.cnt;
        let maa = String::from_iter(literal);
        return (maa, advanced, start, end);
    }

    struct LexerState<'b> {
        char_iter: str::Chars<'b>,
        cnt: u32,
        eof: bool,
    }

    impl<'a> LexerState<'a> {
        fn from(i: str::Chars<'a>) -> LexerState<'a> {
            let b = Self {
                char_iter: i,
                cnt: 0,
                eof: false,
            };

            return b;
        }

        fn next(&mut self) -> Option<char> {
            match self.char_iter.next() {
                None => {
                    self.eof = true;
                    return None;
                }
                Some(t) => {
                    self.cnt = self.cnt + 1;
                    return Some(t);
                }
            }
        }
    }

    enum CharClassified {
        Number(char),
        Alpha(char),
        Symbol(char),
    }

    static ALPHA: &'static str = "abcdefghijklmnopqrstuvwzyzABCDEFGHIJKLMNOPQRSTUVWZYZ";
    static NUMERIC: &'static str = "0123456789";

    fn classify(input: &char) -> CharClassified {
        if is_word_char(input) {
            return CharClassified::Alpha(*input);
        } else if is_number(input) {
            return CharClassified::Number(*input);
        } else {
            return CharClassified::Symbol(*input);
        }
    }

    fn is_word_char(input: &char) -> bool {
        let foo = ALPHA.find(*input);
        return foo != None;
    }

    fn is_number(input: &char) -> bool {
        let foo = NUMERIC.find(*input);
        return foo != None;
    }

    mod tests {
        use std::fs;
        //#[ignore]
        #[test]
        fn test_token_extract() {
            let lines = vec![
                String::from(r"foo{"),
                String::from(r"shebar}ss{"),
                String::from(r"she{bar}ss"),
                String::from(r"she{int}ss"),
                String::from(r"she{ int }ssintss"),
                String::from(r"int 111;"),
            ];

            let mut collector: Vec<super::TokenInfo> = Vec::new();
            super::lexinternal(lines, &mut collector);

            print!("Token list:");
            for i in collector.into_iter() {
                print!("{}:{},", i.token_type as i32, i.data);
            }
        }

        //#[ignore]
        #[test]
        fn test_succeeding() {
            let paths = fs::read_dir("test\\valid").unwrap();

            for path in paths {
                let thepath = path.unwrap();
                println!("Name: {}", thepath.path().display());
                let token_list = super::lex(thepath.path());
                verify_stage_one_tokens(&token_list);
            }
        }

        fn verify_stage_one_tokens(tokenlist: &Vec<super::TokenInfo>) -> bool {
            let success_token_types = vec![
                super::TokenType::KeywordInt,
                super::TokenType::Identifier,
                super::TokenType::OpenParen,
                super::TokenType::CloseParen,
                super::TokenType::OpenBrace,
                super::TokenType::KeywordRet,
                super::TokenType::Intliteral,
                super::TokenType::SemiColon,
                super::TokenType::CloseBrace,
            ];

            for i in tokenlist.into_iter() {
                println!("{}:{}", &i.token_type, &i.data)
            }

            for x in 0..tokenlist.len() {
                let t = &tokenlist[x];
                let u = &t.token_type;
                assert_eq!(&success_token_types[x], u);
            }

            return true;
        }
    }
}
