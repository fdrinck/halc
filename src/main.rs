mod ast;
mod lexer;
mod parser;

use ast::Show;

fn main() {
    let source = "fn bla(a:b, c:d,) -> e { let a = b; }";
    let tokens = lexer::Lexer::new(source).go();
    let tree = parser::Parser::new(&tokens).go().unwrap();
    let mut output = String::new();
    tree.show(source, 0, &mut output);
    println!("{}", output);
}
