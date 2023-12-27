use std::rc::Rc;

use gecko_lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Iden(String),
    Void,
    Unknown,
}


#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpr {
    pub value: Type,
}

impl LiteralExpr {
    pub fn new(value: Type) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupingExpr {
    pub expression: Rc<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Rc<Expr>) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Rc<Expr>) -> Self {
        Self { operator, right }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
}
