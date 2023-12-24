use gecko_lexer::Lexer;
use gecko_parser::Parser;
use gecko_inspector::inspect;

fn main() {
    let input = String::from("let age = 1 + 1;");
    let mut lexer = Lexer::new(input);
    let tokens = lexer.scan_tokens().unwrap();

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse().unwrap();

    let output = inspect(stmts);

    println!("{}", output);
}
