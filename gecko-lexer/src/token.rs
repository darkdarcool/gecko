use gecko_error::LineInfo;

use crate::ttype::TType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TType,
    pub lexeme: String,
    pub lineinfo: LineInfo,
}

impl Token {
    pub fn new(ttype: TType, lexeme: String, lineinfo: LineInfo) -> Token {
        Token {
            ttype,
            lexeme,
            lineinfo,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {}", self.ttype, self.lexeme)
    }
}
