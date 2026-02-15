mod lexer;
mod parser;

fn main() {
    let source = "fn bla;";
    let tokens = lexer::Lexer::new(source).go();
    let tree = parser::Parser::new(&tokens).go();
    println!("{}", tree.show(source));
}
