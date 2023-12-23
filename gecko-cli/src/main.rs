use gecko_lexer::Lexer;
fn main() {
    let mut lexer = Lexer::new(String::from("let age = 3;"));
    let tokens = lexer.scan_tokens().unwrap();

    for token in tokens {
        println!("{}", token.to_string());
    }
}
