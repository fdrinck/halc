mod ast;
mod diagnostics;
mod lexer;
mod parser;

use ast::Show;

fn main() {
    let source = "fn bla(a:b, c:d,)\n-> e\n{\nlet a = b;\n}";
    let tokens = lexer::Lexer::new(source).go();
    let tree = parser::Parser::new(&tokens).go().unwrap();
    let mut output = String::new();
    tree.show(source, 0, &mut output);
    println!("{}", output);

    println!(
        "{}",
        diagnostics::show_source_context(source, ast::Span::new(3, 6))
    );
    println!(
        "{}",
        diagnostics::show_source_context(source, ast::Span::new(21, 22))
    );
}
