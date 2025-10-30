#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenComma, TokenDot, TokenSemicolon,
    TokenMinus, TokenPlus, TokenSlash, TokenStar,
    TokenNot, TokenNotEqual,
    TokenEqual, TokenEqualEqual,
    TokenLess, TokenLessEqual,
    TokenGreater, TokenGreaterEqual,
    TokenIdentifier, TokenString, TokenNumber,
    TokenTrue, TokenFalse,
    TokenAnd, TokenOr,
    TokenIf, TokenElse,
    TokenClass, TokenSuper, TokenThis,
    TokenFun, TokenVar,
    TokenReturn,
    TokenFor, TokenWhile,
    TokenNil,
    TokenPrint,
    TokenError,
    TokenEof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Vec<u8>,
    pub line: usize,
}

pub struct Scanner {
    source: Vec<u8>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn init_scanner(source: &str) -> Self {
        Scanner {
            source: source.as_bytes().to_vec(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() { 0 } else { self.source[self.current] }
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() { 0 } else { self.source[self.current + 1] }
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() as char {
                ' ' | '\r' | '\t' => { self.advance(); },
                '\n' => { self.line += 1; self.advance(); },
                '/' => {
                    if self.peek_next() as char == '/' {
                        while self.peek() != b'\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else { return; }
                }
                _ => return,
            }
        }
    }

    fn get_literal_string(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string literal");
        }

        // Skip the closing quote
        self.advance();
        self.make_token(TokenType::TokenString)
    }

    fn get_literal_number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::TokenNumber)
    }

    fn get_identifier(&mut self) -> Token {
        while self.peek().is_ascii_alphanumeric() || self.peek() == b'_' {
            self.advance();
        }
        self.make_token(TokenType::TokenIdentifier)
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            value: self.source[self.start..self.current].to_vec(),
            line: self.line,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::TokenError,
            value: message.as_bytes().to_vec(),
            line: self.line,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::TokenEof);
        }

        let c = self.advance();

        if (c as char).is_ascii_alphabetic() || c == b'_' {
            return self.get_identifier();
        }
        if (c as char).is_ascii_digit() {
            return self.get_literal_number();
        }

        match c as char {
            '(' => self.make_token(TokenType::TokenLeftParen),
            ')' => self.make_token(TokenType::TokenRightParen),
            '{' => self.make_token(TokenType::TokenLeftBrace),
            '}' => self.make_token(TokenType::TokenRightBrace),
            ';' => self.make_token(TokenType::TokenSemicolon),
            ',' => self.make_token(TokenType::TokenComma),
            '.' => self.make_token(TokenType::TokenDot),
            '-' => self.make_token(TokenType::TokenMinus),
            '+' => self.make_token(TokenType::TokenPlus),
            '/' => self.make_token(TokenType::TokenSlash),
            '*' => self.make_token(TokenType::TokenStar),
            '!' => {
                if self.match_next(b'=') { self.make_token(TokenType::TokenNotEqual) }
                else { self.make_token(TokenType::TokenNot) }
            },
            '=' => {
                if self.match_next(b'=') { self.make_token(TokenType::TokenEqualEqual) }
                else { self.make_token(TokenType::TokenEqual) }
            },
            '<' => {
                if self.match_next(b'=') { self.make_token(TokenType::TokenLessEqual) }
                else { self.make_token(TokenType::TokenLess) }
            },
            '>' => {
                if self.match_next(b'=') { self.make_token(TokenType::TokenGreaterEqual) }
                else { self.make_token(TokenType::TokenGreater) }
            },
            '"' => self.get_literal_string(),
            _ => self.error_token("Unknown character."),
        }
    }
}
