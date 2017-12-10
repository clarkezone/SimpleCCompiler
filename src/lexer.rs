mod lexer {
    use std::path::Path;
    use fileutils::lines_from_file;
    use regex::Regex;

    #[derive(Clone)]
    enum TokenType {
        CloseBrace,
        OpenBrace,
    }

    struct TokenInfo {
        token_type: TokenType,
        source_line: u32,
        start_char: u32,
        end_char: u32,
    }

    struct TokenExtractor {
        regrx_runner: Regex,
        token_type: TokenType,
    }

    impl TokenExtractor {
        //constructor takes a token and a token collector
        pub fn new(regex: &str, tok: TokenType) -> TokenExtractor {
            TokenExtractor {
                regrx_runner: Regex::new(regex).unwrap(),
                token_type: tok,
            }
        }

        pub fn get_token(&self, line: &String, line_num: u32, collector: &mut Vec<TokenInfo>) {
            //find all matches using the regex of the token passed in
            for mat2 in self.regrx_runner.find_iter(line) {
                println!(
                    "Found token {} at {} {}",
                    self.token_type.clone() as i32,
                    mat2.start(),
                    mat2.end()
                );

                //foreach, create a token info
                let new_token_info = TokenInfo {
                    token_type: self.token_type.clone(),
                    source_line: line_num,
                    start_char: mat2.start() as u32,
                    end_char: mat2.end() as u32,
                };

                //Add token to token collector
                collector.push(new_token_info);
            }
        }
    }

    fn lex<P>(filename: P)
    where
        P: AsRef<Path>,
    {
        let lines = lines_from_file(filename);
        let mut collector: Vec<TokenInfo> = Vec::new();
    }

    fn lexinternal(lines: Vec<String>, collector: &mut Vec<TokenInfo>) {
        // init all tokenextractors
        let extractors = init_extractors();

        let mut line_num: u32 = 1;
        for line in lines {
            println!("{} {:?}", line_num, line);
            let tokens = get_tokens(&line, line_num, &extractors, collector);
            line_num = line_num + 1;
        }
    }

    fn init_extractors() -> Vec<TokenExtractor> {
        let mut te: Vec<TokenExtractor> = Vec::new();

        te.push(TokenExtractor::new(r"\{{1}?", TokenType::OpenBrace));
        te.push(TokenExtractor::new(r"\}{1}?", TokenType::CloseBrace));
        return te;
    }

    fn get_tokens(
        line: &String,
        num: u32,
        extractors: &Vec<TokenExtractor>,
        collector: &mut Vec<TokenInfo>,
    ) {
        for e in extractors.into_iter() {
            e.get_token(line, num, collector);
        }
    }

    mod tests {
        #[test]
        fn test_token_extract() {
            let lines = vec![String::from(r"foo{"), String::from(r"shebar}ss{")];

            let mut collector: Vec<super::TokenInfo> = Vec::new();
            super::lexinternal(lines, &mut collector);

            assert_eq!(collector.len(), 3);
        }

        #[test]
        fn test_regex() {
            use regex::Regex;

            let re = Regex::new(r"\{{1}?").unwrap();
            for mat2 in re.find_iter(r"{sss{") {
                println!("Found {} {}", mat2.start(), mat2.end());
            }
        }
    }
}
