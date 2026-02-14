mod lexer;

fn main() {
    let source = " {  } blabla123";
    let tokens = lexer::Lexer::new(source.as_bytes()).go();
    println!("{:?}", tokens);
}
