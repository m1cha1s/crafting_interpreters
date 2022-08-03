use crate::exceptions::Exce;

use super::token::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Exce> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), Exce> {
        use Token::*;

        let c = self.advance();

        match c {
            '(' => self.tokens.push(LeftParen { line: self.line }),
            _ => {
                return Err(Exce::UnknownChar {
                    line: self.line,
                    column: self.current as u64,
                })
            }
        }

        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
