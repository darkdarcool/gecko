use crate::ttype::TType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TType, lexeme: String, line: usize) -> Token {
        Token {
            ttype,
            lexeme,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {}", self.ttype, self.lexeme)
    }
}
