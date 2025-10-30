#[derive(Debug, Clone)]
pub enum TokenType {
    TokenEof,
    TokenError,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub struct Scanner {
    source: Vec<u8>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn init_scanner(source: &str) -> Scanner {
        Scanner {
            source: source.as_bytes().to_vec(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        if self.is_at_end() {
            return self.make_token(TokenType::TokenEof);
        }
        self.advance();
        self.make_token(TokenType::TokenError)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: String::from_utf8(self.source.clone()).unwrap_or_default(),
            line: self.line,
        }
    }
}
