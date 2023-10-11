use crate::lexer::{self, Token};
use std::io::stdin;

pub fn start() {
    let mut scanned_line = String::new();

    loop {
        println!(">>");
        if stdin().read_line(&mut scanned_line).is_ok() {
            let mut lexer = lexer::Lexer::new(scanned_line.to_owned()).into_iter();
            let mut token = lexer.next().unwrap();

            while token != Token::Eof {
                println!("{:?}", token);
                token = lexer.next().unwrap();
            }
        }
    }
}
