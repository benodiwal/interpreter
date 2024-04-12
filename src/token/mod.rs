use std::collections::HashMap;

pub type TokenType = String;

#[derive(Default, Debug)]
pub struct Token {
    pub Type: TokenType,
    pub Literal: String, 
}


pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT"; // add, foobar, x, y, ...
pub const INT: &str = "INT";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";
pub const LT: &str = "<";
pub const GT: &str = ">";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";

pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        
        map.insert("function", FUNCTION.to_string());
        map.insert("let", LET.to_string());
        map.insert("true", TRUE.to_string());
        map.insert("false", FALSE.to_string());
        map.insert("if", IF.to_string());
        map.insert("else", ELSE.to_string());
        map.insert("return", RETURN.to_string());
        
        map
    };
}

