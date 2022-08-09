use crate::{token::Token, expression::Expr};

pub struct Parser {
    tokens: Vec<Token>,
    current: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    
    fn expression(&mut self) -> Expr {
        self.equality()
    }
    
    fn equality(&mut self) -> Expr {
        let expr = self.comparison();
        
        while matches!()
    }
    
    // TODO
    fn comparison(&mut self) -> Expr {
        
    }
}
