// src/scanner.rs
// A simple scanner for a small subset of Lox tokens required by the assignments.

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single-character tokens
    TokenLeftParen,
    TokenRightParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenSemicolon,
    TokenMinus,
    TokenPlus,
    TokenSlash,
    TokenStar,

    // One or two character tokens
    TokenNot,
    TokenNotEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenLess,
    TokenLessEqual,
    TokenGreater,
    TokenGreaterEqual,

    // Literals
    TokenIdentifier,
    TokenString,
    TokenNumber,

    // Keywords
    TokenTrue,
    TokenFalse,
    TokenAnd,
    TokenOr,
    TokenIf,
    TokenElse,
    TokenClass,
    TokenSuper,
    TokenThis,
    TokenFun,
    TokenVar,
    TokenReturn,
    TokenFor,
    TokenWhile,
    TokenNil,
    TokenPrint,

    TokenError,
    TokenEof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Vec<u8>, // raw bytes of lexeme
    pub length: usize,
    pub line: usize,
}

pub struct Scanner {
    source: Vec<u8>,
    start: usize,
    current: usize,
    pub line: usize,
}

impl Scanner {
    pub fn init_scanner(source: &str) -> Self {
        Self {
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
        if self.is_at_end() {
            0
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            0
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let slice = &self.source[self.start..self.current];
        Token {
            token_type,
            value: slice.to_vec(),
            length: slice.len(),
            line: self.line,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::TokenError,
            value: message.as_bytes().to_vec(),
            length: message.len(),
            line: self.line,
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            let c = self.peek();
            match c {
                b' ' | b'\r' | b'\t' => { self.advance(); }
                b'\n' => { self.line += 1; self.advance(); }
                b'/' => {
                    if self.peek_next() == b'/' {
                        // comment to end of line
                        while self.peek() != b'\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn is_digit(c: u8) -> bool { c.is_ascii_digit() }
    fn is_alpha(c: u8) -> bool { c.is_ascii_alphabetic() || c == b'_' }

    fn string(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }
        // consume the closing "
        self.advance();
        self.make_token(TokenType::TokenString)
    }

    fn number(&mut self) -> Token {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == b'.' && Self::is_digit(self.peek_next()) {
            // consume dot
            self.advance();
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }
        self.make_token(TokenType::TokenNumber)
    }

    fn identifier(&mut self) -> Token {
        while Self::is_alpha(self.peek()) || Self::is_digit(self.peek()) {
            self.advance();
        }

        let text = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();

        let token_type = match text.as_str() {
            "true" => TokenType::TokenTrue,
            "false" => TokenType::TokenFalse,
            "and" => TokenType::TokenAnd,
            "or" => TokenType::TokenOr,
            "if" => TokenType::TokenIf,
            "else" => TokenType::TokenElse,
            "class" => TokenType::TokenClass,
            "super" => TokenType::TokenSuper,
            "this" => TokenType::TokenThis,
            "fun" => TokenType::TokenFun,
            "var" => TokenType::TokenVar,
            "return" => TokenType::TokenReturn,
            "for" => TokenType::TokenFor,
            "while" => TokenType::TokenWhile,
            "nil" => TokenType::TokenNil,
            "print" => TokenType::TokenPrint,
            _ => TokenType::TokenIdentifier,
        };

        self.make_token(token_type)
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::TokenEof);
        }

        let c = self.advance();
        // single and double character tokens
        match c {
            b'(' => return self.make_token(TokenType::TokenLeftParen),
            b')' => return self.make_token(TokenType::TokenRightParen),
            b'{' => return self.make_token(TokenType::TokenLeftBrace),
            b'}' => return self.make_token(TokenType::TokenRightBrace),
            b',' => return self.make_token(TokenType::TokenComma),
            b'.' => return self.make_token(TokenType::TokenDot),
            b';' => return self.make_token(TokenType::TokenSemicolon),
            b'-' => return self.make_token(TokenType::TokenMinus),
            b'+' => return self.make_token(TokenType::TokenPlus),
            b'*' => return self.make_token(TokenType::TokenStar),
            b'/' => return self.make_token(TokenType::TokenSlash),
            b'!' => {
                if self.match_next(b'=') { return self.make_token(TokenType::TokenNotEqual); }
                return self.make_token(TokenType::TokenNot);
            }
            b'=' => {
                if self.match_next(b'=') { return self.make_token(TokenType::TokenEqualEqual); }
                return self.make_token(TokenType::TokenEqual);
            }
            b'<' => {
                if self.match_next(b'=') { return self.make_token(TokenType::TokenLessEqual); }
                return self.make_token(TokenType::TokenLess);
            }
            b'>' => {
                if self.match_next(b'=') { return self.make_token(TokenType::TokenGreaterEqual); }
                return self.make_token(TokenType::TokenGreater);
            }
            b'"' => return self.string(),
            _ => {}
        }

        if Self::is_digit(c) {
            return self.number();
        }
        if Self::is_alpha(c) {
            return self.identifier();
        }

        self.error_token("Unknown character.")
    }
}
