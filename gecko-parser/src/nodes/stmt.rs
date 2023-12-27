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
pub struct Fn {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub return_type: Option<Token>,
}

impl Fn {
    pub fn new(name: String, params: Vec<Param>, body: Vec<Stmt>, return_type: Option<Token>) -> Fn {
        Fn {
            name,
            params,
            body,
            return_type,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl(Var),
    /// name, params, body
    FnDecl(Fn),
    Return(Option<Expr>),
    LangImport(Vec<String>),
    FileImport(String),
}
