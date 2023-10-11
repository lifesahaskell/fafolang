use core::fmt;

#[derive(Clone)]
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Self {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lex.read_char();
        lex
    }

    // pub fn next_token(&mut self) -> Result<Token, ()> {
    //     self.skip_whitespace();

    //     let token = match self.ch {
    //         b'{' => Token::LBrace,
    //         b'}' => Token::RBrace,
    //         b'(' => Token::LParen,
    //         b')' => Token::RParen,
    //         b',' => Token::Comma,
    //         b';' => Token::Semicolon,
    //         b'+' => Token::Plus,
    //         b'-' => Token::Minus,
    //         b'!' => {
    //             if self.peek() == b'=' {
    //                 self.read_char();
    //                 Token::NotEqual
    //             } else {
    //                 Token::Bang
    //             }
    //         }
    //         b'<' => Token::LessThan,
    //         b'>' => Token::GreaterThan,
    //         b'*' => Token::Asterisk,
    //         b'/' => Token::ForwardSlash,
    //         b'=' => {
    //             if self.peek() == b'=' {
    //                 self.read_char();
    //                 Token::Equal
    //             } else {
    //                 Token::Assign
    //             }
    //         }
    //         b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
    //             let ident = self.read_ident();
    //             return Ok(match ident.as_str() {
    //                 "fn" => Token::Function,
    //                 "let" => Token::Let,
    //                 "return" => Token::Return,
    //                 _ => Token::Ident(ident),
    //             });
    //         }
    //         b'0'..=b'9' => return Ok(Token::Int(self.read_int())),
    //         0 => Token::Eof,
    //         _ => unreachable!("unable to lex symbol."),
    //     };

    //     self.read_char();
    //     Ok(token)
    // }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position]
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let token = match self.ch {
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            b'*' => Token::Asterisk,
            b'/' => Token::ForwardSlash,
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Some(match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "return" => Token::Return,
                    _ => Token::Ident(ident),
                });
            }
            b'0'..=b'9' => return Some(Token::Int(self.read_int())),
            0 => Token::Eof,
            _ => unreachable!("unable to lex symbol."),
        };

        self.read_char();
        Some(token)
    }
}


#[allow(dead_code)]
#[derive(Debug, Default, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Eof,
    #[default]
    Illegal,

    Str(String),
    Int(String),

    Assign,
    Bang,
    Plus,
    Minus,
    ForwardSlash,
    Asterisk,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,

    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    Return,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Ident(x) => write!(f, "Ident({x})"),
            Eof => write!(f, "EOF"),
            Illegal => write!(f, "ILLEGAL"),
            Str(x) => write!(f, "String({x})"),
            Int(x) => write!(f, "Int({x})"),
            Assign => write!(f, "="),
            Bang => write!(f, "!"),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            ForwardSlash => write!(f, "/"),
            Asterisk => write!(f, "*"),
            Equal => write!(f, "=="),
            NotEqual => write!(f, "!="),
            LessThan => write!(f, "<"),
            GreaterThan => write!(f, ">"),
            Comma => write!(f, ","),
            Semicolon => write!(f, "Semicolon"),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),
            Function => write!(f, "function"),
            Let => write!(f, "let"),
            Return => write!(f, "return"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn get_next_token() {
        let input = r#"let five = 5;
        let ten = 10;
        
        let add = fn(x, y){
            x + y;
        };
        
        let result = add(five, ten);
        
        foobar;"#;
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Ident("foobar".to_string()),
            Token::Semicolon,
            Token::Eof,
        ];

        for token in tokens {
            let cur_token = lexer.next().unwrap();
            println!("expected {:?}, received {:?}", token, cur_token);
            assert_eq!(token, cur_token);
        }
    }
}
