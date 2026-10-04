use crate::token::{keyword, Token, TokenType};

/// Cut `source` into tokens. Returns everything it managed to scan alongside every
/// error it found; the caller decides whether to go on.
pub fn scan(source: &str) -> (Vec<Token>, Vec<String>) {
    let mut s = Scanner {
        src: source.chars().collect(),
        start: 0,
        current: 0,
        line: 1,
        tokens: Vec::new(),
        errors: Vec::new(),
    };
    s.run();
    (s.tokens, s.errors)
}

struct Scanner {
    src: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: Vec<String>,
}

impl Scanner {
    fn run(&mut self) {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let eof_line = match self.tokens.last() {
            Some(last_token) => last_token.line,
            None => 1,
        };

        self.tokens.push(Token {
            kind: TokenType::Eof,
            lexeme: String::new(),
            line: eof_line,
        });
    }


    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            // Single-character tokens
            '(' => self.add(TokenType::LParen),
            ')' => self.add(TokenType::RParen),
            '{' => self.add(TokenType::LBrace),
            '}' => self.add(TokenType::RBrace),
            ',' => self.add(TokenType::Comma),
            ';' => self.add(TokenType::Semicolon),
            '+' => self.add(TokenType::Plus),
            '-' => self.add(TokenType::Minus),
            '*' => self.add(TokenType::Star),

            // One- or two-character operators
            '!' => {
                let kind = if self.matches('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add(kind);
            }
            '=' => {
                let kind = if self.matches('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add(kind);
            }
            '<' => {
                let kind = if self.matches('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add(kind);
            }
            '>' => {
                let kind = if self.matches('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add(kind);
            }

            // Division or line comment
            '/' => {
                if self.matches('/') {
                    // A comment goes until the end of the line
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add(TokenType::Slash);
                }
            }

            // Whitespace (ignored)
            ' ' | '\r' | '\t' => {}

            // Newline increments line counter
            '\n' => self.line += 1,

            // Literals
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

            // Anything else is invalid
            _ => self.error(self.line, "Character is not part of any token."),
        }
    }


    fn string(&mut self) {
        // TODO(you): scan a string literal. A string may span lines (1.5); an unterminated one
        //            is reported at the line it opened on (5.1).
        todo!("string")
    }

    fn number(&mut self) {
        // 1. Consume digits in the integer part
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // 2. Look for fractional part with two-character lookahead
        //    (Section 1.4 & Slide 54: only consume '.' if followed by a digit)
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the '.'
            self.advance();

            // 3. Consume digits in the fractional part
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // 4. Emit the NUMBER token
        self.add(TokenType::Number);
    }


    fn identifier(&mut self) {
        while matches!(self.peek(), 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
            self.advance();
        }

        let word: String = self.src[self.start..self.current].iter().collect();
        self.add(keyword(&word).unwrap_or(TokenType::Identifier));
    }

    // --- primitives ---------------------------------------------------------------

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn advance(&mut self) -> char {
        let c = self.src[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() || self.src[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.src[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            '\0'
        } else {
            self.src[self.current + 1]
        }
    }

    fn add(&mut self, kind: TokenType) {
        self.tokens.push(Token {
            kind,
            lexeme: self.src[self.start..self.current].iter().collect(),
            line: self.line,
        });
    }

    fn error(&mut self, line: usize, message: &str) {
        self.errors.push(format!("[line {}] Error: {}", line, message));
    }
}
