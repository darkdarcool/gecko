use gecko_lexer::token::Token;

use super::{expr::Expr, Param};

#[derive(Clone, Debug)]
pub struct Var {
    pub name: String,
    pub initializer: Option<Expr>
}

impl Var {
    pub fn new(name: String, initializer: Option<Expr>) -> Var {
        Var {
            name,
            initializer
        }
    }
}

#[derive(Clone, Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl(Var),
    /// name, params, body
    FnDecl(String, Vec<Param>, Vec<Stmt>, Option<Token>),
    Return(Option<Expr>),
}
