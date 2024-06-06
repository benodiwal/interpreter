use crate::{ast::{Program, Statement}, Lexer, Token, EOF, LET};

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
}
    
impl Parser {
    fn new(l: Lexer) -> Self {
        let mut parser = Self {
            l,
            cur_token: Token::default(),
            peek_token: Token::default(),
        };
        
        parser.next_token();
        parser.next_token();
        
        parser
    }
}


impl Parser {
    
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_program(&self) -> Option<Program> {
        let program = Program::default();

        // while self.cur_token.Type != EOF {
        //     todo!()
        // }

        None
    }   

    fn parse_statement(&self) -> Option<Box<dyn Statement>> {
        // match self.cur_token.Type {
        // }
        todo!()
    }

}
