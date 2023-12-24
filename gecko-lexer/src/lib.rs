pub mod ttype;
pub mod token;

use std::collections::HashMap;

use gecko_error::{Error, LineInfo};

pub struct Lexer {
    pub input: String,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<token::Token>,
    keywords: HashMap<String, ttype::TType>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            keywords: {
                let mut keywords = HashMap::new();
                keywords.insert(String::from("true"), ttype::TType::TRUE);
                keywords.insert(String::from("false"), ttype::TType::FALSE);
                keywords.insert(String::from("if"), ttype::TType::IF);
                keywords.insert(String::from("else"), ttype::TType::ELSE);
                keywords.insert(String::from("while"), ttype::TType::WHILE);
                keywords.insert(String::from("for"), ttype::TType::FOR);
                keywords.insert(String::from("in"), ttype::TType::IN);
                keywords.insert(String::from("fn"), ttype::TType::FN);
                keywords.insert(String::from("let"), ttype::TType::LET);
                keywords.insert(String::from("return"), ttype::TType::RETURN);
                keywords.insert(String::from("import"), ttype::TType::IMPORT);
                keywords
            },
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<token::Token>, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(token::Token::new(
            ttype::TType::EOF,
            String::from(""),
            LineInfo::new(self.line, self.start, self.current),
        ));

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();

        match c {
            '(' => self.add_token(ttype::TType::LPAREN),
            ')' => self.add_token(ttype::TType::RPAREN),
            '{' => self.add_token(ttype::TType::LBRACE),
            '}' => self.add_token(ttype::TType::RBRACE),
            ',' => self.add_token(ttype::TType::COMMA),
            '.' => self.add_token(ttype::TType::DOT),
            '-' => self.add_token(ttype::TType::MINUS),
            '+' => self.add_token(ttype::TType::PLUS),
            ';' => self.add_token(ttype::TType::SEMICOLON),
            '*' => self.add_token(ttype::TType::STAR),
            '!' => {
                let ttype = if self.match_char('=') {
                    ttype::TType::BANGEQ
                } else {
                    ttype::TType::BANG
                };
                self.add_token(ttype);
            }
            '=' => {
                let ttype = if self.match_char('=') {
                    ttype::TType::EQEQ
                } else {
                    ttype::TType::EQ
                };
                self.add_token(ttype);
            },
            '<' => {
                let ttype = if self.match_char('=') {
                    ttype::TType::LTEQ
                } else {
                    ttype::TType::LT
                };
                self.add_token(ttype);
            },
            '>' => {
                let ttype = if self.match_char('=') {
                    ttype::TType::GTEQ
                } else {
                    ttype::TType::GT
                };
                self.add_token(ttype);
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(ttype::TType::SLASH);
                }
            },
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }

                if self.is_at_end() {
                    return Err(Error::new(
                        gecko_error::LineInfo::new(self.line, self.start, self.current),
                        String::from("Unterminated string"),
                    ));
                }

                self.advance();

                let value = self.input[self.start + 1..self.current - 1].to_string();
                self.add_token(ttype::TType::String(value));
            },
            _ => {
                if Lexer::is_digit(c) {
                    while Lexer::is_digit(self.peek()) {
                        self.advance();
                    }

                    if self.peek() == '.' && Lexer::is_digit(self.peek_next()) {
                        self.advance();

                        while Lexer::is_digit(self.peek()) {
                            self.advance();
                        }
                    }

                    let lexeme = self.input[self.start..self.current].to_string();
                    let value = lexeme.parse::<f64>().unwrap();
                    self.add_token(ttype::TType::Number(value));
                } else if Lexer::is_alpha(c) {
                    while Lexer::is_alphanumeric(self.peek()) {
                        self.advance();
                    }

                    let lexeme = self.input[self.start..self.current].to_string();
                    let ttype = match self.keywords.get(&lexeme) {
                        Some(ttype) => ttype.clone(),
                        None => ttype::TType::Identifier(lexeme),
                    };
                    self.add_token(ttype);
                } else {
                    return Err(Error::new(
                        gecko_error::LineInfo::new(self.line, self.start, self.current),
                        format!("Unexpected character: {}", c),
                    ));
                }
            }
        }

        Ok(())
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.current + 1).unwrap()
        }
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        // include emojis
        Lexer::is_alpha(c) || Lexer::is_digit(c) || c.is_ascii_punctuation()
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input.chars().nth(self.current).unwrap()
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.input.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token(&mut self, ttype: ttype::TType) {
        let text = self.input[self.start..self.current].to_string();
        self.tokens
            .push(token::Token::new(ttype, text, LineInfo::new(self.line, self.start, self.current)));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.input.chars().nth(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
}
