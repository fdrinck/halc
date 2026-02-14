mod lexer;

fn main() {
    let source = " {  } blabla123";
    let tokens = lexer::Lexer::new(&source).go();
    println!("{:?}", tokens);
}
