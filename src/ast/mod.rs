use crate::Token;

// Trait Node
pub trait Node {
    fn token_literal(&self) -> String;
}

// Statement Node
pub trait Statement: Node {
    fn statement_node(&self);
}

// Expression Node
pub trait Expression: Node {
    fn expression_node(&self);
}

// Program Struct
#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty()  {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

// LetStatement struct
pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.Literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {
        todo!()
    }
}

// Identifier struct
pub struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.Literal.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}