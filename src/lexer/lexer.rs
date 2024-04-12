use crate::{Token, TokenType, ASSIGN, ASTERISK, BANG, COMMA, EOF, EQ, GT, IDENT, ILLEGAL, INT, KEYWORDS, LBRACE, LPAREN, LT, MINUS, NOT_EQ, PLUS, RBRACE, RPAREN, SEMICOLON, SLASH};

pub struct Lexer {
    pub input: String,
    pub position: u64, // current position in input (points to current char)
    pub read_position: u64, // current reading position in input (after current char)
    pub ch: u8, // current char under examination
}


// Adding constructor for Lexer
impl Lexer {
    pub fn new(input: &str) -> Self {
        let input_string = input.to_string();
        
        let mut lexer = Self {
            input: input_string,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();
        lexer
    }
}

// Implementing other functions for the Lexer
impl Lexer {

    pub fn read_char(&mut self) {
        let length_of_input = self.input.len() as u64;

        if self.read_position >= length_of_input {
            self.ch = 0;
        } else {
            self.ch = self.input.chars().nth(self.read_position as usize).unwrap() as u8;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = Token::default();

        self.skip_whitespace();

        match self.ch as char {
            '=' => {
                if self.peek_char() as char == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = Token {
                        Type: EQ.to_string(),
                        Literal: format!("{}{}", ch, self.ch),
                    }
                } else {
                    tok = new_token(ASSIGN.to_string(), self.ch);
                }
            },
            ';' => tok = new_token(SEMICOLON.to_string(), self.ch),
            '(' => tok = new_token(LPAREN.to_string(), self.ch),
            ')' => tok = new_token(RPAREN.to_string(), self.ch),
            ',' => tok = new_token(COMMA.to_string(), self.ch),
            '+' => tok = new_token(PLUS.to_string(), self.ch),
            '-' => tok = new_token(MINUS.to_string(), self.ch),
            '!' => {
                if self.peek_char() as char == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = Token {
                        Type: NOT_EQ.to_string(),
                        Literal: format!("{}{}", ch, self.ch),
                    }
                } else {
                    tok = new_token(BANG.to_string(), self.ch);
                }
            },
            '/' => tok = new_token(SLASH.to_string(), self.ch),
            '*' => tok = new_token(ASTERISK.to_string(), self.ch),
            '>' => tok = new_token(GT.to_string(), self.ch),
            '<' => tok = new_token(LT.to_string(), self.ch),
            '{' => tok = new_token(LBRACE.to_string(), self.ch),
            '}' => tok = new_token(RBRACE.to_string(), self.ch),
            _ => {
                if is_letter(self.ch as char) {
                    tok.Literal = self.read_identifier().to_string();
                    tok.Type = lookup_ident(&tok.Literal);
                    return tok;
                } else if is_digit(self.ch as char) {
                    tok.Type = INT.to_string();
                    tok.Literal = self.read_number().to_string();                
                    return tok;
                } else {
                    tok = new_token(ILLEGAL.to_string(), self.ch);
                }
            }
        }

        if self.ch == 0 {
            tok.Literal = "".to_string();
            tok.Type = EOF.to_string();
        }
        
        self.read_char();

        tok
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position as usize;
        while is_letter(self.ch as char) {
            self.read_char();
        }
        let final_position = self.position as usize;
        &self.input[position..final_position]
    }

    fn skip_whitespace(&mut self) {
        while self.ch as char == ' ' || self.ch as char == '\t' || self.ch as char == '\n' || self.ch as char == '\r' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> &str {
        let position = self.position as usize;
        while is_digit(self.ch as char) {
            self.read_char();
        }
        let final_position = self.position as usize;
        &self.input[position..final_position]
    }


    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() as u64 {
            0
        } else {
            self.input.chars().nth(self.read_position as usize).unwrap() as u8
        }
    }

}

fn new_token(token_type: TokenType, ch: u8) -> Token {
    Token {
        Type: token_type,
        Literal: (ch as char).to_string(),
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn lookup_ident(ident: &str) -> TokenType {
    if let Some(tok) = KEYWORDS.get(ident) {
        return tok.clone();        
    }
    IDENT.to_string()
}