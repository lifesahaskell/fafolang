#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Debug, Default, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub ident: Identifier,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: String,
}
