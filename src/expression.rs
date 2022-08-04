use crate::token::Token;

pub enum Expr {
    Literal {
        val: Token,
    },
    Unary {
        op: Token,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
}

pub trait Visitor<T> {
    fn visit(&self, accepter: &mut Self) -> T;
}

pub trait Accepter<T> {
    fn accept(&mut self, visitor: impl Visitor<T>);
}

impl Accepter for Expr::Literal {
    fn accept(&mut self, visitor: impl Visitor<T>) {
        visitor.visit(&mut self);
    }
}
