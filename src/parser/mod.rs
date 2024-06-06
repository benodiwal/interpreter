use crate::{ast::{Identifier, LetStatement, Program, Statement}, Lexer, Token, TokenType, ASSIGN, EOF, IDENT, LET, SEMICOLON};

pub struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
}
    
impl<'a> Parser<'a> {
    fn new(l: &'a mut Lexer) -> Self {
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

impl<'a> Parser<'a> {
    
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
          statements: Vec::new(),  
        };

        while self.cur_token.Type != EOF {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        Some(program)
    }   

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.Type.as_str() {
            LET => Some(Box::new(self.parse_let_statement().unwrap())),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {

        let cur_token = self.cur_token.clone();

        let mut stmt = LetStatement {
            token: cur_token.clone(),
            name: None,
            value: None,
        };

        if !self.expect_peek(IDENT.to_string()) {
            return None;
        }

        stmt.name = Some(Identifier {
            token: cur_token.clone(),
            value: cur_token.Literal,
        });

        if !self.expect_peek(ASSIGN.to_string()) {
            return None;
        }

        while !self.expect_peek(SEMICOLON.to_string()) {
            self.next_token()
        }

        Some(stmt)

    }

    fn current_token_is(&self, t: TokenType) -> bool {
        self.cur_token.Type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.Type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }

}
