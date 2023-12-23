#[derive(Debug, Clone, PartialEq)]
pub enum TType {
    Identifier(String),
    Number(f64),

    EQ, // =
    PLUS, // +
    MINUS, // -
    STAR, // *
    SLASH, // /
    BANG, // !
    EQEQ, // ==
    BANGEQ, // !=
    LT, // <
    LTEQ, // <=
    GT, // >
    GTEQ, // >=
    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }
    COMMA, // ,
    DOT, // .
    SEMICOLON, // ;
    COLON, // :
    ARROW, // ->
    EOF, // end of file
    TRUE, // true
    FALSE, // false
    IF, // if
    ELSE, // else
    WHILE, // while
    FOR, // for
    IN, // in
    FN, // fn
    LET, // let
    RETURN, // return
}
