mod token;
mod lexer;
mod repl;

mod utils;

mod ast;
mod parser;

use token::*;
use lexer::*;
use utils::print_welcome_message;

#[macro_use]
extern crate lazy_static;

fn main() {
    print_welcome_message();
    repl::start();
}
