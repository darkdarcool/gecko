use gecko_lexer::token::Token;

pub mod expr;
pub mod stmt;

#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub type_: Token,
}

impl Param {
    pub fn new(name: String, type_: Token) -> Param {
        Param { name, type_ }
    }
}
