use crate::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[allow(unused)]
pub trait Statement: Node {
    fn statement_node(&self);
}

#[allow(unused)]
pub trait Expression: Node {
    fn expression_node(&self);
}

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

pub struct LetStatement {
    pub token: Token,
    #[allow(unused)]
    pub name: Option<Identifier>,
    #[allow(unused)]
    pub value: Option<Box<dyn Expression>>,
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

pub struct Identifier {
    pub token: Token,
    #[allow(unused)]
    pub value: String,
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

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}
