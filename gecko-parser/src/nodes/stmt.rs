use super::expr::Expr;

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
    VarDecl(Var)
}
