use std::io;
use std::io::Write;
use crate::Lexer;

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        print!("{}", PROMPT);
        stdout.flush().expect("Failed to flush stdout");

        buffer.clear();
        stdin.read_line(&mut buffer).expect("Failed to read line");

        let mut lexer = Lexer::new(&buffer);
        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            if tok.Type == "EOF" {
                break;
            }
        }
    }
}
