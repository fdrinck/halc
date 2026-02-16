mod ast;
mod lexer;
mod parser;

fn main() {
    let source = "fn bla(a:b, c:d,) -> e { let a = b; }";
    let tokens = lexer::Lexer::new(source).go();
    let tree = parser::Parser::new(&tokens).go().unwrap();
    println!("{}", tree.show(source));
}
