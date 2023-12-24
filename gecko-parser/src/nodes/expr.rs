use std::rc::Rc;

use gecko_lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Void,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Type),
    Grouping(Rc<Expr>),
    Variable(Token),
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Unary(Token, Rc<Expr>),
}
