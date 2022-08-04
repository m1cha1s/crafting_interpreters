use crate::exceptions::Exce;

use super::token::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
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

        self.tokens.push(Token::Eof { line: self.line });
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), Exce> {
        use Token::*;

        let line = self.line;
        let c = self.advance();

        match c {
            '(' => self.tokens.push(LeftParen { line }),
            ')' => self.tokens.push(RightParen { line }),
            '{' => self.tokens.push(LeftBrace { line }),
            '}' => self.tokens.push(RightBrace { line }),
            ',' => self.tokens.push(Comma { line }),
            '.' => self.tokens.push(Dot { line }),
            '-' => self.tokens.push(Minus { line }),
            '+' => self.tokens.push(Plus { line }),
            ';' => self.tokens.push(Semicolon { line }),
            '*' => self.tokens.push(Star { line }),
            '!' => {
                if self.match_char('=') {
                    self.tokens.push(BangEqual { line });
                } else {
                    self.tokens.push(Bang { line });
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.tokens.push(EqualEqual { line });
                } else {
                    self.tokens.push(Equal { line });
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.tokens.push(LessEqual { line });
                } else {
                    self.tokens.push(Less { line });
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.tokens.push(GreaterEqual { line });
                } else {
                    self.tokens.push(Greater { line });
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        let _ = self.advance();
                    }
                } else {
                    self.tokens.push(Slash { line });
                }
            }
            '"' => self.string()?,
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => {
                return Err(Exce::UnexpectedChar {
                    line: self.line,
                    column: self.current as u32,
                })
            }
        }

        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) -> Result<(), Exce> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            let _ = self.advance();
        }

        if self.is_at_end() {
            return Err(Exce::UnterminatedString {
                line: self.line,
                column: self.current as u32,
            });
        }

        let _ = self.advance();

        self.tokens.push(Token::String {
            line: self.line,
            value: self.source.as_str()[(self.start + 1)..self.current].to_string(),
        });

        Ok(())
    }
}
