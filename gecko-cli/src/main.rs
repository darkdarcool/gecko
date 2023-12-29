use gecko_lexer::Lexer;
use gecko_parser::Parser;
use gecko_inspector::inspect;

use std::{fs::File, io::Read};

fn read_file(path: &String) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn main() {
    let content = read_file(&String::from("test.gk"));
    let input = String::from(content);
    let stmts;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.scan_tokens().unwrap();

    let mut parser = Parser::new(tokens);
    stmts = parser.parse().unwrap();


    let output = inspect(stmts);

    println!("{}", output);

}
