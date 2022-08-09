use crate::{expression::Expr, token::Token, token::Token::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn synchronize(&mut self) {
        _ = self.advance();

        while !self.is_at_end() {
            if matches!(self.previous().unwrap(), Semicolon { .. }) {
                return;
            }

            match self.peek().unwrap() {
                Class { .. } => return,
                Fun { .. } => return,
                For { .. } => return,
                If { .. } => return,
                Print { .. } => return,
                Return { .. } => return,
                Var { .. } => return,
                While { .. } => return,
                _ => _ = self.advance(),
            }
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while matches!(
            self.peek().unwrap(),
            BangEqual { line: _ } | EqualEqual { line: _ }
        ) {
            let op = self.advance().unwrap();
            let right = Box::new(self.comparison());
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right,
            };
        }
        expr
    }

    fn is_at_end(&self) -> bool {
        if self.current as usize >= self.tokens.len() {
            true
        } else {
            false
        }
    }

    fn previous(&self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            Some(self.tokens[(self.current - 1) as usize].clone())
        }
    }

    fn peek(&self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            Some(self.tokens[self.current as usize].clone())
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            self.current += 1;
            self.previous()
        }
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while matches!(
            self.peek().unwrap(),
            Greater { .. } | GreaterEqual { .. } | Less { .. } | LessEqual { .. }
        ) {
            let op = self.advance().unwrap();
            let right = Box::new(self.term());
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right,
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while matches!(self.peek().unwrap(), Minus { .. } | Plus { .. }) {
            let op = self.advance().unwrap();
            let right = Box::new(self.factor());
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right,
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while matches!(self.peek().unwrap(), Slash { .. } | Star { .. }) {
            let op = self.advance().unwrap();
            let right = Box::new(self.unary());
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right,
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if matches!(self.peek().unwrap(), Bang { .. } | Minus { .. }) {
            let op = self.advance().unwrap();
            let expr = Box::new(self.unary());
            return Expr::Unary { op, expr };
        }

        self.primary().unwrap()
    }

    fn primary(&mut self) -> Option<Expr> {
        if matches!(
            self.peek().unwrap(),
            False { .. } | True { .. } | Nil { .. } | Number { .. } | String { .. }
        ) {
            return Some(Expr::Literal {
                val: self.advance().unwrap(),
            });
        }

        if matches!(self.peek().unwrap(), LeftParen { .. }) {
            let expr = self.expression();
            if !matches!(self.advance().unwrap(), RightParen { .. }) {
                panic!("Expected \"(\"");
            }
            return Some(Expr::Grouping {
                expr: Box::new(expr),
            });
        }

        None
    }
}
