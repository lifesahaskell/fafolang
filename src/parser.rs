use crate::{
    ast::*,
    lexer::{Lexer, Token},
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

#[allow(dead_code)]
impl Parser {
    fn new(lexer: Lexer) -> Parser {
        let mut parser = Self {
            lexer,
            cur_token: Token::Eof,
            peek_token: Token::Eof,
            errors: vec![],
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next().unwrap();
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::default();

        while self.cur_token != Token::Eof {
            if let Some(stmt) = self.parse_statement(self.cur_token.clone()) {
                program.statements.push(stmt);
            }
            self.next_token(); //consume semicolon
        }
        Some(program)
    }

    fn parse_statement(&mut self, cur_token: Token) -> Option<Statement> {
        match cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            Token::Ident(_value) => self.parse_expression_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let ident = if let Token::Ident(name) = &self.cur_token {
            Identifier {
                name: name.to_string(),
            }
        } else {
            self.errors.push(format!(
                "expected next token to be IDENT, got {:?}",
                self.cur_token
            ));
            return None;
        };

        self.next_token();
        if self.cur_token != Token::Assign {
            self.errors.push(format!(
                "expected next token to be '=', got {:?}",
                self.cur_token
            ));
            return None;
        };

        self.next_token();
        let expr = match &self.cur_token {
            Token::Int(value) => Expression::Literal(Literal {
                value: value.to_string(),
            }),
            Token::Ident(name) => Expression::Identifier(Identifier {
                name: name.to_string(),
            }),

            //todo!("add more featureful expression handling")
            _ => {
                self.errors.push(format!(
                    "expected next token to be INT or IDENT, got {:?}",
                    self.cur_token
                ));
                return None;
            }
        };

        Some(Statement::Let(LetStatement { ident, value: expr }))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let expr = match &self.cur_token {
            Token::Int(value) => Expression::Literal(Literal {
                value: value.to_string(),
            }),
            Token::Ident(name) => Expression::Identifier(Identifier {
                name: name.to_string(),
            }),

            //todo!("add more featureful expression handling")
            _ => {
                self.errors.push(format!(
                    "expected next token to be INT or IDENT, got {:?}",
                    self.cur_token
                ));
                return None;
            }
        };

        Some(Statement::Return(ReturnStatement { value: expr }))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        //self.next_token();

        let stmt = Statement::Expression(
            match &self.cur_token {
                Token::Int(_value) => {
                    self.next_token();
                    match &self.cur_token {
                        Token::LessThan => todo!(),
                        Token::GreaterThan => todo!(),
                        Token::Plus => todo!(),
                        Token::Minus => todo!(),
                        Token::Asterisk => todo!(),
                        Token::ForwardSlash => todo!(),
                        _ => {
                            self.errors.push(format!(
                                "expected next token to be an infix Operator, got {:?}",
                                self.peek_token
                            ));        
                            return None;
                        }
                    }
                },
                Token::Ident(value) => Expression::Identifier(Identifier { name: value.to_string() }),

                _ => {
                    self.errors.push(format!(
                        "expected next token to be an Expression, got {:?}",
                        self.cur_token
                    ));

                    return None;
                }
            }
        );
        Some(stmt)
    }
}

pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // < or >
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call         // myFunc(x)
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::{ast::*, lexer::Lexer};

    #[test]
    fn test_let_statements() {
        let input = String::from("
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ");

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        assert!(program.is_some(), "ParseProgram() returned None");

        // Check for parsing errors
        assert!(p.errors.is_empty(), "Parser has errors: {:?}", p.errors);

        let program = program.unwrap();
        assert_eq!(
            program.statements.len(),
            3,
            "program.Statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        let tests = vec![("x", "5"), ("y", "10"), ("foobar", "838383")];

        for (i, (ident_name, literal_value)) in tests.iter().enumerate() {
            match &program.statements[i] {
                Statement::Let(let_stmt) => {
                    assert_eq!(let_stmt.ident.name, *ident_name);

                    // Check the value of the let statement
                    match &let_stmt.value {
                        Expression::Literal(literal) => {
                            assert_eq!(literal.value, *literal_value);
                        }
                        _ => panic!("Expected Literal"),
                    }
                }
                _ => panic!("Expected LetStatement"),
            }
        }
    }

    #[test]
    fn test_parser_errors() {
        let input = String::from("let x 5;");

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let _program = p.parse_program();

        let program = p.parse_program();
        assert!(program.is_some(), "ParseProgram() returned None");

        // Check for parsing errors
        assert!(!p.errors.is_empty(), "Parser has no errors");

        assert_eq!(
            p.errors.len(),
            1,
            "parser has wrong number of errors. got={}",
            p.errors.len()
        );
    }

    #[test]
    fn test_return_statements() {
        let input = String::from("
            return 5;
            return 10;
            return 9933322;
        ");

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program().unwrap();

        let stmt_len = program.statements.len();
        assert_eq!(
            stmt_len, 3,
            "program.statements does not contain 3 statements. got {}",
            stmt_len
        );

        let tests = vec![("5"), ("10"), ("9933322")];

        for (i, literal_value) in tests.iter().enumerate() {
            match &program.statements[i] {
                Statement::Return(return_stmt) => {
                    // Check the value of the let statement
                    match &return_stmt.value {
                        Expression::Literal(literal) => {
                            assert_eq!(literal.value, *literal_value);
                        }
                        _ => panic!("Expected Literal"),
                    }
                }
                _ => panic!("Expected ReturnStatement"),
            }
        }
    }

    #[test]
    fn test_identifier_expressions() {
        let input = String::from("
            foobar;
            barbaz;
            quux;
        ");

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program().unwrap();

        let stmt_len = program.statements.len();
        assert_eq!(
            stmt_len, 3,
            "program.statements does not contain 3 statements. got {}",
            stmt_len
        );

        let tests = vec![("foobar"), ("barbaz"), ("quux")];

        for (i, literal_value) in tests.iter().enumerate() {
            match &program.statements[i] {
                Statement::Expression(expression_stmt) => {
                    match &expression_stmt {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, *literal_value);
                        }
                        _ => panic!("Expected IdentifierExpression"),
                    }
                }
                _ => panic!("Expected ExpressionStatement"),
            }
        }
    }
}
