use std::rc::Rc;

use crate::grammar::{Literal, RcStr, Token, TokenType};

pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<String>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner {
            source: input.as_bytes(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: Rc::from(""),
            literal: None,
            line: self.line,
        });
        std::mem::take(&mut self.tokens)
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LEFT_PAREN),
            b')' => self.add_token(TokenType::RIGHT_PAREN),
            b'{' => self.add_token(TokenType::LEFT_BRACE),
            b'}' => self.add_token(TokenType::RIGHT_BRACE),
            b',' => self.add_token(TokenType::COMMA),
            b'.' => self.add_token(TokenType::DOT),
            b'-' => self.add_token(TokenType::MINUS),
            b'+' => self.add_token(TokenType::PLUS),
            b';' => self.add_token(TokenType::SEMICOLON),
            b'*' => self.add_token(TokenType::STAR),
            b'=' | b'!' | b'<' | b'>' => self.handle_comparison(c),
            b'/' => self.handle_slash(),
            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,
            b'"' => self.handle_string(),
            c if c.is_ascii_digit() => self.handle_number(),
            c if c.is_ascii_alphabetic() || c == b'_' => self.handle_identifier(),
            _ => {
                self.report_error(format!(
                    "[line {}] Error: Unexpected character: {}.",
                    self.line, c as char
                ));
            }
        };
    }

    fn report_error(&mut self, message: String) {
        self.errors.push(message);
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.substr(self.start, self.current),
            literal,
            line: self.line,
        });
    }

    fn handle_comparison(&mut self, c: u8) {
        let (single_char_token, double_char_token) = match c {
            b'=' => (TokenType::EQUAL, TokenType::EQUAL_EQUAL),
            b'!' => (TokenType::BANG, TokenType::BANG_EQUAL),
            b'<' => (TokenType::LESS, TokenType::LESS_EQUAL),
            b'>' => (TokenType::GREATER, TokenType::GREATER_EQUAL),
            _ => unreachable!(),
        };
        if self.match_(b'=') {
            self.add_token(double_char_token);
        } else {
            self.add_token(single_char_token);
        }
    }

    fn handle_slash(&mut self) {
        if self.match_(b'/') {
            self.advance_end_of_line();
        } else {
            self.add_token(TokenType::SLASH);
        }
    }

    fn advance_end_of_line(&mut self) {
        while self.peek() != b'\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn handle_string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.report_error(format!("[line {}] Error: Unterminated string.", self.line));
            return;
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes
        let literal = self.substr(self.start + 1, self.current - 1);
        self.add_token_with_literal(TokenType::STRING, Some(Literal::String(literal)))
    }

    fn handle_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number_str = self.substr_str(self.start, self.current);
        let number: f64 = number_str.parse().unwrap();
        self.add_token_with_literal(TokenType::NUMBER, Some(Literal::Number(number)));
    }

    fn handle_identifier(&mut self) {
        while Self::is_identifier_char(self.peek()) {
            self.advance();
        }
        let text = self.substr_str(self.start, self.current);
        let token_type = TokenType::get_token_type(text);
        self.add_token(token_type)
    }

    fn is_identifier_char(c: u8) -> bool {
        c.is_ascii_alphanumeric() || c == b'_'
    }

    /// Returns an `Rc<str>` slice of the source — used for token lexemes and string literals.
    fn substr(&self, start: usize, end: usize) -> RcStr {
        Rc::from(self.substr_str(start, end))
    }

    /// Returns a `&str` slice of the source — used internally where Rc is not needed.
    fn substr_str(&self, start: usize, end: usize) -> &str {
        std::str::from_utf8(&self.source[start..end]).expect("valid UTF-8 source")
    }

    fn match_(&mut self, expected: u8) -> bool {
        let is_match = self.peek() == expected;
        if is_match {
            self.advance();
        }
        is_match
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}
